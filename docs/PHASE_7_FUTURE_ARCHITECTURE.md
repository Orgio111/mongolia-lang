# Mongolia · Phase 7 engineering design

Revision 1, 2026-11-18. **Status: proposed designs for v0.2 through v1.0.** The current implementation is the v0.1.0 validated AST interpreter. Its executable specification lives in [SPEC.md](../SPEC.md). This document supplies the implementation sequence, interfaces and acceptance cases for the future features described in [FUTURE_DESIGN.md](FUTURE_DESIGN.md). [ROADMAP.md](../ROADMAP.md) records effort ranges, risks and release criteria for each milestone.

## First change after the foundation

Start with a private resolved representation for the existing single-file language. A small change that can be reviewed independently is to lower a forward call to a function ID and an immutable variable read to a local slot. Keep the expression span on both uses. The existing program with `add(12, 30)` must still print 42, duplicate bindings must report the same error, and the English/Mongolian variants must have identical operations after source locations and local spellings are omitted. Complete the whole v0.1 corpus before extending the grammar. This lets the team locate a migration error independently of new language semantics.

| Milestone | Engineering dependency | First deliverable | Evidence to promote it |
|---|---|---|---|
| v0.2 HIR and VM | Checked functions, exact types and original spans | Resolved IDs, verified register instructions, scalar calls followed by static rank-2 tensors | Interpreter/VM stdout, failures, blame spans and resource-limit cases match |
| v0.3 AD | Pure typed tensor operations, shape constraints, SSA verifier | Forward and reverse transforms for add, multiply, matmul and reduction | Analytical checks, finite differences, adjoint identity, then the bilingual MLP fixture |
| v0.4 native CPU | Verified lowering, strict numerical semantics, buffer ownership | CPU matmul on contiguous f64 buffers | Differential corpus on x86_64 and AArch64, then controlled BLAS/SIMD benchmarks |
| v0.5 CUDA | CPU reference, placement checks, event-bound allocation lifetime | Explicit upload, compatible matmul, download | Numerical parity, failure injection and reuse tests on a named GPU |
| v0.6 portable GPU | Capability table and target-specific layout/legalization | Vulkan compute and WebGPU compute each declare a supported subset | Tests on real devices for both supported and rejected programs |
| v0.7 tools | Versioned source snapshots and resolved IDs | Diagnostics, hover, shape display and formatter | Unicode position round trips, stale edit cancellation, parse/format equivalence |
| v0.8 packages | Module identity, governed editions, archive and trust policy | Verified local cache, resolver and lockfile | Offline replay, tamper/yank/conflict tests and publication incident exercise |
| v0.9 AI infrastructure | Tested AD, runtime, packages and explicit effects | Load a versioned model and reproduce a reference forward pass | Format round trip, workload conformance and distributed failure cases |
| v1.0 stability | Supported-target matrix, ABI scope and migration process | Ratified spec, library policy and release artifacts | Compatibility corpus, external review, signed provenance and release criteria |

The VM can begin with static scalar/tensor types attached to HIR. Rich tensor SSA and its differentiation rules become necessary at v0.3. Native code generation and AD may share IR infrastructure while keeping independent release tests. Editor work may begin once source identities are available. Registry work depends on stable package and module identities.

## Compiler and VM contracts

A module identity includes package, edition and normalized path. Reject import cycles initially with the dependency chain in the diagnostic. Collect declarations before checking function bodies so forward references still work. Reject name collisions after NFC and keep a map back to the original source spelling. Use revision-scoped IDs internally. Cache keys include source, transitive interface hashes, edition, compiler revision and numerical mode.

Each SSA block declares typed arguments and ends in one terminator. Each instruction has an opcode, operands, result types, effect information and an origin span. A tensor type contains element type, rank, dimension terms and device. Buffer identity, offsets, strides and aliasing enter during bufferization. The verifier checks definitions, dominance, branch argument types, effect order and guard dominance. Compiler transformations must propagate the source span of a failing operation.

