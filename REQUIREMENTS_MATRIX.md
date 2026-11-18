# Requirements and compatibility ledger

Snapshot v0.1.0, Phase 8. The executable columns refer to the Linux reference interpreter. Design credit requires an identifiable artifact. A proposal does not count as implementation. Historical reports are dated evidence. Current evidence is under `verification/phase8/`.

## Ultra Master Prompt v1.0

| § | Requirement and inspection | Disposition or gap |
|---|---|---|
| 0–1 | Rust compiler team mandate, .mn, mn, repository layout, v0.1.0, licenses, mn.toml, mn.lock, mn-lsp. Cargo manifests, README, init and version checked. | Working CLI, source archive, scaffold and dual-license texts. Mongolia package lock and language server planned. No public name or ownership clearance. |
| 2–3 | AI-native direction, correctness, safety, reproducibility, extensibility and benchmarking. See architecture, ADRs, audited builds and baseline. | Foundation executable. Acceleration and differentiability remain design targets. Stable toolchain pinned. Bit-identical builds across hosts unverified. |
| 4 | All 30 bilingual tokens, mixed sources, XID, NFC, original byte spans, spoof checks, localizations, formatting and LSP encodings. | Token equivalence and executable subset tested. Selected Latin/Cyrillic policy only. Partial translation. Native review, configurable formatter and protocol implementation pending. |
| 5 | Compilation units, lexical grammar, comments/escapes, statements, declarations, expressions, order of evaluation and operators. Compare grammar.ebnf, SPEC and parser. | Normative v0.1 subset covered. Future grammar explicitly sketches modules, generics, control flow, patterns, refs, tensors, FFI and propagation. It has unresolved nonterminals and lacks complete future precedence and ambiguities. |
| 6 | Scalar/compound types, casts/promotions, numeric exceptional behavior, binding, application, branch, generic and differentiation judgments. | i64/f64/bool/string/unit and rank-2 f64 tensors checked. Exact types, local inference, checked i64, IEEE f64. Other widths, never, arrays/slices/tuples, records/enums, Option/Result, generics/refs and casts planned. Future judgments are design rules. |
| 7 | Static/symbolic/dynamic shapes, layout, aliasing, device, guard insertion, allocation, matmul and other shape algorithms. | Static positive rank-2 f64, row-major, owned reads, rectangularity, equal-shape arithmetic and matmul checked. Broadcast, batched matmul, reshape, transpose, concat, reduction, indexing/slicing, convolution, attention, einsum have design constraints only. No runtime guard IR. |
| 8 | Reverse/forward AD, VJP/JVP, multiarg, scalar/nonscalar, higher derivatives, control flow, stop/custom rules, tape/checkpoint, subgradient and numerical verification. | Phase 7 defines contracts and a Python reference for MLP derivatives. No language AD transform or gradients. Higher-order, effects, custom rules and combined control flow require research/conformance. |
| 9 | Moves, lifetimes, shared/exclusive borrows, views, host/device, pools/pinned storage, asynchronous events, OOM/loss/accounting. | Ownership design and completion lease contracts exist. Foundation uses safe Rust host allocations and value semantics. Device-lifetime and allocator fault testing pending. |
| 10 | Pipeline, resolution, HIR, typed IR, AD, SSA/MIR, CFG/effects, passes, verifier/invalidation, strict math. | One lexer/parser/AST/signature checker/interpreter. Phase 1 and 7 specify proposed passes including specialization, buffer reuse, tiling/fusion/vectorization. No IR implementation, optimization pass manager or fast-math interface. |
| 11 | Interpreter, VM choice, CPU LLVM/MLIR, NVVM/CUDA, SPIR-V/Vulkan, WGSL/WebGPU, optional BLAS/SIMD and future hardware. | Only interpreter executes. VM trade-offs, capability detection, rejection/fallback and numeric policies documented. No hardware conformance. ROCm, Metal, Intel, RISC-V and Wasm are feasibility work. |
| 12 | NN primitives, activations/loss/optimizers, inference/training/serialization, mixed precision/quantization, serving, attention, parallel/distributed ML, DLPack/ONNX/safetensors/tokenizers. | Module/API sketches, MLP examples and independent oracle. No production NN module, kernel, serving, format adapter or collectives. |
| 13 | C ABI, ownership, alignment/layout, callbacks/unwinding, linking, Rust wrappers and foreign library interop. | Future contract in docs/FUTURE_DESIGN.md. Stable type layouts, implementation, ABI fixtures and all library adapters pending. |
| 14 | Acyclic functional Cargo workspace and target component map. | CLI → core → 2 direct Unicode crates plus tinyvec transitive. Fuzz is a separate workspace. ARCHITECTURE.md maps future responsibilities. No empty backend crates. |
| 15 | run/check/init/help/version; future build/new/add/remove/update/test/bench/fmt/lint/doc/repl/explain, target, profiles, verbosity, configuration and machine output. | Required commands, localized source errors, nonzero unsupported operations and exit contracts tested. No target selection, native build or resolver. Full future option precedence, verbosity and structured CLI schema remain open. |
| 16 | Incremental LSP, encoding negotiation, diagnostics/hover, completion, references/rename, signatures, formatting, tokens, hints and actions, editor/Tree-sitter. | Phase 7 designs interfaces and encoding fixtures. No mn-lsp executable, VS Code extension, Tree-sitter grammar or measured editing latency. |
| 17 | SemVer, edition, registry/checksums/cache/offline/yanks, provenance, signatures, advisories, publishing, permissions and native deps. | Scaffold only. Resolution and registry policy are design documents. No published registry, finalized edition, package installer or mn.lock writer. |
| 18 | Core/math/tensor/nn/optim/io/fs/net/time/random/device/ffi/collections/serialization, ownership, errors, cost, behavior and compatibility. | Rust primitives only, including print. Future module inventory and APIs in FUTURE_DESIGN. Complete per-operation numerical/complexity and compatibility contracts require RFCs. |
| 19 | Twelve EN/MN showcases, deterministic MLP with training loop. | 9 positive and 2 negative executable fixtures; 12 separate future proposal files. Two requested showcases share the complete MLP/optimizer program in each spelling. Future examples have proposed milestone labels. |
| 20 | Safe Rust frontend, immutable variables, typed functions, checks-before-run, runtime, arithmetic, shapes, bilingual CLI and diagnostics. | Working build. No control-flow claim. Result-based error paths and bounds inspected. Partial localization and total-memory limits documented. |
| 21 | A–E and mandatory negatives including UTF-8/spans, duplicate/undefined, arity, delimiters, ragged/invalid annotations, precedence, overflow, zero divide and exit codes. | Passed in debug and release. See docs/PHASE_2_ACCEPTANCE.md for named tests. Both shape operands, inner dimensions, underline, exit 1 and empty stdout checked for D. |
| 22 | Unit/integration/negative/golden/generated properties, numerical differential, lexer/parser fuzz, IR verification and backend conformance. | Existing tests plus audit regressions. Independent exact matrix oracle across profiles. AST validation tested. IR/backend comparisons, runtime AD tests and device stress depend on future implementations. |
| 23 | Reproducible timing, startup/frontend/check, runtime, memory and fair references. | Measured Linux subset in BENCHMARK_BASELINE.md. CPU/RSS/size evidence only. Scalar-focused, bandwidth, VM/GPU/ML workloads and hardware replication pending. |
| 24 | Malformed input, Unicode, parser resource, overflow, allocation, sandbox/trust, unsafe/FFI, packages/registry and release integrity. | SECURITY_AUDIT.md records policies, checksums, pinned advisory scan, corrections and residual risks. Public reporting owner, package governance operation and signed releases pending. |
| 25 | llms.txt, AGENTS.md, Copilot/Cursor rules, quick reference, machine grammar, mapping, limitations. | Files present. llms.txt now lists all aliases. Future surface explicitly distinguished. No assistant auto-discovery assumption. |
| 26 | Bilingual docs, contribution policies, RFC/review/compatibility/deprecation, release/name checks. | Docs and governance drafts exist. Native-speaker ratification, assigned maintainers, private disclosure channel and public namespace verification pending. |
| 27 | v0.1–1.0 milestones with dependencies, exit tests, estimates and risks. | ROADMAP.md contains per-version gates and person-month estimates. UPDATED_ROADMAP.md gives audit priorities and owners by role. Self-hosting remains separate. |
| 28 | Twenty-five source/design/evidence deliverables. | Artifact crosswalk below. Several long-term specifications and external release checks are partial. Source package is self-contained for v0.1 aside from pinned compiler, linker and Cargo dependencies. |
| 29 | Ordered phases, status, decisions, file provenance, evidence, limitations and package. | Historical design, status and raw evidence remain in archive. This audit uses new logs and its own fix record. |
| 30 | Compilable foundation, bilingual execution, checked shapes, CLI, tested examples, honest roadmap. | Linux foundation gate passed. Public release and future backend support remain conditional. |

