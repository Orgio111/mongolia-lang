# Universal Hyper Performance Expansion Master Plan v3.0 · compatibility matrix

**Scope.** This crosswalk assesses the 50 official expansion areas against the audited Mongolia v0.1.0 snapshot. The 620 item rows count the listed domains, methods, targets, metrics, bridges and proposed names. They are acceptance/work items of very different sizes. Counts measure coverage of the submitted plan and cannot estimate engineering progress.

For the 50-area machine-readable matrix, use [expansion-intake.tsv](verification/phase8/expansion-intake.tsv). For a technical contract and individual acceptance fixture, open the area's design link below. The [full feature register](verification/phase8/v3/features.tsv) contains 620 rows with current status, engineering class and all ten execution-rule fields. The earlier user-supplied “Part 6” quality checklist remains in [REQUIREMENTS_MATRIX.md](REQUIREMENTS_MATRIX.md); v3.0 area 06 is Hybrid Compilation Architecture.

## Reading the audit

| Audit status | Meaning in this snapshot |
|---|---|
| Implemented and verified | The specified executable slice has a passing test or recorded measurement on a named host. |
| Implemented but unverified | Source exists for this feature, but its promised behavior lacks adequate execution evidence. |
| Partially implemented | Directly relevant behavior executes for a subset of the requested scope. The evidence column names its boundary. |
| Planned | A design, accepted direction or explicit project proposal exists. Implementation of the area remains pending. |
| Missing | No matching implementation or substantive prior design was identified in the audited package. This addendum supplies an intake contract. |
| Research required | Feasibility, semantics, hardware access or verification must be resolved before committing to delivery. |

Engineering class describes the proposed home of the work. Core requirement covers semantics and foundational compiler/quality infrastructure. Standard library capability covers reusable language APIs. Optional framework covers independent packages and tools. External integration covers platform, vendor, runtime and format boundaries. Experimental capability covers gated prototypes. Long-term research requires feasibility evidence. A row's primary class is an architectural recommendation. Individual subfeatures can belong in different layers.

## Area-level decisions

**0** implemented and verified | **0** implemented but unverified | **16** partially implemented | **13** planned | **14** missing | **7** research required.