For an operand pair `[M,K1] @ [K2,N]`, reject unequal constants at check time. Propagate equalities between declared symbols only when justified. Independently unknown runtime extents require an equality guard. Guard the product M*N, its byte size and target capacity before allocation. Bind the checked values to SSA operands so optimization cannot move a dependent allocation ahead of its guard. For broadcasting, align from the right and accept equal dimensions or an extent of one. Keep unresolved alternatives as disjunctions until specialization or a runtime guard selects a case.

Lower checked SSA values to typed registers. A call frame owns its register file and return destination. Verify the arity, register bounds, initialization of every read and terminators before dispatch. The first differential suite uses the existing v0.1 output, diagnostic and budget contracts. Charge resource units at their original semantic operations even after inlining. For example, optimizing away an unused integer division by zero would remove an observable error. Only eliminate it when the compiler has proved it cannot fail. Buffer printed output through successful completion, as required by v0.1.

Proposed pass sequence: resolve → type/shape/effect analysis → verify → AD on requested regions → verify → pure simplification and specialization → fusion → bufferization and liveness → target lowering. Changes to CFG edges invalidate dominance and liveness. Replacing tensor dimensions invalidates shape proofs and cached specializations. Changing stores or calls invalidates alias/effect summaries. Preserve signed zero, checked integers, observable error order and strict contraction order under default arithmetic. Reassociation or a relaxed vendor kernel needs an explicit mode and a documented accuracy contract.

## Differentiation

For a differentiable input tree x, reverse mode takes a result cotangent and produces an input cotangent with matching field shapes. Forward mode takes an input tangent and returns an output tangent. The proposed `grad(f)(x)` uses a scalar real output with seed 1. A vector output requires an explicit VJP seed or JVP direction. Select active arguments explicitly. Record saved primal values and their lifetimes before lowering tensors to mutable buffers. Accumulate contributions at every shared use, in deterministic order under strict mode.

| Typed operation | Forward rule | Reverse contribution for incoming g |
|---|---|---|
| z = x+y | dx+dy | Add g to both inputs, summing expanded broadcast axes back to their original shapes |
| z = x*y | dx*y+x*dy | Add g*y and g*x to the respective inputs, reduce broadcast axes |
| Z = A@B | dA@B + A@dB | dA += g@transpose(B), dB += transpose(A)@g |
| z = mean(X), positive element count n | mean(dX) | Expand g/n to X's shape |
| z = tanh(x) | (1−z²)*dx | Add g*(1−z²) |
| stop_gradient(x) | Zero tangent | End the reverse path |

As a hand-check, let A=[[1,2]], B=[[3],[4]], and let g=[[2]]. The primal is [[11]], dA=[[6,8]] and dB=[[2],[4]]. With tangent seeds dA=[[1,−1]], dB=[[2],[1]], the forward directional derivative is [[3]]. Its inner product with g is 6, equal to the sum of the input tangent–cotangent inner products. Add a shared-use case `x*x+x` at x=3 with unit seed, expecting derivative 7. Negative fixtures cover integer-only input, an invalid cotangent shape, printing inside a differentiated function and an external function lacking a derivative rule.

For branches, record which branch ran. For loops, save the trip count and the per-iteration values required by reverse rules. Replay checkpointed regions only when effects, precision and random state permit the same values. Define ReLU's proposed subgradient at zero as 0 and test it separately from smooth finite differences. Higher derivatives require differentiable derivative rules. Custom rules must declare active inputs, returned tree, effects and saved values. Reject unsupported operations at the original span.

The independent Python fixture uses the exact parameter order and deterministic examples from the proposed two-layer MLP. It computes analytic gradients for all 13 parameters and checks central differences at three step sizes. On this host, the largest discrepancy across all three steps was below 1.5e-9. The reference loss fell from 1.3173184637359858 to 0.0029560297431827035 after 200 SGD updates of 0.05. For a first strict f64 CPU implementation, compare the initial gradient, four predictions and final loss with proposed absolute tolerance 1e-8. Compare every parameter gradient before judging training progress. Both source spellings should yield the same typed program and results. Revisit tolerances for each lower precision and device. These figures are from Python. Mongolia AD and training execution are planned. [Reference program](phase7/mlp_reference.py) · [Recorded values](../verification/phase7/mlp-reference.json) · [English showcase](../examples/future/mlp_training.mn.proposal) · [Mongolian showcase](../examples/future/mlp_training_mn.mn.proposal)