### Original deliverables 1–25

| Deliverables | Where to inspect | Scope |
|---|---|---|
| 1 blueprint, 2 ADRs | ARCHITECTURE.md, docs/PHASE_1_ARCHITECTURE.md, rfcs/ | Foundation decisions and migration path |
| 3 spec, 4 EBNF, 5 type/shape rules | SPEC.md, grammar.ebnf, docs/future-grammar.ebnf, docs/FUTURE_DESIGN.md | Normative current subset, incomplete long-term grammar |
| 6 pipeline/IR, 7 full target workspace | ARCHITECTURE.md, docs/PHASE_7_FUTURE_ARCHITECTURE.md | Checked AST live, future crate/IR contracts proposed |
| 8 runnable compiler, 9 bilingual examples, 10 tests, 11 CI | crates/, examples/, fuzz/, .github/workflows/ci.yml | Linux verified; hosted three-OS jobs configured |
| 12 benchmarks | BENCHMARK_BASELINE.md, benchmarks/, scripts/phase8_benchmark.py | Single host measured |
| 13 GPU, 14 AD, 15 ownership, 16 FFI | docs/FUTURE_DESIGN.md and Phase 7 design | Design and independent numerical oracle; ABI and hardware validation pending |
| 17 standard library, 18 packages, 19 LSP | docs/FUTURE_DESIGN.md and Phase 7 design | Interface and implementation proposals |
| 20 assistant files, 21 governance | llms.txt, AGENTS.md, .cursor/, .github/, GOVERNANCE.md, SECURITY.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md | Files present, public operational roles pending |
| 22 roadmap, 23 risk register, 24 limitations, 25 verification | ROADMAP.md, UPDATED_ROADMAP.md, KNOWN_LIMITATIONS.md, BUILD_REPORT.md, verification/phase8/ | Current and historical records |