### I. Universal language capabilities

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 01 | [Universal General-Purpose Programming](docs/EXPANSION_V3_ENGINEERING.md#01-universal-general-purpose-programming) | Partially implemented | Core requirement | Typed functions and host arithmetic execute. Application platforms lack deployable libraries and target contracts. |
| 02 | [Multi-Paradigm Programming](docs/EXPANSION_V3_ENGINEERING.md#02-multi-paradigm-programming) | Partially implemented | Core requirement | Procedures with immutable values and direct calls run. Advanced paradigms lack language or library implementations. |
| 03 | [Advanced Type System](docs/EXPANSION_V3_ENGINEERING.md#03-advanced-type-system) | Partially implemented | Core requirement | Finite structural tensor types and checked primitive types exist. Advanced polymorphism, effects and user ADTs remain open. |
| 04 | [Metaprogramming](docs/EXPANSION_V3_ENGINEERING.md#04-metaprogramming) | Missing | Experimental capability | Parser produces owned syntax. No user-visible macro, reflection or compile-time evaluator exists. |
| 05 | [Advanced Error Handling](docs/EXPANSION_V3_ENGINEERING.md#05-advanced-error-handling) | Partially implemented | Core requirement | Spanned compiler errors and recoverable checked runtime errors exist. Language Result/Option, cancellation and stack traces are future work. |

### II. Next-generation compiler

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 06 | [Hybrid Compilation Architecture](docs/EXPANSION_V3_ENGINEERING.md#06-hybrid-compilation-architecture) | Partially implemented | Core requirement | Tree-walking interpreter runs. VM/AOT have designs; JIT tiers and remote execution lack working engines. |
| 07 | [Multi-Backend Compiler Infrastructure](docs/EXPANSION_V3_ENGINEERING.md#07-multi-backend-compiler-infrastructure) | Planned | Core requirement | LLVM/MLIR, NVVM, SPIR-V and WGSL have proposed paths. Cargo builds the Rust interpreter only. |
| 08 | [Adaptive Optimization Engine](docs/EXPANSION_V3_ENGINEERING.md#08-adaptive-optimization-engine) | Planned | Experimental capability | Several static passes are designed. No Mongolia pass manager, adaptive recompilation or deoptimization ships. |
| 09 | [Hardware-Aware Autotuning](docs/EXPANSION_V3_ENGINEERING.md#09-hardware-aware-autotuning) | Research required | Experimental capability | Backend cache keys are proposed. There is no measured tuning search or persistent tuner. |
| 10 | [AI-Guided Compiler Optimization](docs/EXPANSION_V3_ENGINEERING.md#10-ai-guided-compiler-optimization) | Research required | Long-term research | No learned model selects passes or scheduling decisions. |
| 11 | [Compiler Correctness Engineering](docs/EXPANSION_V3_ENGINEERING.md#11-compiler-correctness-engineering) | Partially implemented | Core requirement | Whole-program validation, seeded input generation, differential arithmetic and fuzz campaigns exist. IR proofs, translation validation and staged bootstrap are open. |

### III. Universal memory and runtime

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 12 | [Unified Memory Architecture](docs/EXPANSION_V3_ENGINEERING.md#12-unified-memory-architecture) | Planned | Core requirement | Host allocation is internal to the interpreter. No user allocator, regions, manual memory API or borrow checker exists. |
| 13 | [Advanced Heterogeneous Memory](docs/EXPANSION_V3_ENGINEERING.md#13-advanced-heterogeneous-memory) | Planned | Core requirement | Completion leases, pinned buffers and async transfers have designs. No device allocator or transfer executes. |
| 14 | [Low-Latency Runtime](docs/EXPANSION_V3_ENGINEERING.md#14-low-latency-runtime) | Missing | Experimental capability | Startup and RSS are measured for a bounded interpreter. No real-time profile, task scheduler or standalone native runtime exists. |
| 15 | [Advanced Concurrency](docs/EXPANSION_V3_ENGINEERING.md#15-advanced-concurrency) | Missing | Core requirement | Single-thread interpreter has no language tasks, atomics, channels or scheduler. |
| 16 | [Distributed Computing](docs/EXPANSION_V3_ENGINEERING.md#16-distributed-computing) | Planned | Optional framework | Distributed tensors, collectives and failure semantics are design goals. No cluster runtime ships. |

### IV. Universal hardware support

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 17 | [CPU Architecture Expansion](docs/EXPANSION_V3_ENGINEERING.md#17-cpu-architecture-expansion) | Partially implemented | External integration | The Rust-hosted CLI is verified on x86-64 Linux. Mongolia-native CPU lowering and ISA-specific optimization have no execution record. |
| 18 | [GPU Architecture Expansion](docs/EXPANSION_V3_ENGINEERING.md#18-gpu-architecture-expansion) | Planned | External integration | CUDA, Vulkan and WebGPU have separate designs. No GPU kernel or driver call executes in v0.1. |
| 19 | [AI Accelerators](docs/EXPANSION_V3_ENGINEERING.md#19-ai-accelerators) | Research required | External integration | Generic device architecture is proposed. No NPU/TPU/DSP adapter or named device conformance exists. |
| 20 | [FPGA and Hardware Description](docs/EXPANSION_V3_ENGINEERING.md#20-fpga-and-hardware-description) | Research required | Long-term research | No HLS, RTL emitter, FPGA simulator or co-design toolchain exists. |
| 21 | [Embedded and Bare-Metal Computing](docs/EXPANSION_V3_ENGINEERING.md#21-embedded-and-bare-metal-computing) | Missing | External integration | Current binary needs Rust std, a host OS and heap. No freestanding target profile exists. |
| 22 | [Operating System Development](docs/EXPANSION_V3_ENGINEERING.md#22-operating-system-development) | Missing | Optional framework | No privileged target, syscall ABI, unsafe language blocks or kernel support exists. |

### V. Universal software development

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 23 | [Full-Stack Web Development](docs/EXPANSION_V3_ENGINEERING.md#23-full-stack-web-development) | Missing | Optional framework | Wasm is a feasibility target. No browser bindings, web server or framework exists. |
| 24 | [Mobile Development](docs/EXPANSION_V3_ENGINEERING.md#24-mobile-development) | Missing | External integration | No Android/iOS target integration, SDK bindings or signed app packaging exists. |
| 25 | [Desktop Development](docs/EXPANSION_V3_ENGINEERING.md#25-desktop-development) | Missing | Optional framework | The CLI runs on Linux and CI describes other hosts. GUI/application integration is absent. |
| 26 | [Game Development](docs/EXPANSION_V3_ENGINEERING.md#26-game-development) | Missing | Optional framework | No game engine, graphics renderer, asset pipeline or game loop ships. |
| 27 | [Cloud and Enterprise Development](docs/EXPANSION_V3_ENGINEERING.md#27-cloud-and-enterprise-development) | Missing | Optional framework | No deployable services, authentication middleware or cloud integration in the package. |
| 28 | [Database and Data Engineering](docs/EXPANSION_V3_ENGINEERING.md#28-database-and-data-engineering) | Missing | Optional framework | C/SQLite interoperability is a design direction. No SQL adapter, storage engine or Arrow/Parquet implementation exists. |
| 29 | [Scientific and Mathematical Computing](docs/EXPANSION_V3_ENGINEERING.md#29-scientific-and-mathematical-computing) | Partially implemented | Standard library capability | Dense f64 matmul and elementary arithmetic pass exact small-matrix checks. Advanced algorithms and precision families are open. |
| 30 | [Financial Computing](docs/EXPANSION_V3_ENGINEERING.md#30-financial-computing) | Missing | Optional framework | i64/f64 exist. Decimal, fixed-point and financial domain packages are absent. |
| 31 | [Blockchain and Cryptographic Computing](docs/EXPANSION_V3_ENGINEERING.md#31-blockchain-and-cryptographic-computing) | Partially implemented | Optional framework | Interpreter work/output budgets and deterministic reference operations exist. No consensus VM, smart-contract compiler or cryptographic package. |
| 32 | [Robotics and Real-Time Systems](docs/EXPANSION_V3_ENGINEERING.md#32-robotics-and-real-time-systems) | Missing | External integration | Single-host measurements give no deadline guarantee. ROS 2, controls and real-time runtime are absent. |
| 33 | [Quantum Computing Interfaces](docs/EXPANSION_V3_ENGINEERING.md#33-quantum-computing-interfaces) | Research required | Long-term research | No circuit representation or quantum provider interface exists. |

### VI. Advanced AI infrastructure

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 34 | [Native AI Super Engine](docs/EXPANSION_V3_ENGINEERING.md#34-native-ai-super-engine) | Partially implemented | Core requirement | Eager CPU tensor literals and checked rank-2 operations execute. Lazy graphs, AD, graph optimization and devices remain proposed. |
| 35 | [Large Language Model Infrastructure](docs/EXPANSION_V3_ENGINEERING.md#35-large-language-model-infrastructure) | Planned | Optional framework | Transformer-related APIs, KV caches, batching and interoperability are design targets. No model training or serving stack runs. |
| 36 | [Advanced Numerical Precision](docs/EXPANSION_V3_ENGINEERING.md#36-advanced-numerical-precision) | Partially implemented | Core requirement | f64 arithmetic/tensors are verified. f32 is reserved; other low-precision formats, scaling and calibration have no executable support. |
| 37 | [Native Dataflow and Graph Computing](docs/EXPANSION_V3_ENGINEERING.md#37-native-dataflow-and-graph-computing) | Planned | Experimental capability | Proposed tensor SSA supplies a graph design. No general dataflow graph runtime or serializer exists. |
| 38 | [AI Agent Infrastructure](docs/EXPANSION_V3_ENGINEERING.md#38-ai-agent-infrastructure) | Missing | Optional framework | No provider interface, agent framework or tool-permission layer exists. |

### VII. Universal interoperability

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 39 | [Mongolia Universal Bridge](docs/EXPANSION_V3_ENGINEERING.md#39-mongolia-universal-bridge) | Planned | External integration | C-compatible ABI and ownership rules have designs. No foreign code executes through mn. |
| 40 | [Legacy Code Migration](docs/EXPANSION_V3_ENGINEERING.md#40-legacy-code-migration) | Research required | Long-term research | No migration tool or semantic translator ships. |

### VIII. Developer productivity

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 41 | [Advanced Developer Toolchain](docs/EXPANSION_V3_ENGINEERING.md#41-advanced-developer-toolchain) | Planned | Optional framework | run/check/init and diagnostic rendering work. LSP/formatter tooling is designed. No listed debugger, interactive REPL or build cache ships. |
| 42 | [Mongolia Studio](docs/EXPANSION_V3_ENGINEERING.md#42-mongolia-studio) | Planned | Optional framework | Existing-editor/LSP integration is designed. No Studio executable or VS Code extension ships. |
| 43 | [Universal Build System](docs/EXPANSION_V3_ENGINEERING.md#43-universal-build-system) | Partially implemented | Core requirement | The compiler project has pinned Cargo builds and a checksummed source package. Mongolia has no build graph, linker frontend or package resolver. |

### IX. Security and reliability

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 44 | [Capability-Based Security](docs/EXPANSION_V3_ENGINEERING.md#44-capability-based-security) | Partially implemented | Core requirement | Source, tokens, depth, tensor elements, work and output have limits. No permission broker, process wall-clock cap or sandbox exists. |
| 45 | [High-Assurance Computing](docs/EXPANSION_V3_ENGINEERING.md#45-high-assurance-computing) | Partially implemented | Long-term research | Static typing, checked arithmetic and deterministic reference rules cover a subset. No proof, safety-critical profile or certification exists. |
| 46 | [Observability and Fault Tolerance](docs/EXPANSION_V3_ENGINEERING.md#46-observability-and-fault-tolerance) | Partially implemented | Standard library capability | The CLI reports localized runtime diagnostics and buffered output is discarded on failure. No service supervision or tracing pipeline exists. |

### X. Global performance and ecosystem

| ID | Official area and contract | Audit status | Primary engineering class | Current evidence and boundary |
|---|---|---|---|---|
| 47 | [Universal Performance Benchmarking](docs/EXPANSION_V3_ENGINEERING.md#47-universal-performance-benchmarking) | Partially implemented | Core requirement | Linux startup, lexer/parser, checked matmul, RSS and size have raw data, Python/BLAS references and numerical checks. Other domains lack comparable implementations. |
| 48 | [Independent Mongolia Technology Ecosystem](docs/EXPANSION_V3_ENGINEERING.md#48-independent-mongolia-technology-ecosystem) | Planned | Optional framework | Library domains have roadmap sketches. None of the nineteen named products is established by this repository. |
| 49 | [Proprietary-Named Open-Source Core Technologies](docs/EXPANSION_V3_ENGINEERING.md#49-proprietary-named-open-source-core-technologies) | Planned | Experimental capability | Eight proposed names map to design responsibilities. No named component implementation, ownership or naming clearance is asserted. |
| 50 | [Full Compiler Self-Hosting](docs/EXPANSION_V3_ENGINEERING.md#50-full-compiler-self-hosting) | Research required | Long-term research | Current compiler is Rust. Stage-0/1/2 and a Mongolia-written frontend/middle-end do not exist. |

## First implementable dependency slice

1. For 03, 06 and 11, resolve functions and local bindings to stable IDs in HIR for existing v0.1 programs. Preserve original byte spans and the first observable error. No new surface syntax is needed.
2. Verify the HIR, lower the same subset to a typed register VM, then compare interpreter/VM stdout, exit status, diagnostic spans, checked arithmetic, shape failures and budget boundaries. The first VM feature claim requires this evidence.
3. Add a minimal typed tensor IR and effect model before implementing AD or native lowering. Require adjoint/numerical tests for AD and separate CPU conformance before device or autotuning claims.
4. Keep domain libraries and product charters out of the compiler workspace until each has a concrete consumer and a maintainer. MACE, MUSE, MMIR, MATO, MAM, MAG, MUB and MAO remain proposed names pending governance and availability review.

The strategic scope is universal computing through a shared semantic model and reusable integrations. “Maximum performance” is a research objective. Each public performance or platform claim requires its own reproducible workload, reference, numerical contract and supported-target evidence.
