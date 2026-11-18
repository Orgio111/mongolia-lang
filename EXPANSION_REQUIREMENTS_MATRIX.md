# Expansion requirements and traceability

The integration covers v1.0 prompt sections 0–30 and its 25 deliverables, the separately supplied engineering-quality checklist, v3.0 sections I–XII, and the complete Phase 8 source/evidence package. No input is outstanding. SPEC.md governs execution. The expansion plan's execution rules govern how a proposal advances to implementation. Its universal performance objective is subject to capability-specific evidence.

## Requirement ownership

| Source | Authoritative coverage | Integration use |
|---|---|---|
| Original identity, specification, implementation and governance mandate | [Original section and deliverable ledger](REQUIREMENTS_MATRIX.md) | Retain v0.1 behavior and original v0.2–v1.0 engineering gates. |
| Phases 1–7 | [Architecture](ARCHITECTURE.md), [specification](SPEC.md), [historical build record](BUILD_REPORT.md), [Phase 7 design](docs/PHASE_7_FUTURE_ARCHITECTURE.md) | Inherit normative subset, ADRs, module boundaries and proposed IR migration. |
| Phase 8 | [Audit](AUDIT_REPORT.md), [security](SECURITY_AUDIT.md), [commands](verification/phase8/commands.json), [benchmark](BENCHMARK_BASELINE.md), [known limits](KNOWN_LIMITATIONS.md) | Preserve verified code, published findings and limits as a versioned baseline. |
| v3.0 sections I–X | [50-area crosswalk](EXPANSION_V3_MATRIX.md), [620 original item contracts](verification/phase8/v3/features.tsv) | Keep original names, classes, audit status, evidence and acceptance fixtures. |
| v3.0 sections XI–XII | [Engineering contracts](docs/EXPANSION_V3_ENGINEERING.md), [priority matrix](IMPLEMENTATION_PRIORITY_MATRIX.md), [release gates](MASTER_ROADMAP.md) | Assign core/library/framework/integration/experimental/research scope. Qualify individual features and targets before making claims. |

Each row below joins to item rows by the two-digit area ID. Within an area the source item label identifies a requirement. The existing register carries its technical specification, dependencies, architecture decision, milestones, acceptance criterion, correctness and performance tests, security risks, maintenance and compatibility policy. Source/test/design paths appear in its baseline evidence column. Maintain these records when promoting a feature.