## Native execution and device memory

Use a backend request containing typed IR, target triple/device, strictness mode and explicit fallback policy. Return either a verified artifact with its requirements and source map, or a structured diagnostic for the unsupported operation, dtype, layout or device. Probe features on the executing device before loading cached code. A cache key includes IR digest, target features, numerical mode, optimization profile, backend/toolchain build identity and driver compatibility.

| Path | Proposed first lowering | Acceptance test and reason for rejection |
|---|---|---|
| CPU | Lower contracted tensor operations to MLIR Linalg, bufferize to MemRef, lower loops/vectors to LLVM, choose native SIMD by target features | Compare rectangular/non-square results against the reference. Exercise unaligned and noncontiguous inputs once views exist. BLAS dispatch requires a compatible layout, dtype, accumulation order and ABI. |
| NVIDIA | Outline a GPU kernel, lower supported scalar/vector operations to NVVM, load target-compatible PTX using a narrow driver adapter | Check launch, transfer and event errors. Validate mixed-precision accumulation separately. cuBLAS/cuBLASLt dispatch requires compatible math mode. cuDNN and Tensor Cores are later capabilities. |
| Vulkan | Legalize device IR to the supported SPIR-V compute subset, bind checked descriptor ranges, size workgroups to device limits | Compare a legal kernel and reject an unsupported dtype/workgroup before submission. Include explicit memory barriers for dependencies. |
| WebGPU | Lower a separately verified subset to WGSL and construct bind-group layouts from the storage contract | Probe features and buffer limits. Reject an f64 workload where unavailable. A user-enabled host fallback must identify copies and synchronization. |

The first native CPU implementation should keep a simple strict loop available. Optimized libraries can change arithmetic order or contraction behavior. Publish flags, accumulator type, hardware, data layout, transfer time, synchronization points and numerical error alongside any performance measurement. The CPU interpreter remains a test oracle.

An owned tensor has a unique allocation identity. A shared view carries region, strides, device and lifetime. Mutable access requires exclusive access to the addressed region. Treat unknown overlap conservatively and zero-stride broadcast views as read-only. A host-to-device transfer produces a separately owned destination and a completion event. Charge requested and reserved bytes separately for pools, pinned host staging and active device buffers.

| Lifecycle event | Runtime action | Verification case |
|---|---|---|
| Successful submission | Record resource leases and dependencies before user handles may drop | Drop A immediately after enqueuing A@B, then read the completed result |
| Cross-stream read-after-write | Consumer waits for producer event on the owning queue/device | Delay the first stream and verify that the second does not observe old bytes |
| Failed submission | Release resources for unsubmitted work; preserve leases for any submitted prefix | Inject failure between upload and kernel submission |
| Completion or cancellation | Dropping a future abandons interest in the result. Recycle only after all device references retire | Drop result, issue same-size allocation, assert distinct live storage until both events complete |
| Device loss | Poison the context generation, end waits with a typed error and quarantine allocations until teardown | Report loss to all dependent futures and reject caches/handles from the previous context |
| Allocation failure | Return a recoverable error with device, requested bytes and pool state | Force a small budget, free completed work, retry and verify data isolation |

Start lifetime testing with a deterministic fake queue that controls completion order, then repeat on physical hardware. Asynchronous FFI adapters must retain both buffer and callback leases, translate foreign status codes and contain unwinding at the ABI boundary. C-compatible slices use explicit length and ownership. A later stable C ABI requires target-specific layout/size/alignment fixtures.

## Package resolution and tools

A package identity comprises registry origin, namespace and package name. An index entry describes one immutable version, dependencies, checksum, edition and yanked state. Resolve constraints for the full graph, including features and target predicates. Use deterministic candidate/tie ordering and backtrack on conflict. Prefer a compatible existing lock entry for ordinary builds. An explicit update may change eligible versions. Report incompatible constraints with the introducing dependency paths.

Record exact sources, versions, checksums, resolved features, target predicates and dependency edges in a versioned mn.lock. A locked build compares manifest requirements to the lock and errors on mismatch. Index and archive downloads go into temporary files. Validate size and digest, reject path escapes and write the verified content to a digest-keyed cache by atomic rename. Offline mode uses this cache exclusively. Keep a verified yanked version usable for a locked build subject to the project's explicit advisory policy. A fresh resolution excludes it.