## Submitted expansion Part 6

| Supplied requirement | Classification | Evidence and boundary |
|---|---|---|
| Canonical source → lexer → AST → HIR → typed tensor IR → MIR → VM/MLIR pipeline | Partially implemented | Common frontend/checker and interpreter run today. Phase 7 specifies remaining boundaries. |
| Identical semantics across interpreter and compiled backends | Planned | Profile comparisons pass for the same interpreter. Compiled language backend required for this gate. |
| Define static, symbolic and dynamic tensor shapes | Implemented and verified (design artifact); runtime partially implemented | SPEC and shape design define the three categories. Only static shapes execute. |
| XID/NFC/confusables/UTF-8 spans | Partially implemented | Real XID/NFC and span tests, limited spoof policy. |
| Define device placement, ownership, asynchronous work and recoverable allocation | Partially implemented | Documented device contracts; host tensor allocation failures use Result. GPU validation pending. |
| Stable Rust and reproducible dependencies | Implemented and verified | Pinned toolchain, both lockfiles, package checksums, locked builds. |
| Runnable minimal v0.1, separate implemented/planned and tested examples | Implemented and verified | Core/CLI, SPEC, STATUS and 11 exact CLI expectations. |
| No required-path placeholders or pseudocode | Implemented and verified | Source inspection and AST/CLI execution; proposal files use .mn.proposal. |
| Lexer/parser/type/runtime unit, bilingual integration and negative tests | Implemented and verified | command-03/06 plus stress, CLI, golden and checked_program suites. |
| Known matmul results and source diagnostics | Implemented and verified | A–E, caret and shape goldens, exact dyadic matrix oracle. |
| Actual build/test/fmt/Clippy | Implemented and verified | Five requested commands with literal exit codes in commands.json. |
| README, LICENSE, CONTRIBUTING, CHANGELOG, SPEC, ROADMAP, llms, AGENTS | Implemented and verified | Root paths present, LICENSE points to both complete licenses. |
| CI configuration, executable examples and benchmark infrastructure | Implemented and verified locally | CI workflow inspected; remote Windows/macOS execution unverified. Measured frontend, CLI and CPU fixture supplied. |
| Separate future architecture, complete source, known limits | Implemented and verified | SPEC/Phase 7 design split, clean locked build, installation smoke and KNOWN_LIMITATIONS. |
| Evidence before optimization/performance claims | Implemented and verified for this audit | Raw timings, scope, oracles and spread reported. |

## Official expansion plan v3.0

The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) uses the supplied 01–50 headings in order. The [engineering contracts](docs/EXPANSION_V3_ENGINEERING.md) and [620-item register](verification/phase8/v3/features.tsv) attach delivery classifications, acceptance fixtures and execution-rule fields. The supplied Part 6 engineering-quality checklist above remains a separate source.
