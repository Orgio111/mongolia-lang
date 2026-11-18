# Implementation priority and complexity

Priority orders the **next independently testable slice**. Maturity comes from the Phase 8 audit. Complexity covers the full area and estimates integration difficulty rather than elapsed time. These are proposed planning judgments. Only the resolved HIR work is scheduled for Phase 9. Other P0 rows contribute acceptance checks to that work.

| Scale | Operational meaning |
|---|---|
| P0 | Protect the running foundation or unblock the resolved-HIR handoff now. |
| P1 | A direct prerequisite for the original language, AI or native backend roadmap. |
| P2 | Start when named interfaces and a consumer exist. |
| P3 | Require a sponsor, hardware or feasibility RFC before allocating implementation work. |
| M | Bounded coordination or governance, with few runtime dependencies. |
| L | Multiple components or external environments with established integration methods. |
| XL | Changes to semantics, multiple targets, ecosystem-wide contracts or unresolved research. |

Entry gates refer to the [dependency graph](FEATURE_DEPENDENCY_GRAPH.md). They describe prerequisites for the proposed next slice. Later subfeatures have additional dependencies in the [item register](verification/phase8/v3/features.tsv). A legacy implementation can already supply part of a future gate.

| ID | Official area | Priority | Audited maturity | Complexity | Entry gates | Next slice |
|---|---|---|---|---|---|---|
| 01 | Universal General-Purpose Programming | P2 | Partially implemented | XL | L, B | Portable language subset and one independent consumer |
| 02 | Multi-Paradigm Programming | P1 | Partially implemented | XL | H, L | Immutable procedures through resolved slots, then generic functions |
| 03 | Advanced Type System | P0 | Partially implemented | XL | A, H | Exact local and return types with stable resolved identities |
| 04 | Metaprogramming | P3 | Missing | XL | L, Q | Hygiene and fuel limits for one deterministic expansion |
| 05 | Advanced Error Handling | P0 | Partially implemented | L | A, H | Preserve the first error, span and exit status in lowering |
| 06 | Hybrid Compilation Architecture | P0 | Partially implemented | XL | A, H | Forward calls by function ID and variables by slot |
| 07 | Multi-Backend Compiler Infrastructure | P1 | Planned | XL | T, M | Legalized CPU matmul with a verified lowering boundary |
| 08 | Adaptive Optimization Engine | P2 | Planned | XL | T, C | One proved, effect-preserving optimization before adaptive passes |
| 09 | Hardware-Aware Autotuning | P3 | Research required | XL | O, N | Compare two valid kernels and invalidate mismatched cache keys |
| 10 | AI-Guided Compiler Optimization | P3 | Research required | XL | O | Held-out evaluation against a fixed verified pass pipeline |
| 11 | Compiler Correctness Engineering | P0 | Partially implemented | XL | A | Negative verifier fixtures and differential execution oracle |
| 12 | Unified Memory Architecture | P1 | Planned | XL | T | Make buffer identity, owner and checked byte length explicit |
| 13 | Advanced Heterogeneous Memory | P1 | Planned | XL | M | Simulate stream completion and deferred reclamation |
| 14 | Low-Latency Runtime | P3 | Missing | XL | Q | Qualify a bounded task profile on a named host |
| 15 | Advanced Concurrency | P2 | Missing | XL | S, M | Join a scoped task under normal completion and cancellation |
| 16 | Distributed Computing | P2 | Planned | XL | Q, K | Two workers with explicit idempotency and replay policy |
| 17 | CPU Architecture Expansion | P1 | Partially implemented | XL | M | CPU target ABI and strict scalar reference on two architectures |
| 18 | GPU Architecture Expansion | P1 | Planned | XL | M, C | One explicit transfer and checked CUDA kernel, then portable paths |
| 19 | AI Accelerators | P3 | Research required | XL | M, F | One vendor adapter with named physical device and dtype evidence |
| 20 | FPGA and Hardware Description | P3 | Research required | XL | T, X | Cycle-accurate simulation of one pure dataflow kernel |
| 21 | Embedded and Bare-Metal Computing | P3 | Missing | XL | X, M | Emulator boot of a no-host-service fixture |
| 22 | Operating System Development | P3 | Missing | XL | X, Q | An isolated emulator service after freestanding boot works |
| 23 | Full-Stack Web Development | P2 | Missing | L | B, X | One browser interoperability and request/response fixture |
| 24 | Mobile Development | P3 | Missing | L | B, X | One installed test app with checked callback lifetime |
| 25 | Desktop Development | P2 | Missing | L | B, X | Accessible native control on two declared hosts |
| 26 | Game Development | P3 | Missing | XL | B, Q | A headless scene with deterministic state replay |
| 27 | Cloud and Enterprise Development | P2 | Missing | XL | B, Q | Versioned service with bounded retries and restart test |
| 28 | Database and Data Engineering | P2 | Missing | XL | B, F | Transactional database adapter before storage-engine work |
| 29 | Scientific and Mathematical Computing | P1 | Partially implemented | XL | A | Keep exact dense matrix results as oracle for one new routine |
| 30 | Financial Computing | P3 | Missing | L | B, L | Exact decimal rounding and ledger replay |
| 31 | Blockchain and Cryptographic Computing | P3 | Partially implemented | XL | B, Q | A versioned deterministic profile tested against budget boundaries |
| 32 | Robotics and Real-Time Systems | P3 | Missing | XL | X, Q | Bounded non-actuating sensor replay, with recorded jitter |
| 33 | Quantum Computing Interfaces | P3 | Research required | XL | B, F | Classical simulator round trip for one circuit format |
| 34 | Native AI Super Engine | P1 | Partially implemented | XL | T | A pure tensor region with verified shapes and effects |
| 35 | Large Language Model Infrastructure | P2 | Planned | XL | I | Pinned tiny transformer forward pass and cached-token parity |
| 36 | Advanced Numerical Precision | P1 | Partially implemented | XL | T | One explicit f32 conversion and reference rounding policy |
| 37 | Native Dataflow and Graph Computing | P2 | Planned | XL | T | A verified pure DAG with fresh/incremental output parity |
| 38 | AI Agent Infrastructure | P3 | Missing | L | B, Q | Permission-limited mock tool with durable replay |
| 39 | Mongolia Universal Bridge | P1 | Planned | XL | M, C | Owned/borrowed C buffer round trip with error translation |
| 40 | Legacy Code Migration | P3 | Research required | XL | H, F | Compare one supported pure subset with a source interpreter |
| 41 | Advanced Developer Toolchain | P1 | Planned | L | H | Map an editor revision to a scoped definition and original span |
| 42 | Mongolia Studio | P3 | Planned | L | E | Existing-editor tensor inspection before an IDE charter |
| 43 | Universal Build System | P0 | Partially implemented | XL | A | Locked workspace and reproducible verification before language build graphs |
| 44 | Capability-Based Security | P0 | Partially implemented | XL | A | Preserve v0.1 budget enforcement and failure atomicity for output |
| 45 | High-Assurance Computing | P2 | Partially implemented | XL | H | State a small invariant and compare its model with execution |
| 46 | Observability and Fault Tolerance | P1 | Partially implemented | L | A | Preserve diagnostic origins and stable, localized CLI outcomes |
| 47 | Universal Performance Benchmarking | P0 | Partially implemented | L | A | Reproduce the Phase 8 workload before tuning the new pipeline |
| 48 | Independent Mongolia Technology Ecosystem | P2 | Planned | XL | K, F | Charter one independently maintained library with real consumer |
| 49 | Proprietary-Named Open-Source Core Technologies | P3 | Planned | M | H | Map proposed names to measured components before naming review |
| 50 | Full Compiler Self-Hosting | P3 | Research required | XL | J | Port a small self-contained tool before stage bootstrap |
