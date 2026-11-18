# Mongolia engineering blueprint

Compiler 0.1.2 retains the Phase 9 frontend, static checker and AST interpreter. The release pipeline packages the host executable on each verified target. See [platform evidence](PLATFORM_SUPPORT_MATRIX.md) and [Phase 11 design gates](PHASE_11_EXECUTION_PLAN.md).

Status: v0.1 foundation implementation; future capabilities below are design proposals.
Mongolia is an English/Mongolian programming language with one semantic model. The first
release establishes a small, statically checked scalar and rank-2 tensor language. It does
not claim Rust-like borrowing, native compilation, automatic differentiation, GPU support,
or competitive performance. The roadmap is conditional on verification evidence.

## Boundaries and architecture

The executable dependency graph is `mn-cli -> mn-core -> unicode-ident, unicode-normalization, unicode-script`.
`mn-core::frontend` owns byte spans, errors, canonical AST types, lexer and parser.
`mn-core::engine` owns static validation and interpretation. `mn-cli` owns file IO,
argument processing, localization and process exit status. Core never terminates the process.
There are no cyclic crate dependencies and no empty backend crates. ASTs own strings and
children; source bytes remain owned by the caller for diagnostic rendering. Types are
structural; tensor shape is part of the type. Function names are resolved across the whole
program before execution. Local environments are function-scoped and immutable.

The public boundary is `parse(source) -> Result<Program, Error>`,
`check(program) -> Result<(), Error>`, and `run(program) -> Result<String, Error>`.
For repeated execution use `let checked = engine::validate(&program)?;` followed by
`checked.run()?`. The checked handle borrows immutable syntax and retains the signature
table built during validation. Each execution starts with fresh work and output budgets.
The CLI uses this interface for both check and run. The simpler check and run functions
remain available for clients that hold an AST.
Output collection makes tests deterministic but imposes a memory limit requirement for
untrusted inputs. This interpreter is not a security sandbox.

The [Phase 7 design](docs/PHASE_7_FUTURE_ARCHITECTURE.md) specifies the transition from CheckedProgram, operation verifiers and cross-backend acceptance fixtures.

## Migration to the target compiler

Source -> tokens -> AST -> resolution/HIR -> checked tensor IR -> AD -> SSA/MIR ->
optimizations -> backend. Initially, resolution and type checking share AST traversal.
Introduce HIR when modules require stable symbol IDs; introduce typed IR before AD.
Every new representation requires a verifier and a source-origin map. No backend may
reinterpret arithmetic, error order, or shape requirements independently.

| Target crate | Responsibility and dependencies |
|---|---|
| mn-span | Source IDs, byte spans, checked positions; no compiler dependencies |
| mn-diagnostics | Codes, structured labels, localized renderers; span |
| mn-lexer | Canonical bilingual tokens; span, diagnostics |
| mn-ast | Owned syntax tree; span |
| mn-parser | Syntax construction; lexer, ast, diagnostics |
| mn-hir | Resolved symbol IDs and desugared forms; span |
| mn-resolve | Module graph and name resolution; ast, hir, diagnostics |
| mn-shape | Shape expressions, constraints, guard plans; no runtime |
| mn-typeck | Type inference and effect checks; hir, shape, diagnostics |
| mn-ir | Typed operations, CFG, SSA and verification; span, shape |
| mn-autodiff | Verified derivative transformations; ir |
| mn-optimizer | Verified semantics-preserving transforms; ir |
| mn-codegen | Backend trait and capability descriptors; ir |
| mn-codegen-llvm/cuda/vulkan/webgpu | Target lowering; codegen, ir; isolated vendor dependencies |
| mn-runtime | Allocations, devices, streams, kernels; no frontend |
| mn-vm | Typed register VM; ir, runtime |
| mn-ffi | Explicit C ABI bridges; runtime |
| mn-std | Builtin signatures and implementations; runtime |
| mn-package | Manifests, resolver, integrity; no compiler internals |
| mn-fmt | Lossless syntax formatting; lexer/parser |
| mn-lsp | Incremental database and protocol adapters; frontend services |
| mn-cli | Composition root; core services and selected backend features |

Only create each crate when it owns working functionality. `std/`, `editors/`, `fuzz/`,
and backend integration directories grow with implemented milestones, not empty scaffolds.

## IR contract (proposed)

HIR: globally unique symbols, explicit binding scopes, original source spans.
Typed Tensor IR: every value has element type, rank, dimensions, device and effect;
operations carry constraints and resolved overloads. A dynamic guard dominates every
operation whose safety requires it. Memory identity is distinct from value identity.
SSA: one definition per value, dominance of uses, complete phi/block arguments,
explicit terminators, structurally valid CFG, effect tokens ordering observable actions.
Buffer IR: ownership and alias facts, checked sizes, layouts and asynchronous dependencies.

Recommended pass order: verify -> shape propagation -> effect analysis -> AD -> verify ->
constant folding -> dead-code elimination -> CSE -> bounded inlining -> shape specialization
-> fusion -> bufferization -> memory planning -> tiling/vectorization -> target lowering.
AD precedes destructive buffer reuse. CSE and DCE preserve effects and trapping operations.
Constant folding uses language overflow semantics. Reassociation, contraction and approximate
math require explicit fast-math policy. Each pass declares preserved analyses; otherwise
invalidate alias, dominance, liveness and shape caches it could affect. Verify after every
pass in debug/CI and at external backend boundaries in release builds.

## Backend decisions

A register VM is the proposed v0.2 execution engine: typed registers reduce dispatch and
make SSA lowering direct. A stack VM is easier to encode but adds stack movement and
obscures tensor lifetimes. Compare code size, dispatch cost, startup and debug quality before
freezing bytecode. Bytecode is private and versioned, never a stable package ABI.

Native CPU uses MLIR structured tensor/loop abstractions and LLVM lowering, with optional
BLAS adapters selected by shape/layout/capability. CUDA, Vulkan and WebGPU remain separate
capability-checked lowering paths. Unsupported targets fail explicitly; silently switching
devices could change latency, numerical behavior and allocation failure behavior.

## Engineering gates

Every implementation change updates the specification and negative tests. Promotion from
proposal to implemented requires runnable examples and recorded checks. Public release
requires Unicode security review, resource-limit tests, dependency audit, licenses, naming
checks and passing cross-platform CI. BUILD_REPORT.md is evidence; this design is not.