An acceptance registry can contain A→B(^1) and C→B(≥1.2,<2) with B versions 1.1 and 1.2. The first resolution selects 1.2. Shuffling index order must preserve lock bytes. Yanking 1.2 must preserve the existing verified lock and make fresh resolution fail with both constraint paths. A changed archive byte must fail integrity checking before extraction or native linking. Publishing needs namespace authorization, immutable version storage, provenance verification, owner recovery, revocation procedures and an advisory channel. Arbitrary installation scripts are disabled by default. The local-module milestone precedes registry distribution.

For the language server, index each document by URI and revision and keep source bytes plus per-line encoding offsets. A negotiated UTF-8/UTF-16/UTF-32 position converts to a valid scalar boundary. Return ranges from the source revision producing the answer. Example line prefix `a😀Ө`: after the emoji the byte offset is 5, UTF-16 offset 3, scalar offset 2 (all zero-based). Test round trips, CRLF handling, supplementary characters, decomposed accents, and rejected mid-scalar edits. Cancel stale jobs and discard responses from superseded revisions.

Token/parse queries depend on source revisions, name queries on module exports, and type/shape queries on resolved symbols and their signatures. Editing a private function body invalidates its dependent body queries. Changing an exported type invalidates dependent modules. Hover shows the resolved type and shape, rename reruns collision checks across affected scopes, and go-to-definition returns the defining source span. The formatter needs a trivia-preserving syntax tree to preserve comments and literal spelling. Test idempotence for en/mn/preserve keyword modes, then reparse and compare canonical semantics. A VS Code client registers .mn documents and negotiates server capabilities. Tree-sitter highlighting may share syntax fixtures while the compiler supplies authoritative diagnostics.

## Delivery decisions and open questions

| Choice for first implementation | Reason | Revisit when |
|---|---|---|
| Use existing bilingual conformance cases as the VM oracle | They already distinguish parse, static and runtime failures | A new edition changes documented behavior |
| Deliver CPU scalar gradients before MLP training | One VJP/JVP pair isolates transform bugs from optimizers and ownership | Elementwise and matmul adjoints pass the shared-use and dot-product fixtures |
| Require explicit device transfer and fallback | Cost, synchronization and placement are observable | A device RFC defines portable inference semantics |
| Limit package resolution to one compatible version per identity in the first resolver | Conflict diagnostics and offline replay stay tractable | A reviewed isolation model permits multiple versions |
| Keep prototypes private to working modules | The existing Cargo dependency graph remains buildable | A distinct component has working APIs, a verifier and independent tests |

Research items include higher-order AD under mutation, alias proofs for arbitrary strided slices, distributed collective failure semantics and profitable cross-device fusion. Record algorithmic, memory and numerical evidence before selecting an approach. FFI safety review, hardware availability, Unicode security, native terminology review and registry governance each have separate owners and release gates in ROADMAP.md.

## Phase report

This phase delivers future subsystem contracts, proposals for implementation order, numerical fixtures and acceptance gates. The accompanying roadmap, architecture and project status link the decisions to existing milestones. The Python oracle was executed, and the Phase 6 compiler checks were repeated as recorded in BUILD_REPORT.md. Future backend, lifetime, resolver and editor tests above are acceptance specifications. Implementing the first v0.2 HIR slice is the next engineering milestone. Completion of the seven-phase foundation protocol leaves public release and v1.0 stabilization subject to their recorded gates.

## Technical references

Mongolia's pass choices, ABI boundaries and tests are project proposals. Pin and verify concrete upstream tool versions during implementation.

- MLIR tensor-to-buffer analysis and in-place storage rules. [MLIR Bufferization](https://mlir.llvm.org/docs/Bufferization/)
- Kernel outlining and target-specific GPU compilation. [MLIR GPU dialect](https://mlir.llvm.org/docs/Dialects/GPU/)
- Numerical checks and the relationship between JVPs and VJPs. [JAX Autodiff Cookbook](https://docs.jax.dev/en/latest/notebooks/autodiff_cookbook.html)
- Position encoding and client/server negotiation. [LSP 3.17 specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