| ID | Official area | Items | Primary class | Audited status | Verified coverage and gap |
|---|---|---:|---|---|---|
| 01 | Universal General-Purpose Programming | 17 | Core requirement | Partially implemented | Typed functions and host arithmetic execute. Application platforms lack deployable libraries and target contracts. |
| 02 | Multi-Paradigm Programming | 12 | Core requirement | Partially implemented | Procedures with immutable values and direct calls run. Advanced paradigms lack language or library implementations. |
| 03 | Advanced Type System | 14 | Core requirement | Partially implemented | Finite structural tensor types and checked primitive types exist. Advanced polymorphism, effects and user ADTs remain open. |
| 04 | Metaprogramming | 10 | Experimental capability | Missing | Parser produces owned syntax. No user-visible macro, reflection or compile-time evaluator exists. |
| 05 | Advanced Error Handling | 11 | Core requirement | Partially implemented | Spanned compiler errors and recoverable checked runtime errors exist. Language Result/Option, cancellation and stack traces are future work. |
| 06 | Hybrid Compilation Architecture | 11 | Core requirement | Partially implemented | Tree-walking interpreter runs. VM/AOT have designs; JIT tiers and remote execution lack working engines. |
| 07 | Multi-Backend Compiler Infrastructure | 10 | Core requirement | Planned | LLVM/MLIR, NVVM, SPIR-V and WGSL have proposed paths. Cargo builds the Rust interpreter only. |
| 08 | Adaptive Optimization Engine | 15 | Experimental capability | Planned | Several static passes are designed. No Mongolia pass manager, adaptive recompilation or deoptimization ships. |
| 09 | Hardware-Aware Autotuning | 11 | Experimental capability | Research required | Backend cache keys are proposed. There is no measured tuning search or persistent tuner. |
| 10 | AI-Guided Compiler Optimization | 9 | Long-term research | Research required | No learned model selects passes or scheduling decisions. |
| 11 | Compiler Correctness Engineering | 12 | Core requirement | Partially implemented | Whole-program validation, seeded input generation, differential arithmetic and fuzz campaigns exist. IR proofs, translation validation and staged bootstrap are open. |
| 12 | Unified Memory Architecture | 15 | Core requirement | Planned | Host allocation is internal to the interpreter. No user allocator, regions, manual memory API or borrow checker exists. |
| 13 | Advanced Heterogeneous Memory | 11 | Core requirement | Planned | Completion leases, pinned buffers and async transfers have designs. No device allocator or transfer executes. |
| 14 | Low-Latency Runtime | 9 | Experimental capability | Missing | Startup and RSS are measured for a bounded interpreter. No real-time profile, task scheduler or standalone native runtime exists. |
| 15 | Advanced Concurrency | 14 | Core requirement | Missing | Single-thread interpreter has no language tasks, atomics, channels or scheduler. |
| 16 | Distributed Computing | 12 | Optional framework | Planned | Distributed tensors, collectives and failure semantics are design goals. No cluster runtime ships. |
| 17 | CPU Architecture Expansion | 15 | External integration | Partially implemented | The Rust-hosted CLI is verified on x86-64 Linux. Mongolia-native CPU lowering and ISA-specific optimization have no execution record. |
| 18 | GPU Architecture Expansion | 10 | External integration | Planned | CUDA, Vulkan and WebGPU have separate designs. No GPU kernel or driver call executes in v0.1. |
| 19 | AI Accelerators | 6 | External integration | Research required | Generic device architecture is proposed. No NPU/TPU/DSP adapter or named device conformance exists. |
| 20 | FPGA and Hardware Description | 7 | Long-term research | Research required | No HLS, RTL emitter, FPGA simulator or co-design toolchain exists. |
| 21 | Embedded and Bare-Metal Computing | 12 | External integration | Missing | Current binary needs Rust std, a host OS and heap. No freestanding target profile exists. |
| 22 | Operating System Development | 10 | Optional framework | Missing | No privileged target, syscall ABI, unsafe language blocks or kernel support exists. |
| 23 | Full-Stack Web Development | 14 | Optional framework | Missing | Wasm is a feasibility target. No browser bindings, web server or framework exists. |
| 24 | Mobile Development | 12 | External integration | Missing | No Android/iOS target integration, SDK bindings or signed app packaging exists. |
| 25 | Desktop Development | 11 | Optional framework | Missing | The CLI runs on Linux and CI describes other hosts. GUI/application integration is absent. |
| 26 | Game Development | 15 | Optional framework | Missing | No game engine, graphics renderer, asset pipeline or game loop ships. |
| 27 | Cloud and Enterprise Development | 14 | Optional framework | Missing | No deployable services, authentication middleware or cloud integration in the package. |
| 28 | Database and Data Engineering | 14 | Optional framework | Missing | C/SQLite interoperability is a design direction. No SQL adapter, storage engine or Arrow/Parquet implementation exists. |
| 29 | Scientific and Mathematical Computing | 15 | Standard library capability | Partially implemented | Dense f64 matmul and elementary arithmetic pass exact small-matrix checks. Advanced algorithms and precision families are open. |
| 30 | Financial Computing | 8 | Optional framework | Missing | i64/f64 exist. Decimal, fixed-point and financial domain packages are absent. |
| 31 | Blockchain and Cryptographic Computing | 9 | Optional framework | Partially implemented | Interpreter work/output budgets and deterministic reference operations exist. No consensus VM, smart-contract compiler or cryptographic package. |
| 32 | Robotics and Real-Time Systems | 11 | External integration | Missing | Single-host measurements give no deadline guarantee. ROS 2, controls and real-time runtime are absent. |
| 33 | Quantum Computing Interfaces | 6 | Long-term research | Research required | No circuit representation or quantum provider interface exists. |
| 34 | Native AI Super Engine | 14 | Core requirement | Partially implemented | Eager CPU tensor literals and checked rank-2 operations execute. Lazy graphs, AD, graph optimization and devices remain proposed. |
| 35 | Large Language Model Infrastructure | 18 | Optional framework | Planned | Transformer-related APIs, KV caches, batching and interoperability are design targets. No model training or serving stack runs. |
| 36 | Advanced Numerical Precision | 14 | Core requirement | Partially implemented | f64 arithmetic/tensors are verified. f32 is reserved; other low-precision formats, scaling and calibration have no executable support. |
| 37 | Native Dataflow and Graph Computing | 10 | Experimental capability | Planned | Proposed tensor SSA supplies a graph design. No general dataflow graph runtime or serializer exists. |
| 38 | AI Agent Infrastructure | 12 | Optional framework | Missing | No provider interface, agent framework or tool-permission layer exists. |
| 39 | Mongolia Universal Bridge | 27 | External integration | Planned | C-compatible ABI and ownership rules have designs. No foreign code executes through mn. |
| 40 | Legacy Code Migration | 8 | Long-term research | Research required | No migration tool or semantic translator ships. |
| 41 | Advanced Developer Toolchain | 16 | Optional framework | Planned | run/check/init and diagnostic rendering work. LSP/formatter tooling is designed. No listed debugger, interactive REPL or build cache ships. |
| 42 | Mongolia Studio | 10 | Optional framework | Planned | Existing-editor/LSP integration is designed. No Studio executable or VS Code extension ships. |
| 43 | Universal Build System | 13 | Core requirement | Partially implemented | The compiler project has pinned Cargo builds and a checksummed source package. Mongolia has no build graph, linker frontend or package resolver. |
| 44 | Capability-Based Security | 13 | Core requirement | Partially implemented | Source, tokens, depth, tensor elements, work and output have limits. No permission broker, process wall-clock cap or sandbox exists. |
| 45 | High-Assurance Computing | 11 | Long-term research | Partially implemented | Static typing, checked arithmetic and deterministic reference rules cover a subset. No proof, safety-critical profile or certification exists. |
| 46 | Observability and Fault Tolerance | 13 | Standard library capability | Partially implemented | The CLI reports localized runtime diagnostics and buffered output is discarded on failure. No service supervision or tracing pipeline exists. |
| 47 | Universal Performance Benchmarking | 22 | Core requirement | Partially implemented | Linux startup, lexer/parser, checked matmul, RSS and size have raw data, Python/BLAS references and numerical checks. Other domains lack comparable implementations. |
| 48 | Independent Mongolia Technology Ecosystem | 19 | Optional framework | Planned | Library domains have roadmap sketches. None of the nineteen named products is established by this repository. |
| 49 | Proprietary-Named Open-Source Core Technologies | 8 | Experimental capability | Planned | Eight proposed names map to design responsibilities. No named component implementation, ownership or naming clearance is asserted. |
| 50 | Full Compiler Self-Hosting | 10 | Long-term research | Research required | Current compiler is Rust. Stage-0/1/2 and a Mongolia-written frontend/middle-end do not exist. |

**Coverage check.** 50 unique areas and 620 unique (area, item) pairs. At area level 16 are partial, 13 planned, 14 missing, and 7 require research. At item level 12 are implemented and verified, 34 partial, 122 planned, 331 missing and 121 require research. No item currently uses implemented-but-unverified. These are counts of unequal units. They give no percentage estimate of project completion.

**Promotion rule.** The owning team attaches its RFC, version, target, linked test and measured outcome to the item. A change to a parent area needs a review of every child and the support declaration. An experiment remains scoped to its specific fixture until the public acceptance gate passes.
