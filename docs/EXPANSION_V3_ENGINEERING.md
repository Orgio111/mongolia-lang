# v3.0 engineering contracts

The official 50 headings are the audit units. This companion specifies each workstream and lists the individual items the user supplied. Every item has a delivery class, a current status and a feature-specific test or demonstration. Each inherits its area's technical constraints, dependencies, milestones, test strategy, risk, maintenance and compatibility obligations. The [feature register](../verification/phase8/v3/features.tsv) expands all ten fields on every row for filtering or issue import. These are proposed acceptance contracts. A future-tense test has no recorded pass until an implementation and its evidence are attached.

Dependencies reference required slices of other areas. For example, design resource/event ownership in 13 using a simulated queue, then implement a single GPU dispatch in 18, then qualify peer transfers in 13. This sequencing avoids a circular implementation gate. The source plan's numbering applies to areas; within an area labels are grouped by engineering work, without assigning new official numbers.

## 01. Universal General-Purpose Programming

**Partially implemented · Core requirement**

Typed functions and host arithmetic execute. Application platforms lack deployable libraries and target contracts.

Evidence · [SPEC](../SPEC.md) · [CLI](../crates/mn-cli/src/main.rs) · [TEST](../verification/phase8/command-03.log)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Keep portable value/control semantics in core and platform-facing services behind modules. |
| Required dependencies | 02,03,05; libraries depend on 39 |
| Implementation milestones | Freeze a portable program corpus, then add each domain through an independent reference application. |
| Acceptance criteria | Build and run a minimal application on its declared target with dependencies recorded. |
| Correctness tests | Compare shared computations with the interpreter and check unsupported API diagnostics. |
| Performance tests | Separate compute, startup and boundary-call costs per application. |
| Security risks | Ambient OS authority and unsafe platform APIs. |
| Maintenance | Language team owns semantics; domain maintainers own platform fixtures. |
| Compatibility | Publish support per domain, target and SDK version. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Systems programming | Core requirement | Missing | Compile a checked buffer routine with a specified ABI |
| Operating systems | Optional framework | Missing | Boot a minimal kernel fixture in an emulator |
| Web development | Optional framework | Missing | Serve and render a request |
| Mobile applications | Optional framework | Missing | Install and launch a signed test app |
| Desktop applications | Optional framework | Missing | Open an accessible window |
| Game development | Optional framework | Missing | Replay a deterministic scene |
| Embedded systems | Optional framework | Missing | Run a bounded freestanding image |
| Robotics | Optional framework | Missing | Replay a simulated sensor stream |
| Cloud computing | Optional framework | Missing | Serve a health-checked deployment |
| Scientific computing | Standard library capability | Partially implemented | Reproduce a reference dense matrix product |
| Financial technology | Optional framework | Missing | Replay exact decimal ledger totals |
| Database engines | Optional framework | Missing | Recover a committed transaction after restart |
| Blockchain infrastructure | Optional framework | Missing | Replay a versioned state transition |
| Cybersecurity | External integration | Missing | Validate a defensive log-analysis fixture |
| Aerospace computing | Long-term research | Missing | Verify a simulated benign telemetry pipeline against its timing specification |
| Real-time systems | Long-term research | Missing | Demonstrate a documented deadline bound on a named target |
| Quantum computing interfaces | Long-term research | Missing | Round-trip a circuit and identify the execution provider |

## 02. Multi-Paradigm Programming

**Partially implemented · Core requirement**

Procedures with immutable values and direct calls run. Advanced paradigms lack language or library implementations.

Evidence · [SPEC](../SPEC.md) · [RUN](../crates/mn-core/src/engine/runtime.rs) · [TEST](../verification/phase8/command-03.log)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Define binding, calls, effects and evaluation order once for all paradigms. |
| Required dependencies | 03,05; specialized paradigms require 15,16,34 |
| Implementation milestones | Establish procedural fixtures, then pure functions and generics, then effectful libraries. |
| Acceptance criteria | Two paradigms expressing the same pure calculation produce the same value/error ordering. |
| Correctness tests | Compare equivalent programs including nested calls and errors in arguments. |
| Performance tests | Measure calls/closures, allocation and dispatch against the direct procedural case. |
| Security risks | Hidden effects and unsound aliasing. |
| Maintenance | Language design group curates cross-paradigm fixtures. |
| Compatibility | Syntax additions require an edition or an additive RFC. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Procedural programming | Core requirement | Partially implemented | Call typed functions with local immutable bindings and explicit returns |
| Functional programming | Core requirement | Partially implemented | Preserve pure evaluation and add first-class functions |
| Object-oriented programming | Core requirement | Missing | Verify dispatch and subtype contracts |
| Generic programming | Core requirement | Planned | Type-check valid instantiations and reject unsatisfied constraints |
| Declarative programming | Standard library capability | Missing | Evaluate a declared query against a reference result |
| Reactive programming | Standard library capability | Missing | Propagate one update through a bounded dependency graph |
| Data-oriented programming | Core requirement | Partially implemented | Validate row-major tensor results and introduce explicit layout APIs |
| Concurrent programming | Core requirement | Missing | Join a scoped task with deterministic lifetime handling |
| Distributed programming | Optional framework | Planned | Execute a two-worker computation with defined failure behavior |
| Differentiable programming | Core requirement | Planned | Validate a scalar derivative against finite differences |
| Logic programming through suitable libraries | Optional framework | Missing | Resolve a finite rule query with documented termination |
| Event-driven programming | Optional framework | Missing | Dispatch ordered events with specified cancellation |

## 03. Advanced Type System

**Partially implemented · Core requirement**

Finite structural tensor types and checked primitive types exist. Advanced polymorphism, effects and user ADTs remain open.

Evidence · [CHECK](../crates/mn-core/src/engine/checker.rs) · [SPEC](../SPEC.md) · [FUTURE](../docs/FUTURE_DESIGN.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Introduce a decidable base with explicit boundaries for effect, shape and dynamic checks. |
| Required dependencies | Resolved HIR, binding model and constraint diagnostics |
| Implementation milestones | Add ADTs/exhaustiveness then generics, followed by isolated constraint experiments. |
| Acceptance criteria | Specify a terminating judgment or resource diagnostic for each newly accepted form. |
| Correctness tests | Positive and negative typing fixtures, including ambiguous and recursive constraints. |
| Performance tests | Measure checking time and peak memory versus nesting and constraint count. |
| Security risks | Nontermination and accepted unsound coercions. |
| Maintenance | Type-system owners maintain versioned rules and inference cases. |
| Compatibility | Gate inference changes on the previous accepted-program corpus. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Algebraic data types | Core requirement | Planned | Encode a sum and check exhaustive constructors |
| Higher-order functions | Core requirement | Planned | Type a function passed and returned by another function |
| Const generics | Core requirement | Planned | Instantiate a checked size argument |
| Higher-rank polymorphism | Core requirement | Research required | Check a binder-sensitive example under a decidable rule |
| Associated types | Core requirement | Missing | Resolve a constrained implementation-associated result |
| Refinement types | Core requirement | Research required | Reject a counterexample or emit a bounded solver diagnostic |
| Dependent shape types | Core requirement | Partially implemented | Verify known matmul dimensions and add dependent binders |
| Type-level programming | Core requirement | Research required | Bound reduction and normalize equivalent type expressions |
| Structural and nominal typing where appropriate | Core requirement | Partially implemented | Retain structural tensor equality and test nominal identity |
| Existential types | Core requirement | Research required | Hide a representation without exposing its witness |
| Exhaustive pattern matching | Core requirement | Planned | Report a missing variant |
| Effect tracking | Core requirement | Planned | Reject an undeclared effect in a pure function |
| Compile-time constraint solving | Core requirement | Partially implemented | Refute a static contraction mismatch and add symbolic constraints |
| Optional dynamic values with explicit boundaries | Experimental capability | Missing | Check a failing dynamic-to-static conversion |

## 04. Metaprogramming

**Missing · Experimental capability**

Parser produces owned syntax. No user-visible macro, reflection or compile-time evaluator exists.

Evidence · [GRAM](../grammar.ebnf) · [ARCH](../ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use scoped expansion and a deterministic compile-time evaluator with explicit inputs. |
| Required dependencies | Stable syntax identities, hygiene and effects |
| Implementation milestones | Hygiene prototype, bounded evaluator, typed expansion verifier. |
| Acceptance criteria | Identical declared inputs yield identical expanded syntax and diagnostics. |
| Correctness tests | Capture/shadow cases, staged type error and nondeterministic-input rejection. |
| Performance tests | Record expansion time and output growth with increasing nesting. |
| Security risks | Build-time effects and expansion exhaustion. |
| Maintenance | Frontend owners version expansion traces and budgets. |
| Compatibility | Version the macro API separately from compiler internals. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Hygienic macros | Experimental capability | Missing | Prevent accidental local capture |
| Compile-time function execution | Experimental capability | Missing | Recompute a pure function deterministically |
| AST transformations | Experimental capability | Missing | Preserve span provenance through a rewrite |
| Typed code generation | Experimental capability | Missing | Verify the emitted type |
| Compile-time reflection | Experimental capability | Missing | Query a declared type without host access |
| Derive macros | Experimental capability | Missing | Generate and type-check a derived implementation |
| Domain-specific language construction | Experimental capability | Missing | Compile a tiny embedded DSL with source labels |
| Staged programming | Long-term research | Research required | Verify cross-stage value restrictions |
| Partial evaluation | Long-term research | Research required | Compare specialized and unspecialized execution |
| Compile-time serialization generation | Experimental capability | Missing | Round-trip generated codecs across a schema revision |

## 05. Advanced Error Handling

**Partially implemented · Core requirement**

Spanned compiler errors and recoverable checked runtime errors exist. Language Result/Option, cancellation and stack traces are future work.

Evidence · [DIAG](../crates/mn-core/src/diagnostics.rs) · [RUN](../crates/mn-core/src/engine/runtime.rs) · [CLI](../crates/mn-cli/src/main.rs)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Distinguish typed program failures, cancellation and host faults; preserve original source context. |
| Required dependencies | 03, effect model and cleanup semantics |
| Implementation milestones | Typed results, propagation, then unwinding/cleanup and cancellation boundaries. |
| Acceptance criteria | Each failure has a defined propagation path and observable cleanup behavior. |
| Correctness tests | Nested failures, exact division error, interrupted resource ownership. |
| Performance tests | Measure success-path overhead and error/stack rendering separately. |
| Security risks | Lost cleanup or sensitive data in diagnostic chains. |
| Maintenance | Runtime owners maintain failure injection and the bilingual catalog. |
| Compatibility | Retain public exit codes and document changes in error taxonomy. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Typed errors | Core requirement | Missing | Distinguish program-level error variants |
| Result and Option types | Core requirement | Planned | Check success and absence branches |
| Error propagation | Core requirement | Planned | Propagate a typed failure through nested calls |
| Structured error chains | Core requirement | Missing | Retain cause identity and original location |
| Recoverable runtime errors | Core requirement | Implemented and verified | Check division by zero returns a diagnostic and exit 1 |
| Stack traces | Core requirement | Missing | Render function frames with original spans |
| Resource cleanup | Core requirement | Partially implemented | Release live resources on an early return and error |
| Cancellation propagation | Core requirement | Missing | Cancel a child task and await cleanup |
| Panic isolation | Core requirement | Missing | Contain a panic at a defined boundary |
| Configurable panic behavior | Core requirement | Missing | Select a documented panic policy |
| Fault recovery | Standard library capability | Missing | Recover an injected failure under a stated state contract |

## 06. Hybrid Compilation Architecture

**Partially implemented · Core requirement**

Tree-walking interpreter runs. VM/AOT have designs; JIT tiers and remote execution lack working engines.

Evidence · [RUN](../crates/mn-core/src/engine/runtime.rs) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [CLI](../crates/mn-cli/src/main.rs)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Select an execution tier behind one verified representation and effect model. |
| Required dependencies | 03,11; validated HIR and MIR |
| Implementation milestones | AST oracle, register VM, AOT, then separately justified tiering prototypes. |
| Acceptance criteria | Each advertised strategy reproduces output, traps and source origins. |
| Correctness tests | Differential fixture runs including early failure, side effects and resource budgets. |
| Performance tests | Report cold compile, first execution and steady throughput separately. |
| Security risks | Unverified generated code and stale cache entries. |
| Maintenance | Compiler team versions tier contracts and records target provenance. |
| Compatibility | Backend selection preserves observable language semantics. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Tree-walking interpreter | Core requirement | Implemented and verified | Execute checked AST programs in both syntaxes |
| Bytecode interpreter | Core requirement | Planned | Compare typed bytecode execution with the AST oracle |
| Baseline JIT compiler | Experimental capability | Research required | Emit one native block with correct traps |
| Optimizing JIT compiler | Experimental capability | Research required | Compare optimized and baseline execution |
| Ahead-of-time native compiler | Core requirement | Planned | Run one compiled binary against interpreter expectations |
| Tiered compilation | Experimental capability | Research required | Preserve frames across a tier transition |
| Incremental compilation | Core requirement | Planned | Compare edited and fresh build artifacts |
| Profile-guided recompilation | Experimental capability | Research required | Replay a profile with deterministic output |
| Cross-compilation | Core requirement | Planned | Execute an artifact on a different named target |
| Remote compilation | External integration | Missing | Authenticate a build result from one remote worker |
| Distributed compilation | Experimental capability | Missing | Compare partitioned and serial build outputs |

## 07. Multi-Backend Compiler Infrastructure

**Planned · Core requirement**

LLVM/MLIR, NVVM, SPIR-V and WGSL have proposed paths. Cargo builds the Rust interpreter only.

Evidence · [ARCH](../ARCHITECTURE.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use a shared verifier and one lowering adapter per materially distinct target. |
| Required dependencies | 06,11; capability registry |
| Implementation milestones | Define target capabilities, lower CPU fixture, then add each device path. |
| Acceptance criteria | Each claimed operation runs on a named target or reports unsupported capability. |
| Correctness tests | Differential arithmetic, shape guards and ABI/layout fixtures per backend. |
| Performance tests | Report lowering latency, artifact size and target execution time. |
| Security risks | Driver ABI and code-generation trust boundaries. |
| Maintenance | Backend owners pin toolchain versions and provide target runners. |
| Compatibility | Version IR interchange and publish target capability tables. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| LLVM | External integration | Planned | Verify generated LLVM IR and run the CPU fixture |
| MLIR | External integration | Planned | Verify typed lowering across dialect boundaries |
| SPIR-V | External integration | Planned | Validate and execute a compute module |
| WGSL | External integration | Planned | Compile a supported shader on WebGPU |
| NVVM | External integration | Planned | Verify a GPU kernel lowering |
| PTX | External integration | Planned | Load and run a version-matched kernel |
| Cranelift | External integration | Research required | Evaluate a second emitter against a measured unmet latency target |
| WebAssembly | External integration | Research required | Validate a Wasm module on a named runtime |
| Native machine-code emission | Core requirement | Planned | Execute target output against the reference |
| C source generation for selected targets | Experimental capability | Research required | Compile emitted C with two toolchains and compare |

## 08. Adaptive Optimization Engine

**Planned · Experimental capability**

Several static passes are designed. No Mongolia pass manager, adaptive recompilation or deoptimization ships.

Evidence · [ARCH](../ARCHITECTURE.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Give each transformation explicit legality, invalidation and rollback conditions. |
| Required dependencies | 06,07,11; profiles, liveness and effects |
| Implementation milestones | Verified static passes, offline profiles, then guarded speculative regions. |
| Acceptance criteria | Disabled and enabled passes preserve traps, output and strict floating behavior. |
| Correctness tests | Differential tests at guard failures, calls and exceptional exits. |
| Performance tests | Measure compile-time cost, warmup and post-optimization throughput. |
| Security risks | Incorrect deoptimization state or leaked profile inputs. |
| Maintenance | Pass owners keep legality and equivalence fixtures per optimization. |
| Compatibility | Make target-specific or relaxed math options explicit. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Runtime specialization | Experimental capability | Planned | Reject a specialization when a shape guard fails |
| Profile-guided optimization | Experimental capability | Missing | Validate profile schema and output equivalence |
| Link-time optimization | External integration | Missing | Compare whole-link output under on/off settings |
| ThinLTO | External integration | Missing | Check summary/index compatibility and outputs |
| Speculative optimization | Long-term research | Research required | Force a failed assumption and restore reference behavior |
| Deoptimization | Long-term research | Research required | Reconstruct locals and source location |
| On-stack replacement | Long-term research | Research required | Continue a live invocation across tiers |
| Dynamic dispatch optimization | Experimental capability | Missing | Check guarded fast and generic call paths |
| Escape analysis | Experimental capability | Planned | Verify escaping values retain lifetime |
| Alias analysis | Core requirement | Planned | Preserve effects for possibly aliased values |
| Interprocedural optimization | Experimental capability | Planned | Track effects through a call edge |
| Whole-program optimization | Experimental capability | Planned | Keep externally observable behavior after global rewrites |
| Automatic vectorization | Experimental capability | Planned | Compare strict scalar/vector results on tail elements |
| Automatic parallelization | Long-term research | Research required | Prove independence before scheduling iterations |
| Adaptive inlining | Experimental capability | Missing | Bound code growth using representative call profiles |

## 09. Hardware-Aware Autotuning

**Research required · Experimental capability**

Backend cache keys are proposed. There is no measured tuning search or persistent tuner.

Evidence · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [BENCH](../BENCHMARK_BASELINE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Tune only among numerically validated kernels within resource and power budgets. |
| Required dependencies | 07,11,47; target telemetry |
| Implementation milestones | Collect device facts, benchmark candidates, validate, then cache the winner. |
| Acceptance criteria | A changed compiler, device or numeric policy invalidates cached choices. |
| Correctness tests | Check tuned results against the reference and test stale/missing measurements. |
| Performance tests | Measure search overhead, reuse break-even and hold-out performance. |
| Security risks | Untrusted cache entries and uncontrolled resource use. |
| Maintenance | Numerical/backend owners maintain representative tuning workloads. |
| Compatibility | Cache schema and math mode form part of compatibility. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Processor architecture | Experimental capability | Research required | Reject tuning data for a different microarchitecture |
| CPU cache hierarchy | Experimental capability | Research required | Measure tile choices across working-set sizes |
| GPU compute capabilities | Experimental capability | Research required | Exclude kernels requiring absent features |
| Tensor dimensions | Experimental capability | Research required | Select only guards covering the actual shape |
| Data types | Experimental capability | Research required | Revalidate error bounds at each dtype |
| Memory bandwidth | Experimental capability | Research required | Measure transfer-limited candidates separately |
| Available RAM and VRAM | Experimental capability | Research required | Reject candidates exceeding the budget |
| Device transfer costs | Experimental capability | Research required | Include copies in end-to-end selection |
| Kernel launch overhead | Experimental capability | Research required | Measure empty-launch overhead separately |
| Power consumption | Experimental capability | Research required | Compare energy under a fixed workload |
| Runtime workload characteristics | Experimental capability | Research required | Evaluate held-out workload distributions |

## 10. AI-Guided Compiler Optimization

**Research required · Long-term research**

No learned model selects passes or scheduling decisions.

Evidence · [ARCH](../ARCHITECTURE.md) · [BENCH](../BENCHMARK_BASELINE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Constrain model choices to validated transformations with deterministic fallback. |
| Required dependencies | 08,09,11,47; verified candidate policy |
| Implementation milestones | Collect baseline data, train offline, evaluate on held-out programs, gate rollout. |
| Acceptance criteria | Reject invalid candidates and reproduce decisions from pinned model and features. |
| Correctness tests | Independent verifier plus comparison to the untuned compiler for every selected plan. |
| Performance tests | Report inference time, compile latency and held-out speed/accuracy distributions. |
| Security risks | Poisoned training inputs and nondeterministic decisions. |
| Maintenance | Research owners publish model provenance, evaluation and rollback path. |
| Compatibility | Model updates never alter required semantics. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Compiler pass selection | Long-term research | Research required | Verify selected passes against the baseline |
| Pass ordering | Long-term research | Research required | Compare candidate pipelines with a fixed IR corpus |
| Kernel scheduling | Long-term research | Research required | Validate tile and synchronization legality |
| Memory layout selection | Long-term research | Research required | Verify tensor indices after layout conversion |
| Instruction scheduling | Long-term research | Research required | Preserve dependencies and traps |
| Device selection | Long-term research | Research required | Check numerical capability before selecting a device |
| Loop transformations | Long-term research | Research required | Prove iteration domain and dependence preservation |
| Compilation latency optimization | Long-term research | Research required | Evaluate build time with bounded inference overhead |
| Performance prediction | Long-term research | Research required | Measure held-out prediction error and fallback cost |

## 11. Compiler Correctness Engineering

**Partially implemented · Core requirement**

Whole-program validation, seeded input generation, differential arithmetic and fuzz campaigns exist. IR proofs, translation validation and staged bootstrap are open.

Evidence · [TEST](../verification/phase8/command-03.log) · [ORACLE](../verification/phase8/command-10.log) · [FUZZ](../verification/phase8/fuzz-parser.log) · [SPEC](../SPEC.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Treat the validated interpreter as the executable oracle; introduce verification at every new IR boundary. |
| Required dependencies | Normative semantics; each new IR needs a verifier |
| Implementation milestones | Grow semantic corpus, verify HIR/MIR, compare second backend, then formalize critical operations. |
| Acceptance criteria | Every lowering rejects malformed IR and has a reproducible differential corpus. |
| Correctness tests | Generate typed inputs, vary equivalent expressions and compare diagnostics, values and effects. |
| Performance tests | Measure verifier cost and corpus throughput by stage. |
| Security risks | Silent miscompilation, unspecified floating behavior and unsound validation. |
| Maintenance | Compiler QA keeps minimized regressions, seeds and expected outputs. |
| Compatibility | Semantic changes require a spec diff and migration judgment. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Formal operational semantics | Core requirement | Partially implemented | Relate reference evaluation rules to an independently checkable model |
| IR invariant verification | Core requirement | Planned | Reject malformed typed IR before execution |
| Translation validation | Experimental capability | Research required | Validate each lowered fragment against source semantics |
| Differential compiler testing | Core requirement | Partially implemented | Compare a second compiler backend with the existing AST and matrix oracle |
| Metamorphic testing | Core requirement | Implemented and verified | Verify matrix identity and bilingual spelling invariance |
| Random program generation | Core requirement | Partially implemented | Extend seeded mutation and generated arithmetic to typed whole programs |
| Miscompilation detection | Core requirement | Partially implemented | Detect mismatched outputs and minimize a failing program |
| Memory model verification | Long-term research | Research required | Model alias/lifetime invariants across asynchronous completion |
| Optimization equivalence checking | Long-term research | Research required | Prove or refute rewrites on a bounded fragment |
| Undefined behavior analysis | Long-term research | Research required | Enumerate forbidden operations and check proof obligations |
| Verified critical compiler components | Long-term research | Research required | Independently check a proof for one critical component |
| Reproducible compiler bootstrap | Long-term research | Research required | Compare independent multi-stage bootstrap artifacts |

## 12. Unified Memory Architecture

**Planned · Core requirement**

Host allocation is internal to the interpreter. No user allocator, regions, manual memory API or borrow checker exists.

Evidence · [RUN](../crates/mn-core/src/engine/runtime.rs) · [TENSOR](../crates/mn-core/src/engine/tensor.rs) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Represent allocation origin, lifetime and aliasing in typed operations before exposing allocator choices. |
| Required dependencies | 03,11; layout/lifetime IR |
| Implementation milestones | Host ownership model, checked allocators, then regions and sharing policies. |
| Acceptance criteria | Every allocation has a documented owner, failure mode and compatible release path. |
| Correctness tests | Model move/borrow errors, allocator mismatch, repeated allocation and checked-size failures. |
| Performance tests | Peak live bytes, copies, fragmentation and allocation latency by strategy. |
| Security risks | Invalid lifetimes, double release and memory exhaustion. |
| Maintenance | Runtime owners maintain allocator contracts and fault fixtures. |
| Compatibility | Specify layout/ownership across API boundaries before exposing stable ABI. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Ownership and borrowing | Core requirement | Planned | Reject conflicts between shared and exclusive borrows |
| Region-based allocation | Experimental capability | Missing | Forbid values escaping the region |
| Arena allocation | Experimental capability | Missing | Release an arena only after its last live reference |
| Stack allocation | Experimental capability | Missing | Reject escaping stack addresses |
| Heap allocation | Core requirement | Partially implemented | Preserve checked tensor allocation failure behavior |
| Explicit manual allocation | Experimental capability | Missing | Require a documented owner and deallocation contract |
| Typed allocators | Experimental capability | Missing | Reject freeing into an incompatible allocator |
| Memory pools | Core requirement | Planned | Delay reuse until all dependencies complete |
| Zero-copy buffers | External integration | Planned | Prove ownership/layout compatibility on exchange |
| Reference counting | Core requirement | Planned | Keep a shared resource alive through its last handle |
| Optional managed memory | Experimental capability | Research required | Evaluate collector interaction with owned values and pause budgets |
| NUMA-aware allocation | Experimental capability | Research required | Measure verified locality on a NUMA machine |
| Huge-page support | Experimental capability | Research required | Verify observable behavior with huge pages available/unavailable |
| Memory-mapped files | External integration | Missing | Validate mapping bounds and access modes |
| Shared-memory interfaces | External integration | Missing | Reject incompatible shared layout and access |

## 13. Advanced Heterogeneous Memory

**Planned · Core requirement**

Completion leases, pinned buffers and async transfers have designs. No device allocator or transfer executes.

Evidence · [FUTURE](../docs/FUTURE_DESIGN.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Hold allocation leases until all dependent completion events finish. |
| Required dependencies | 12,18; event model and device ABI |
| Implementation milestones | Single queue transfer, cross-stream dependencies, reclamation, then peer transfers. |
| Acceptance criteria | Dropping a user handle cannot recycle an allocation before final completion. |
| Correctness tests | Fault-inject failed transfer, device loss, reorder completion and aliasing. |
| Performance tests | Measure transfers, residency, peak VRAM and fragmentation by workload. |
| Security risks | Use-after-free on queued work and cross-device data exposure. |
| Maintenance | Device owners maintain driver/version compatibility and fault tests. |
| Compatibility | Expose device identity and transfer/sync costs explicitly. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Unified virtual addressing where supported | External integration | Research required | Probe support and verify host/device pointer rules |
| Device-local memory | Core requirement | Planned | Reject cross-device access without transfer |
| Pinned host memory | Core requirement | Planned | Keep source pages live until completion |
| GPU memory virtualization where supported | External integration | Research required | Detect unsupported mapping and define revocation |
| Memory residency tracking | Experimental capability | Planned | Account for every live residency transition |
| Device-to-device transfers | External integration | Planned | Compare copied values and report synchronization |
| Peer-to-peer GPU communication | External integration | Research required | Exercise supported peers and reject absent topology |
| Tensor offloading | Experimental capability | Planned | Restore values and ownership after a spill |
| Asynchronous memory reclamation | Experimental capability | Planned | Verify pending buffers cannot be reused |
| Memory fragmentation reduction | Experimental capability | Planned | Measure fragmentation at equal maximum live bytes |
| Allocation lifetime analysis | Experimental capability | Planned | Check last use across streams and failed commands |

## 14. Low-Latency Runtime

**Missing · Experimental capability**

Startup and RSS are measured for a bounded interpreter. No real-time profile, task scheduler or standalone native runtime exists.

Evidence · [RUN](../crates/mn-core/src/engine/runtime.rs) · [BENCH](../BENCHMARK_BASELINE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Specify bounded profiles and measure tail latency under defined load. |
| Required dependencies | 06,12,15,47 |
| Implementation milestones | Inventory allocations, expose profile contracts, then implement suitable task/runtime policies. |
| Acceptance criteria | A profile lists allowed allocations and demonstrates measured deadline behavior. |
| Correctness tests | Exercise cold start, pressure, cancellation and blocked operations. |
| Performance tests | Publish p50/p95/p99/max and misses with hardware/load details. |
| Security risks | Unbounded pauses and incorrect deadline guarantees. |
| Maintenance | Runtime team maintains target-specific tail-latency suites. |
| Compatibility | Profiles are opt-in and promises name the target/workload. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Predictable allocation behavior | Core requirement | Partially implemented | Retain checked buffers and bound allocations in a target profile |
| Configurable runtime footprints | Experimental capability | Missing | Measure a minimal and a full runtime configuration |
| Stackless asynchronous tasks | Experimental capability | Missing | Resume a suspended task without a native stack per task |
| Lightweight task scheduling | Experimental capability | Missing | Measure ready-to-run delay under saturation |
| Runtime initialization optimization | Experimental capability | Missing | Measure cold start with phase attribution |
| Low-overhead synchronization | Experimental capability | Missing | Verify memory order before measuring contention |
| Real-time-compatible execution profiles | Long-term research | Research required | Specify deadline assumptions and verify under load |
| Minimal standalone runtime | Experimental capability | Missing | Link and run a program with a declared runtime footprint |
| Runtime-free native compilation where feasible | Long-term research | Research required | Identify a subset that links with no runtime dependencies |

## 15. Advanced Concurrency

**Missing · Core requirement**

Single-thread interpreter has no language tasks, atomics, channels or scheduler.

Evidence · [SPEC](../SPEC.md) · [FUTURE](../docs/FUTURE_DESIGN.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Build structured task ownership and cancellation before unsafe shared-state APIs. |
| Required dependencies | 03,05,12; concurrency memory model |
| Implementation milestones | Specify happens-before, then scoped tasks, channels and a scheduler. |
| Acceptance criteria | Joining a scope accounts for all child work and resources. |
| Correctness tests | Deterministic scheduler tests, cancellation propagation and memory-order litmus tests. |
| Performance tests | Measure handoff latency, contention, fairness and saturation. |
| Security risks | Races, deadlock, priority inversion and queue exhaustion. |
| Maintenance | Runtime owners maintain memory-model and scheduler fixtures. |
| Compatibility | Version synchronization semantics independently of scheduler policy. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Structured concurrency | Core requirement | Missing | Account for all children at scope exit |
| Async/await | Core requirement | Missing | Check suspension points and borrowed lifetimes |
| Futures | Core requirement | Missing | Enforce the documented completion protocol |
| Channels | Standard library capability | Missing | Check bounded send/receive order and closure |
| Work-stealing scheduler | Standard library capability | Missing | Run balanced and skewed task trees |
| Thread pools | Standard library capability | Missing | Drain and join workers on shutdown |
| Actor model libraries | Optional framework | Missing | Isolate actor state across asynchronous sends |
| Atomic operations | Core requirement | Missing | Validate ordering with litmus fixtures |
| Lock-free data structures | Experimental capability | Research required | Check linearizability and reclamation |
| Fine-grained parallelism | Experimental capability | Missing | Measure parallel overhead by grain size |
| Data-race prevention | Core requirement | Planned | Reject overlapping mutable access |
| Deadlock analysis | Long-term research | Research required | Distinguish definite cycles from conservative warnings |
| Cancellation and deadlines | Standard library capability | Missing | Propagate expiry through a task scope |
| Backpressure | Standard library capability | Missing | Prove queue size remains bounded |

## 16. Distributed Computing

**Planned · Optional framework**

Distributed tensors, collectives and failure semantics are design goals. No cluster runtime ships.

Evidence · [FUTURE](../docs/FUTURE_DESIGN.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [ROAD](../ROADMAP.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Keep transport and cluster policy above core; define consistency and recovery contracts. |
| Required dependencies | 15,39,44,46 |
| Implementation milestones | Loopback protocol, two-node collectives, then failure and elastic recovery. |
| Acceptance criteria | Under a documented failure model work completes or reports a bounded error. |
| Correctness tests | Partition, duplicate, reorder and restart simulated peers against a state oracle. |
| Performance tests | Measure latency, scaling efficiency, network volume and recovery time. |
| Security risks | Unauthenticated peers, duplicate effects and split-brain state. |
| Maintenance | Distributed maintainers version protocols and rehearse recovery. |
| Compatibility | Negotiate protocol/data versions and define replay guarantees. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Distributed actors | Optional framework | Missing | Test duplicate delivery and actor restart |
| Remote procedure calls | Optional framework | Missing | Define retries and idempotency |
| Distributed task graphs | Optional framework | Missing | Schedule a DAG with worker failure |
| Cluster scheduling | Optional framework | Missing | Bound resource assignment across workers |
| Distributed tensors | Optional framework | Planned | Validate partition and collective reconstruction |
| Fault-tolerant execution | Optional framework | Planned | Recover or report a failed worker |
| Checkpoint and recovery | Optional framework | Planned | Restore a consistent checkpoint |
| Data locality optimization | Optional framework | Missing | Reduce remote bytes on the same workload |
| Distributed caching | Optional framework | Missing | Invalidate entries after input changes |
| Service discovery | Optional framework | Missing | Expire stale service instances |
| Collective communication | External integration | Planned | Verify reduction across independent workers |
| Elastic resource allocation | Optional framework | Missing | Rebalance after adding or removing a worker |

## 17. CPU Architecture Expansion

**Partially implemented · External integration**

The Rust-hosted CLI is verified on x86-64 Linux. Mongolia-native CPU lowering and ISA-specific optimization have no execution record.

Evidence · [BUILD](../PHASE_8_REPORT.md) · [BENCH](../BENCHMARK_BASELINE.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Track interpreter hosts, codegen targets and ISA extensions independently. |
| Required dependencies | 07,11,47; target runners |
| Implementation milestones | Retain host baseline, add AArch64 runner, then target-feature dispatch. |
| Acceptance criteria | Each host/ISA claim has a successful run with feature detection recorded. |
| Correctness tests | Baseline/optimized comparisons, unavailable instruction-path tests and endian checks. |
| Performance tests | Report per-ISA kernel throughput, startup and binary size. |
| Security risks | Illegal instructions and ABI or alignment mismatches. |
| Maintenance | Target owners maintain hardware inventory and runner availability. |
| Compatibility | Baseline binary must run without optional ISA features. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| x86-64 | External integration | Partially implemented | Retain verified interpreter host and qualify native code separately |
| AArch64 | External integration | Planned | Execute the same corpus on a real or explicitly identified emulated target |
| ARM Cortex-M | External integration | Missing | Boot a constrained test image |
| RISC-V | External integration | Research required | Qualify ABI and vector assumptions |
| PowerPC | External integration | Research required | Test endian and ABI variants |
| WebAssembly execution environments | External integration | Research required | Run an ABI fixture in a named engine |
| Future processor architectures | External integration | Research required | Require published ISA and runner access |
| SSE | External integration | Missing | Dispatch safely across the declared minimum feature set |
| AVX2 | External integration | Planned | Compare vector results with scalar |
| AVX-512 | External integration | Planned | Reject absent ISA subsets |
| AMX | External integration | Research required | Check OS tile-state enablement |
| ARM NEON | External integration | Planned | Verify vector tail behavior |
| SVE | External integration | Research required | Test multiple vector lengths |
| SVE2 | External integration | Research required | Qualify SVE2-specific operations |
| RISC-V Vector Extensions | External integration | Research required | Exercise variable-length loops and scalar fallback |

## 18. GPU Architecture Expansion

**Planned · External integration**

CUDA, Vulkan and WebGPU have separate designs. No GPU kernel or driver call executes in v0.1.

Evidence · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [FUTURE](../docs/FUTURE_DESIGN.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use capability-based lowering with explicit math, layout and fallback rules. |
| Required dependencies | 07,11,13; device SDK and runners |
| Implementation milestones | Single reference operation, backend-specific kernels, then multi-vendor portability. |
| Acceptance criteria | Each claimed target reports device identity and passes its declared numerical tolerance. |
| Correctness tests | Compare CPU reference with each driver and test unsupported capabilities. |
| Performance tests | Measure synchronized kernel, launch and transfer costs separately. |
| Security risks | Driver faults, out-of-bounds device storage and silent precision downgrade. |
| Maintenance | Target owners review supported SDK versions and hardware results. |
| Compatibility | Publish backend-specific feature levels and explicit fallback policy. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| NVIDIA CUDA | External integration | Planned | Run a checked tensor kernel on a named CUDA device |
| Vulkan Compute | External integration | Planned | Validate descriptors and queue synchronization |
| WebGPU | External integration | Planned | Validate WGSL and device limits |
| SPIR-V | External integration | Planned | Validate a compute module before driver submission |
| GPU capability detection | External integration | Planned | Report supported operation and dtype pairs |
| Multi-vendor device abstraction | External integration | Planned | Compare a common fixture across two vendors |
| Portable kernel programming | External integration | Planned | Reject a kernel outside the target capability subset |
| AMD ROCm | External integration | Research required | Evaluate a documented ROCm runtime path |
| Intel GPU | External integration | Research required | Qualify a specific driver/device combination |
| Apple Metal | External integration | Research required | Evaluate Metal resource and synchronization mappings |

## 19. AI Accelerators

**Research required · External integration**

Generic device architecture is proposed. No NPU/TPU/DSP adapter or named device conformance exists.

Evidence · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [ARCH](../ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use documented vendor APIs and declare supported operations/dtypes per adapter. |
| Required dependencies | 07,13,36; vendor SDK |
| Implementation milestones | Select one SDK, enumerate capabilities, compile one graph and verify hardware output. |
| Acceptance criteria | A named physical accelerator completes a reference graph within declared error. |
| Correctness tests | Compare simulator and hardware independently, exercise unsupported operations. |
| Performance tests | Report compilation, transfer, inference latency and power if measurable. |
| Security risks | Untrusted drivers and unsupported precision fallback. |
| Maintenance | Adapter owners track SDK licensing, releases and access to hardware. |
| Compatibility | Version SDK/firmware support separately from language editions. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| NPU | External integration | Research required | Validate one compiled inference subgraph |
| TPU | External integration | Research required | Execute an adapter fixture on an identified device |
| DSP | External integration | Research required | Check quantized reference and transfer cost |
| Dedicated inference accelerators | External integration | Research required | Verify supported operations and fallback |
| Edge AI processors | External integration | Research required | Measure under stated power/memory limits |
| Custom AI hardware | External integration | Research required | Require a documented SDK and conformance runner |

## 20. FPGA and Hardware Description

**Research required · Long-term research**

No HLS, RTL emitter, FPGA simulator or co-design toolchain exists.

Evidence · [ARCH](../ARCHITECTURE.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Define a restricted clocked dataflow subset before attempting synthesis. |
| Required dependencies | 07,11; timing/resource IR and vendor tools |
| Implementation milestones | Specify cycle model, simulate a small kernel, then synthesize for one board. |
| Acceptance criteria | Simulator and board agree on values, latency contract and reset behavior. |
| Correctness tests | Cycle-accurate comparison with golden traces and backpressure cases. |
| Performance tests | Measure initiation interval, clock rate, logic use and compile time. |
| Security risks | Invalid timing assumptions and untrusted bitstreams. |
| Maintenance | Hardware researchers own tool pins and board regressions. |
| Compatibility | Version timing/protocol interfaces and identify each board revision. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| High-level synthesis | Long-term research | Research required | Compare synthesized datapath against software reference |
| FPGA kernels | Long-term research | Research required | Validate a board kernel against cycle simulation |
| Hardware dataflow graphs | Long-term research | Research required | Verify backpressure through a graph |
| Register-transfer-level generation | Long-term research | Research required | Check reset and output timing |
| Hardware/software co-design | Long-term research | Research required | Verify both sides of the transport interface |
| Hardware simulation | Long-term research | Research required | Replay deterministic clock/input traces |
| Verilog and SystemVerilog interoperability | External integration | Research required | Round-trip a module interface through a simulator |

## 21. Embedded and Bare-Metal Computing

**Missing · External integration**

Current binary needs Rust std, a host OS and heap. No freestanding target profile exists.

Evidence · [SPEC](../SPEC.md) · [ARCH](../ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Define a no-host profile with explicit target startup and bounded storage. |
| Required dependencies | 07,12,17; target ABI and linker |
| Implementation milestones | Target description, emulator smoke test, then real board acceptance. |
| Acceptance criteria | A declared freestanding program boots and completes without host services. |
| Correctness tests | Inspect imports, test interrupt return, memory layout and failure handling in emulation. |
| Performance tests | Measure image size, stack high-water, boot time and interrupt latency. |
| Security risks | Unsafe MMIO, interrupt races and invalid linker layouts. |
| Maintenance | BSP maintainers pin emulator/board versions and memory maps. |
| Compatibility | Board support names ABI, required runtime and unsupported features. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Freestanding compilation | External integration | Missing | Boot without hosted process services |
| No standard-library execution | External integration | Missing | Inspect target imports for hidden std dependencies |
| No heap-required execution | External integration | Missing | Run without an allocator |
| Interrupt handlers | External integration | Missing | Verify save/restore and nested interrupts |
| Memory-mapped I/O | External integration | Missing | Validate alignment and register width |
| Volatile memory operations | External integration | Missing | Inspect required emitted accesses |
| Hardware abstraction layers | External integration | Missing | Run one driver fixture with two HALs |
| Inline assembly | External integration | Missing | Check clobbers and calling convention |
| Linker scripts | External integration | Missing | Verify section placement against a memory map |
| Bootloaders | External integration | Missing | Validate transfer to the application entry |
| Embedded device debugging | External integration | Missing | Resolve a source location on hardware |
| Real-time operating system integration | External integration | Missing | Verify task lifecycle against one RTOS |

## 22. Operating System Development

**Missing · Optional framework**

No privileged target, syscall ABI, unsafe language blocks or kernel support exists.

Evidence · [SPEC](../SPEC.md) · [ARCH](../ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Isolate hardware-specific unsafe layers from portable kernel services. |
| Required dependencies | 21,12,15; architecture contracts |
| Implementation milestones | Freestanding boot, controlled exception handling, then isolated service prototypes. |
| Acceptance criteria | A minimal kernel boots in a named emulator and enforces one declared isolation invariant. |
| Correctness tests | Simulate page faults, resource depletion and privilege violations. |
| Performance tests | Measure boot, context-switch and syscall latency with a fixed image. |
| Security risks | Privilege escalation, memory isolation and driver faults. |
| Maintenance | Systems maintainers review every unsafe boundary and emulator regression. |
| Compatibility | Version boot and system-call ABIs explicitly. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Kernels | Optional framework | Missing | Boot and isolate one user task |
| Device drivers | Optional framework | Missing | Handle device removal cleanly |
| Filesystems | Optional framework | Missing | Replay a journal and verify file contents |
| Network stacks | Optional framework | Missing | Round-trip a protocol conformance packet |
| Memory managers | Optional framework | Missing | Detect invalid or conflicting mappings |
| Process schedulers | Optional framework | Missing | Check runnable task progress and accounting |
| System utilities | Optional framework | Missing | Exercise a documented OS interface |
| Hypervisors | Optional framework | Missing | Verify guest state isolation in a test harness |
| Virtual machines | Optional framework | Missing | Boot a constrained guest fixture |
| Container runtimes | Optional framework | Missing | Verify declared resource and filesystem isolation |

## 23. Full-Stack Web Development

**Missing · Optional framework**

Wasm is a feasibility target. No browser bindings, web server or framework exists.

Evidence · [FUTURE](../docs/FUTURE_DESIGN.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Keep protocol stacks, browser bindings and UI packages outside core. |
| Required dependencies | 39,43,44; optional 15 and Wasm |
| Implementation milestones | Wasm/JS boundary, request/response fixture, then higher-level framework. |
| Acceptance criteria | A versioned client/server example operates in a real supported browser/runtime. |
| Correctness tests | Protocol interop, disconnects, Unicode payloads and cancellation. |
| Performance tests | Measure cold load, request latency, concurrency and asset size. |
| Security risks | Untrusted network data, origin and permission boundaries. |
| Maintenance | Web maintainers track browser and protocol versions. |
| Compatibility | Negotiate protocol features and publish supported browser versions. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| WebAssembly compilation | External integration | Research required | Run a compiled module in an identified browser engine |
| JavaScript interoperability | External integration | Missing | Round-trip a typed value and callback |
| Browser APIs | External integration | Missing | Handle an unavailable capability |
| DOM integration | External integration | Missing | Update and release a DOM node |
| HTTP clients and servers | Standard library capability | Missing | Round-trip bounded requests and responses |
| WebSocket | Standard library capability | Missing | Check close handshake and message bounds |
| HTTP/2 | External integration | Missing | Verify stream multiplexing and cancellation |
| HTTP/3 | External integration | Missing | Validate version negotiation |
| QUIC | External integration | Missing | Exercise loss recovery on a bounded test transport |
| Server-side rendering | Optional framework | Missing | Compare serialized markup with a fixture |
| Reactive UI | Optional framework | Missing | Propagate edits without stale DOM state |
| WebGPU applications | Optional framework | Missing | Render or compute through a browser device |
| Edge computing | Optional framework | Missing | Handle runtime resource limits |
| Full-stack frameworks | Optional framework | Missing | Run a versioned client/server example |

## 24. Mobile Development

**Missing · External integration**

No Android/iOS target integration, SDK bindings or signed app packaging exists.

Evidence · [ARCH](../ARCHITECTURE.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use platform-owned lifecycle, UI and permission APIs through explicit bridges. |
| Required dependencies | 17,39,43; platform SDK/toolchains |
| Implementation milestones | Package an app shell, bind one safe callback, then test device lifecycle. |
| Acceptance criteria | A runnable app on each claimed OS survives suspend/resume and denied permission. |
| Correctness tests | Run simulator and device lifecycle tests, callback lifetime checks. |
| Performance tests | Measure launch time, package size, frame pacing and energy. |
| Security risks | Sensitive sensors, permission handling and callback lifetime. |
| Maintenance | Mobile teams track OS/SDK releases and signing requirements. |
| Compatibility | Publish minimum OS and versioned ABI/binding rules. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Android NDK interoperability | External integration | Missing | Call a native function through the NDK |
| JNI | External integration | Missing | Manage local/global references across callbacks |
| Android SDK bindings | External integration | Missing | Exercise activity lifecycle |
| iOS SDK bindings | External integration | Missing | Exercise a simulator lifecycle |
| Swift interoperability | External integration | Missing | Round-trip an owned object |
| Objective-C interoperability | External integration | Missing | Verify retained object lifetime |
| Native mobile UI | External integration | Missing | Render an accessible native control |
| Touch and gesture processing | External integration | Missing | Replay ordered gesture input |
| Sensor APIs | External integration | Missing | Handle missing permission or hardware |
| Mobile GPU acceleration | External integration | Missing | Validate device result against CPU |
| App packaging | External integration | Missing | Install a correctly signed test package |
| Platform-specific permission handling | External integration | Missing | Reject denied sensor access |

## 25. Desktop Development

**Missing · Optional framework**

The CLI runs on Linux and CI describes other hosts. GUI/application integration is absent.

Evidence · [BUILD](../PHASE_8_REPORT.md) · [CI](../.github/workflows/ci.yml)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Prefer thin windowing and accessibility adapters with an event-driven portable layer. |
| Required dependencies | 39,43; OS UI APIs |
| Implementation milestones | Create window/event loop, add an accessible control, then package per platform. |
| Acceptance criteria | A keyboard- and assistive-technology-accessible application runs on each claimed host. |
| Correctness tests | Test focus, Unicode input, clipboard permissions and close/reopen. |
| Performance tests | Measure cold launch, idle RSS and event-to-frame latency. |
| Security risks | Clipboard exposure, unsafe callbacks and untrusted media. |
| Maintenance | UI maintainers own platform matrix and accessibility regressions. |
| Compatibility | Version OS-specific features and preserve documented event order. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Windows | External integration | Missing | Run the CLI corpus on Windows and independently qualify a GUI stack |
| macOS | External integration | Missing | Run the CLI corpus on macOS and independently qualify a GUI stack |
| Linux | External integration | Partially implemented | Qualify the existing Linux CLI separately from window-system APIs |
| Native windowing | External integration | Missing | Open a window and close without leaked handles |
| Cross-platform GUI | Optional framework | Missing | Run the same widget interaction on two hosts |
| Accessibility support | External integration | Missing | Navigate a control with assistive technology |
| Native notifications | External integration | Missing | Handle an unavailable notification permission |
| Clipboard APIs | External integration | Missing | Round-trip authorized Unicode content |
| File dialogs | External integration | Missing | Handle cancel and Unicode paths |
| Audio and video interfaces | External integration | Missing | Validate playback timing and device-loss handling |
| Installer generation | External integration | Missing | Install and remove a signed test build |

## 26. Game Development

**Missing · Optional framework**

No game engine, graphics renderer, asset pipeline or game loop ships.

Evidence · [STATUS](../IMPLEMENTATION_STATUS.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Build ECS, physics and rendering as libraries with explicit frame ownership. |
| Required dependencies | 15,18,25,39; graphics/audio SDKs |
| Implementation milestones | Deterministic headless update, rendering adapter, then asset pipeline. |
| Acceptance criteria | A small scene replays its simulation deterministically on declared targets. |
| Correctness tests | Golden scene state, asset round trips and render tolerance tests. |
| Performance tests | Frame-time percentiles, simulation throughput and memory per scene. |
| Security risks | Untrusted assets, driver inputs and network state. |
| Maintenance | Engine maintainers version scene/asset schemas and runner scenes. |
| Compatibility | Keep simulation tick and serialized assets versioned independently. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| 2D game engines | Optional framework | Missing | Replay a sprite scene |
| 3D game engines | Optional framework | Missing | Compare a reference 3D scene |
| Entity Component Systems | Optional framework | Missing | Check component query and deletion order |
| Physics engines | Optional framework | Missing | Validate a deterministic simulated trajectory |
| Collision detection | Optional framework | Missing | Check separated, touching and overlapping shapes |
| Animation systems | Optional framework | Missing | Compare keyed poses at known times |
| Shader programming | Optional framework | Missing | Compile a typed shader with a known reference output |
| Audio engines | Optional framework | Missing | Render a known signal |
| Multiplayer networking | Optional framework | Missing | Replay delayed and duplicate messages |
| Asset pipelines | Optional framework | Missing | Reject corrupt assets with bounded memory |
| AI-driven game characters | Optional framework | Missing | Reproduce behavior with a fixed seed |
| Procedural generation | Optional framework | Missing | Reproduce generated assets from seed and version |
| VR and AR | External integration | Missing | Record frame pacing on named hardware |
| Ray tracing | External integration | Missing | Compare a reference image within tolerance |
| GPU-based simulation | External integration | Missing | Compare device and reference simulation states |

## 27. Cloud and Enterprise Development

**Missing · Optional framework**

No deployable services, authentication middleware or cloud integration in the package.

Evidence · [STATUS](../IMPLEMENTATION_STATUS.md) · [FUTURE](../docs/FUTURE_DESIGN.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use standard protocols and reusable service libraries with explicit authority. |
| Required dependencies | 15,16,39,44,46 |
| Implementation milestones | Minimal local service, packaging, then authenticated multi-instance deployment. |
| Acceptance criteria | Document deployment, health checks and behavior on dependency failure. |
| Correctness tests | Contract, retry/idempotence and restart tests with isolated credentials. |
| Performance tests | Measure p99 request latency, throughput and recovery time under load. |
| Security risks | Credential exposure and authorization errors. |
| Maintenance | Service maintainers own protocol updates and incident playbooks. |
| Compatibility | Version public service schemas and deployment configuration. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Microservices | Optional framework | Missing | Deploy two independently versioned services |
| REST APIs | Optional framework | Missing | Check schema and status contract |
| gRPC | External integration | Missing | Round-trip a generated service method |
| Serverless computing | External integration | Missing | Run a cold and warm invocation |
| Container deployment | External integration | Missing | Build and run a minimal image |
| Kubernetes integration | External integration | Missing | Exercise readiness, graceful termination and restart |
| Distributed tracing | Standard library capability | Missing | Propagate a trace across a service boundary |
| Configuration management | Standard library capability | Missing | Reject invalid configuration atomically |
| Authentication | External integration | Missing | Reject invalid identity and expired credentials |
| Authorization | Optional framework | Missing | Prove access denial under insufficient grants |
| Enterprise messaging | Optional framework | Missing | Check redelivery semantics |
| Event streaming | Optional framework | Missing | Resume from a committed offset |
| High-availability services | Optional framework | Missing | Fail over without losing committed state |
| Service supervision | Optional framework | Missing | Restart a failing worker with bounded retries |

## 28. Database and Data Engineering

**Missing · Optional framework**

C/SQLite interoperability is a design direction. No SQL adapter, storage engine or Arrow/Parquet implementation exists.

Evidence · [FUTURE](../docs/FUTURE_DESIGN.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use external databases/formats before building a storage engine; specify lifetime and transaction boundaries. |
| Required dependencies | 12,39; storage and format specs |
| Implementation milestones | Read-only adapter, transactional writes, then typed columnar exchange. |
| Acceptance criteria | Return consistent query results after restart and schema negotiation. |
| Correctness tests | Reference SQL results, crash/recovery and schema/dtype round trips. |
| Performance tests | Report query latency, scan throughput, storage size and copy count. |
| Security risks | Corrupt files, unsafe buffer ownership and lost commits. |
| Maintenance | Data maintainers version adapters and durability tests. |
| Compatibility | Document isolation levels and backward-readable file schemas. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Embedded databases | External integration | Planned | Round-trip an embedded SQLite transaction |
| SQL interoperability | External integration | Planned | Bind typed values through a checked wrapper |
| Query optimization | Optional framework | Missing | Compare rewritten versus original query results |
| Transaction processing | Optional framework | Missing | Verify commit/rollback |
| ACID semantics | Optional framework | Missing | Reopen after simulated interruption |
| Columnar storage | Optional framework | Missing | Round-trip typed columns with null values |
| Vector databases | Optional framework | Missing | Compare search quality against exact neighbors |
| Graph databases | Optional framework | Missing | Verify traversal against a reference graph |
| Time-series databases | Optional framework | Missing | Check window boundaries and timestamp units |
| Streaming data processing | Optional framework | Missing | Verify checkpoint/resume ordering |
| Apache Arrow interoperability | External integration | Missing | Round-trip an array with offsets and nulls |
| Parquet interoperability | External integration | Missing | Read an independent writer's typed file |
| Zero-copy data exchange | External integration | Planned | Prove buffer lifetime and layout across the adapter |
| Large-scale analytics | Optional framework | Missing | Reproduce an aggregate across partitions |

## 29. Scientific and Mathematical Computing

**Partially implemented · Standard library capability**

Dense f64 matmul and elementary arithmetic pass exact small-matrix checks. Advanced algorithms and precision families are open.

Evidence · [TENSOR](../crates/mn-core/src/engine/tensor.rs) · [ORACLE](../verification/phase8/command-10.log) · [BENCH](../BENCHMARK_BASELINE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Specify dtype, conditioning, tolerance, reproducibility and error bounds per routine. |
| Required dependencies | 03,36,39,47 |
| Implementation milestones | Validate dense primitives, wrap a trusted numerical library, then add domain solvers. |
| Acceptance criteria | Each routine has a reference, an accuracy envelope and failure-domain specification. |
| Correctness tests | Check analytic solutions, invariants, ill-conditioned cases and independent references. |
| Performance tests | Report accuracy beside throughput over diverse shapes and problem sizes. |
| Security risks | Overflow, invalid domains and unbounded workspace. |
| Maintenance | Numerical owners maintain references, tolerance rationales and datasets. |
| Compatibility | Version numerical policy, RNG streams and reproducibility guarantees. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Advanced linear algebra | Standard library capability | Partially implemented | Retain checked dense matmul and independently validate new decompositions |
| Symbolic mathematics | Standard library capability | Missing | Verify substitution preserves meaning |
| Numerical integration | Standard library capability | Missing | Measure convergence on analytic integrals |
| Optimization algorithms | Standard library capability | Missing | Verify a known minimizer and stopping conditions |
| Differential equation solvers | Standard library capability | Missing | Compare against closed-form trajectories |
| FFT | Standard library capability | Missing | Check forward/inverse round trips |
| Sparse matrices | Standard library capability | Missing | Compare sparse products with dense |
| Statistical computing | Standard library capability | Missing | Check estimator bias on fixed datasets |
| Monte Carlo simulation | Standard library capability | Missing | Reproduce seeded estimates with uncertainty bounds |
| Computational fluid dynamics | Optional framework | Missing | Validate a standard conservation fixture |
| Molecular simulation | Optional framework | Missing | Compare a simple reference trajectory |
| Geospatial computation | Optional framework | Missing | Validate coordinate transforms |
| Arbitrary-precision arithmetic | Standard library capability | Missing | Compare large exact integers with a trusted implementation |
| Interval arithmetic | Standard library capability | Missing | Prove sampled results remain within returned bounds |
| Reproducible numerical computation | Standard library capability | Partially implemented | Retain exact-dyadic matrix and strict-f64 reference results |

## 30. Financial Computing

**Missing · Optional framework**

i64/f64 exist. Decimal, fixed-point and financial domain packages are absent.

Evidence · [SPEC](../SPEC.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Use explicit scale and rounding contexts plus auditable domain calculations. |
| Required dependencies | 03,05,29; decimal/rounding contract |
| Implementation milestones | Define decimal fixtures, add a checked arithmetic package, then model/reporting adapters. |
| Acceptance criteria | Replay a published fixture with exact decimal totals and rounding trace. |
| Correctness tests | Tie-breaking, cancellation, scale overflow and ledger conservation cases. |
| Performance tests | Measure throughput and tail latency at fixed precision and audit settings. |
| Security risks | Silent rounding drift and exposure of transaction data. |
| Maintenance | Domain owners review numeric conventions and model versions. |
| Compatibility | Persist scale, currency and rounding policy with serialized values. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Fixed-point arithmetic | Standard library capability | Missing | Check scale-preserving addition and checked multiplication |
| Decimal arithmetic | Standard library capability | Missing | Validate ties and decimal rounding modes |
| Risk modeling | Optional framework | Missing | Compare a model against a published synthetic fixture |
| High-throughput transaction processing | Optional framework | Missing | Reconcile committed totals under retries |
| Fraud detection | Optional framework | Missing | Measure precision/recall on labeled synthetic data |
| Financial simulations | Optional framework | Missing | Replay a versioned seeded simulation |
| Event-driven market data processing | Optional framework | Missing | Handle duplicated and reordered events |
| Auditable calculation pipelines | Optional framework | Missing | Recompute a result from its inputs and policy version |

## 31. Blockchain and Cryptographic Computing

**Partially implemented · Optional framework**

Interpreter work/output budgets and deterministic reference operations exist. No consensus VM, smart-contract compiler or cryptographic package.

Evidence · [SPEC](../SPEC.md) · [RUN](../crates/mn-core/src/engine/runtime.rs) · [TEST](../verification/phase8/command-03.log)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Separate consensus determinism from ordinary host execution and use audited library providers. |
| Required dependencies | 39,44,45; audited crypto libraries |
| Implementation milestones | Define deterministic subset/fuel, then validate external hashes/signatures and versioned contract execution. |
| Acceptance criteria | A reference execution trace agrees across two supported implementations and repeated replay. |
| Correctness tests | Known-answer vectors, invalid signatures, metering boundaries and state replay. |
| Performance tests | Measure verification cost, execution per fuel unit and state growth. |
| Security risks | Key handling, replay and divergent consensus. |
| Maintenance | Security/domain maintainers review provider updates and protocol forks. |
| Compatibility | Freeze consensus semantics per chain version; current fuel is a foundation policy. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Cryptographic primitives through audited libraries | External integration | Missing | Check provider identity and known-answer vectors |
| Digital signatures | External integration | Missing | Verify and reject corrupted signatures |
| Hashing | External integration | Missing | Compare digests against published vectors |
| Smart-contract compilation | Experimental capability | Missing | Compare metered contract traces across implementations |
| Deterministic execution | Core requirement | Partially implemented | Define a consensus-compatible subset and compare traces across hosts |
| Resource-metered execution | Core requirement | Partially implemented | Verify fuel exhaustion at the documented boundary |
| Blockchain interoperability | External integration | Missing | Round-trip one supported chain format |
| Zero-knowledge proof libraries | External integration | Research required | Verify independent proof vectors |
| Verifiable computation | Long-term research | Research required | Specify the verification property and trusted assumptions |

## 32. Robotics and Real-Time Systems

**Missing · External integration**

Single-host measurements give no deadline guarantee. ROS 2, controls and real-time runtime are absent.

Evidence · [BENCH](../BENCHMARK_BASELINE.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Keep control loops bounded and specify failure states and timing assumptions per target. |
| Required dependencies | 14,15,21,39; middleware and target timing |
| Implementation milestones | Simulation integration, deterministic replay, then supervised hardware timing trials. |
| Acceptance criteria | A benign simulator fixture meets documented timing and safe-stop behavior. |
| Correctness tests | Replay timestamped sensors, delayed messages, missed deadlines and loss of input. |
| Performance tests | Measure jitter, worst observed latency and deadline misses under stated load. |
| Security risks | Unsafe actuation and stale sensor data. |
| Maintenance | Domain owners maintain simulator scenarios and independent safety review. |
| Compatibility | Define timestamp units, message schema and hardware-specific limits. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Robotics middleware interoperability | External integration | Missing | Round-trip timestamped simulated sensor messages |
| ROS 2 bindings | External integration | Missing | Validate one versioned message type and QoS profile |
| Sensor fusion | Optional framework | Missing | Compare an estimate against simulated ground truth |
| Motion planning | Optional framework | Missing | Validate collision-free paths in a benign simulator |
| Computer vision | Optional framework | Missing | Compare detections against a pinned synthetic scene |
| Control systems | Optional framework | Missing | Check a bounded simulated control response |
| Real-time scheduling | Long-term research | Research required | Measure deadline response under an explicit scheduling model |
| Industrial communication protocols | External integration | Missing | Validate a documented protocol on a simulator |
| Drone software infrastructure | Optional framework | Missing | Replay benign non-actuating telemetry with timestamps |
| Safety-oriented execution | Long-term research | Research required | Validate safe-stop behavior during input failure |
| Predictable latency | Long-term research | Research required | Report jitter and missed deadlines under specified load |

## 33. Quantum Computing Interfaces

**Research required · Long-term research**

No circuit representation or quantum provider interface exists.

Evidence · [ARCH](../ARCHITECTURE.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Distinguish simulator results, classical control and actual provider jobs in result metadata. |
| Required dependencies | 39; circuit model and external provider |
| Implementation milestones | Specify a circuit subset, round-trip an interchange format, then submit optional provider jobs. |
| Acceptance criteria | A reference circuit round-trips and produces the expected simulator distribution. |
| Correctness tests | Analytic small circuits, unsupported gates and probabilistic acceptance bounds. |
| Performance tests | Measure translation, queue and execution time separately by execution mode. |
| Security risks | Provider credentials, quotas and false hardware attribution. |
| Maintenance | Integration owners pin format and provider versions. |
| Compatibility | Declare supported gates, precision, shot count and model/target identity. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Quantum circuit representation | Long-term research | Research required | Serialize and reconstruct gate order |
| Classical-quantum hybrid algorithms | Long-term research | Research required | Validate classical control over a simulated circuit |
| Quantum simulator integration | External integration | Research required | Compare a small analytic circuit distribution |
| OpenQASM interoperability | External integration | Research required | Round-trip a supported circuit subset |
| QIR interoperability | External integration | Research required | Validate a circuit under a specified QIR profile |
| External quantum hardware APIs | External integration | Research required | Record provider, device, shots and job state |

## 34. Native AI Super Engine

**Partially implemented · Core requirement**

Eager CPU tensor literals and checked rank-2 operations execute. Lazy graphs, AD, graph optimization and devices remain proposed.

Evidence · [TENSOR](../crates/mn-core/src/engine/tensor.rs) · [RUN](../crates/mn-core/src/engine/runtime.rs) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Keep eager semantics as oracle; validate shape, dtype, device and effects before graph rewrites. |
| Required dependencies | 07,11,13,36,37 |
| Implementation milestones | Typed tensor IR, validated graph capture, then optimized execution per target. |
| Acceptance criteria | Captured and eager paths agree on output, failure order and declared numerical tolerance. |
| Correctness tests | Static/dynamic shape mismatch, alias/effect boundaries and gradient references. |
| Performance tests | Measure kernel, compile, allocation and end-to-end training separately. |
| Security risks | Unbounded graphs and invalid specialization or device access. |
| Maintenance | Tensor team maintains op semantics, derivative registry and oracle corpus. |
| Compatibility | Version operators, shape guards and math modes. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Lazy tensor execution | Experimental capability | Planned | Compare materialized outputs with eager reference |
| Eager tensor execution | Core requirement | Implemented and verified | Evaluate a checked CPU f64 tensor expression |
| Graph capture | Experimental capability | Planned | Preserve shape and execution order in a captured graph |
| Dynamic shape specialization | Experimental capability | Planned | Invalidate specialization when a guard fails |
| Automatic operator selection | Experimental capability | Planned | Match the selected operator to strict reference output |
| Operator scheduling | Experimental capability | Planned | Honor producer and effect dependencies |
| Graph-level optimization | Experimental capability | Planned | Verify equivalence before and after graph rewrites |
| Memory-efficient attention | Long-term research | Research required | Measure peak memory and numerical error on the same attention inputs |
| Sparse attention | Long-term research | Research required | Compare an allowed sparse mask with a dense masked reference |
| Custom kernel programming | Experimental capability | Planned | Verify declared shapes, effects and output for a registered kernel |
| Fused training operators | Experimental capability | Planned | Compare forward and gradient outputs with unfused reference |
| Quantization-aware training | Experimental capability | Missing | Record gradient/accuracy effects under a fixed quantizer |
| Mixture-of-experts execution | Optional framework | Missing | Verify routing and aggregation against a small reference |
| Automatic device placement | Experimental capability | Research required | Include transfer cost and enforce device compatibility |

## 35. Large Language Model Infrastructure

**Planned · Optional framework**

Transformer-related APIs, KV caches, batching and interoperability are design targets. No model training or serving stack runs.

Evidence · [FUTURE](../docs/FUTURE_DESIGN.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Keep serving and model architecture above typed tensor primitives. |
| Required dependencies | 16,34,36,39; tokenizer and model formats |
| Implementation milestones | Small reference forward pass, tokenizer/weights interop, then batching and serving. |
| Acceptance criteria | A pinned small model matches reference tokens/logits within a published tolerance. |
| Correctness tests | Compare full versus cached decode, padding, quantization and cancellation. |
| Performance tests | Report prefill, time-to-first-token, decode throughput and KV memory. |
| Security risks | Model provenance, context isolation and cache leakage. |
| Maintenance | Model runtime owners pin checkpoints, tokenizer versions and tolerances. |
| Compatibility | Version model configuration, tokenizer and cache layout. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Transformer architecture | Optional framework | Planned | Check logits against a pinned tiny model |
| Tokenization | External integration | Planned | Round-trip reference IDs and offsets for multilingual input |
| RoPE | Optional framework | Missing | Compare rotated query/key vectors with analytic values |
| Grouped-query attention | Optional framework | Missing | Compare shared-key group mapping with a dense implementation |
| KV-cache management | Optional framework | Planned | Compare cached and uncached next-token outputs |
| Paged attention | Optional framework | Missing | Compare paged and contiguous KV layouts |
| Continuous batching | Optional framework | Missing | Preserve per-request outputs across batch changes |
| Speculative decoding | Optional framework | Missing | Validate the declared output-distribution contract |
| Prefix caching | Optional framework | Missing | Reject invalid prefix reuse |
| Model sharding | Optional framework | Planned | Reassemble shards without loss |
| Tensor parallelism | Optional framework | Planned | Compare partitioned and single-device outputs |
| Expert parallelism | Optional framework | Missing | Verify routed tokens return to the correct sequence |
| LoRA | Optional framework | Missing | Compare adapted and merged weights |
| QLoRA | Optional framework | Missing | Record quantization error and training memory |
| Parameter-efficient fine-tuning | Optional framework | Missing | Verify only selected parameter groups update |
| Streaming inference | Optional framework | Missing | Preserve token order and cancellation |
| Model serving | Optional framework | Planned | Validate bounded requests and model/version routing |
| Distributed inference | Optional framework | Planned | Recover or report a lost worker under a declared policy |

## 36. Advanced Numerical Precision

**Partially implemented · Core requirement**

f64 arithmetic/tensors are verified. f32 is reserved; other low-precision formats, scaling and calibration have no executable support.

Evidence · [SPEC](../SPEC.md) · [RUN](../crates/mn-core/src/engine/runtime.rs) · [TEST](../verification/phase8/command-03.log)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Specify storage, compute, accumulator and rounding policy independently for each operation. |
| Required dependencies | 03,07,11,47; backend dtype capabilities |
| Implementation milestones | Add scalar format and conversion rules, then accumulation, then hardware-specific modes. |
| Acceptance criteria | Each dtype/mode states representable cases, exceptional inputs and reference error bounds. |
| Correctness tests | Compare edge values, saturation, NaNs, rounding ties and overflow to independent references. |
| Performance tests | Record throughput, error and memory at matched operation and shape. |
| Security risks | Silent narrowing, nondeterministic rounding and unstable training. |
| Maintenance | Numerical maintainers review dtype tables and calibration fixtures. |
| Compatibility | Require explicit precision changes and publish backend-specific support. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| FP64 | Core requirement | Implemented and verified | Retain strict f64 scalar/tensor tests |
| FP32 | Core requirement | Planned | Specify and test f32 rounding and literal conversion |
| TF32 where supported | External integration | Research required | Require explicit math mode and check error bounds |
| BF16 | Core requirement | Planned | Verify conversions against a high-precision reference |
| FP16 | Core requirement | Planned | Check subnormal and overflow behavior |
| FP8 | External integration | Research required | Qualify a specific format and accumulation policy |
| FP4 where supported | External integration | Research required | Validate exact encoding and device capability |
| INT8 | Experimental capability | Planned | Verify quantized ranges and saturation |
| INT4 | Experimental capability | Research required | Check packing, zero points and accumulation bounds |
| Mixed-precision accumulation | Core requirement | Planned | Compare products under declared accumulator dtype |
| Dynamic loss scaling | Experimental capability | Planned | Detect overflow and restore finite gradients |
| Quantization calibration | Experimental capability | Planned | Reproduce scales from a versioned dataset |
| Stochastic rounding | Long-term research | Research required | Check seeded reproducibility and unbiased expectation |
| Precision-aware optimization | Experimental capability | Planned | Validate accuracy limits before timing transformed operations |

## 37. Native Dataflow and Graph Computing

**Planned · Experimental capability**

Proposed tensor SSA supplies a graph design. No general dataflow graph runtime or serializer exists.

Evidence · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [FUTURE](../docs/FUTURE_DESIGN.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Separate pure dataflow from stateful nodes and define incremental invalidation rules. |
| Required dependencies | 03,11,15; dependency and effect IR |
| Implementation milestones | Static DAG verifier, demand evaluation, then streams/partitions. |
| Acceptance criteria | Recomputed and incrementally updated graphs produce equal outputs on the same inputs. |
| Correctness tests | Cycles, invalidation after edits, reordered pure nodes and bounded stream queues. |
| Performance tests | Measure graph build, incremental recomputation, peak memory and scheduler overhead. |
| Security risks | Cycles, stale dependencies and untrusted serialized graphs. |
| Maintenance | Graph owners maintain schema, scheduler and invalidation tests. |
| Compatibility | Version graph format, node semantics and stateful scheduling contracts. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Static computation graphs | Experimental capability | Planned | Verify a typed DAG before execution |
| Dynamic computation graphs | Experimental capability | Missing | Validate nodes and shapes as topology changes |
| Streaming graphs | Experimental capability | Missing | Check event ordering and bounded buffering |
| Demand-driven execution | Experimental capability | Missing | Verify unused pure nodes stay unevaluated |
| Incremental graph updates | Experimental capability | Missing | Compare recomputation with a fresh graph |
| Reactive computation | Experimental capability | Missing | Check update propagation and cycle handling |
| Graph partitioning | Experimental capability | Missing | Preserve dependency edges across partitions |
| Graph scheduling | Experimental capability | Planned | Keep effects and dependency order |
| Automatic dependency analysis | Experimental capability | Planned | Recover edges from typed operands |
| Graph serialization | Experimental capability | Missing | Round-trip versioned graph nodes and attributes |

## 38. AI Agent Infrastructure

**Missing · Optional framework**

No provider interface, agent framework or tool-permission layer exists.

Evidence · [STATUS](../IMPLEMENTATION_STATUS.md) · [FUTURE](../docs/FUTURE_DESIGN.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Build a library with explicit capability grants, durable state and observable tool results. |
| Required dependencies | 35,39,44,46; persistent store |
| Implementation milestones | Mock provider and read-only tools, durable replay, then provider adapters. |
| Acceptance criteria | Actions carry authority and can be audited across cancellation/restart. |
| Correctness tests | Mock provider contracts, denied permissions, replay, stale state and timeout tests. |
| Performance tests | Measure orchestration overhead, queue delay, retrieval latency and task cost. |
| Security risks | Prompt injection, leaked credentials and duplicated external side effects. |
| Maintenance | Framework owners maintain provider schemas, policy tests and trace redaction. |
| Compatibility | Version tool schemas, persisted task state and provider adapters separately. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Model-provider interfaces | External integration | Missing | Mock a provider timeout and schema mismatch |
| Tool execution | Optional framework | Missing | Enforce a per-tool grant |
| Structured function calls | Optional framework | Missing | Reject invalid argument schemas |
| Agent orchestration | Optional framework | Missing | Replay a deterministic synthetic workflow |
| Retrieval-augmented generation | Optional framework | Missing | Trace citations to retrieved documents |
| Embedding pipelines | Optional framework | Missing | Version model and vector dimensions |
| Vector search | Optional framework | Missing | Compare approximate recall with exact retrieval |
| Multi-agent coordination | Optional framework | Missing | Bound handoffs and message queues |
| Persistent task state | Optional framework | Missing | Restore idempotent work after restart |
| Permission-controlled tool access | Optional framework | Missing | Deny unauthorized effects |
| Local and cloud model integration | External integration | Missing | Verify equivalent adapter lifecycle on two provider types |
| Agent observability | Optional framework | Missing | Trace decisions and redact private tool arguments |

## 39. Mongolia Universal Bridge

**Planned · External integration**

C-compatible ABI and ownership rules have designs. No foreign code executes through mn.

Evidence · [FUTURE](../docs/FUTURE_DESIGN.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Start with one explicit C ABI; layer host-runtime-specific handles and adapters above it. |
| Required dependencies | 03,05,12; target ABI and audited wrappers |
| Implementation milestones | Owned/borrowed C buffers, callbacks and error translation, then language bindings by demand. |
| Acceptance criteria | Each supported direction passes values/errors across a real foreign runtime and frees resources correctly. |
| Correctness tests | Cross-runtime round trips, callback lifetimes, alignment, unwind containment and invalid inputs. |
| Performance tests | Measure boundary latency, copying and batch throughput per adapter. |
| Security risks | Dangling foreign pointers, exception escape and mismatched ownership. |
| Maintenance | Interop maintainers own ABI fixtures, runtime matrix and generated bindings. |
| Compatibility | Version ABI and binding APIs independently; document target-specific layouts. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| C | External integration | Planned | Verify repr-C scalar/layout fixtures |
| Rust | External integration | Planned | Round-trip values through explicit C ABI wrappers |
| C++ | External integration | Missing | Prevent exception unwinding across the ABI |
| Python | External integration | Missing | Retain/release Python object handles correctly |
| JavaScript | External integration | Missing | Round-trip JS values and errors |
| TypeScript | External integration | Missing | Check generated declarations against runtime bindings |
| Java | External integration | Missing | Manage JVM object lifetime |
| Kotlin | External integration | Missing | Verify nullable Kotlin types across the adapter |
| C# | External integration | Missing | Validate managed/unmanaged buffer lifetime |
| .NET | External integration | Missing | Test runtime calling convention and callbacks |
| Swift | External integration | Missing | Verify release of transferred objects |
| Objective-C | External integration | Missing | Match retained/autoreleased ownership |
| Go | External integration | Missing | Respect runtime pointer/lifetime rules |
| Julia | External integration | Missing | Verify dtype/layout and GC roots |
| R | External integration | Missing | Root managed objects across callbacks |
| MATLAB | External integration | Missing | Round-trip matrix shape and storage order |
| SQL | External integration | Missing | Bind typed SQL parameters |
| WebAssembly | External integration | Missing | Validate linear-memory bounds and interface types |
| Binding generation | External integration | Missing | Compile generated bindings against a pinned header |
| Foreign object lifetime management | External integration | Planned | Keep borrowed handles alive through callbacks |
| ABI compatibility | External integration | Planned | Compare sizes, alignment and calling conventions |
| Exception translation | External integration | Planned | Return typed errors across a non-unwinding boundary |
| Callback safety | External integration | Planned | Wait for callback completion before dropping context |
| Zero-copy exchange where compatible | External integration | Planned | Check strides, dtype, ownership and lifetime |
| Cross-runtime error handling | External integration | Planned | Preserve source error and destination status |
| Importing external libraries | External integration | Planned | Call a declared foreign symbol |
| Exporting Mongolia libraries into other languages | External integration | Planned | Invoke an exported function from an independent host |

## 40. Legacy Code Migration

**Research required · Long-term research**

No migration tool or semantic translator ships.

Evidence · [STATUS](../IMPLEMENTATION_STATUS.md) · [ARCH](../ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Translate an explicit source subset with diagnostics for unsupported constructs and human review. |
| Required dependencies | 03,11,39; source-language semantics |
| Implementation milestones | Inventory semantics, migrate a restricted fixture, then compare against original execution. |
| Acceptance criteria | Every migrated fixture passes independent differential cases or reports the unsupported boundary. |
| Correctness tests | Ordering, numeric overflow, mutation, resource cleanup and foreign calls. |
| Performance tests | Measure translation time, manual repair effort and runtime change per corpus. |
| Security risks | Semantic drift and unsafe generated boundaries. |
| Maintenance | Migration owners pin source-language versions and minimize mismatch cases. |
| Compatibility | Publish supported subsets and version migration rules. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Python-to-Mongolia migration tools | Long-term research | Research required | Compare a pure supported subset against CPython |
| C-to-Mongolia assisted translation | Long-term research | Research required | Reject unspecified or unsupported source behavior |
| Rust-to-Mongolia assisted migration | Long-term research | Research required | Preserve moves and error semantics on the supported subset |
| Type inference from legacy code | Long-term research | Research required | Report ambiguity with reviewable evidence |
| Automated binding generation | External integration | Research required | Verify generated wrappers against source ABI |
| Semantic compatibility analysis | Long-term research | Research required | Explain mismatches in evaluation order and conversion |
| Differential testing of translated programs | Long-term research | Research required | Run original and translated fixtures against shared inputs |
| AI-assisted code modernization | Long-term research | Research required | Require reviewable diffs and independent semantic tests |

## 41. Advanced Developer Toolchain

**Planned · Optional framework**

run/check/init and diagnostic rendering work. LSP/formatter tooling is designed. No listed debugger, interactive REPL or build cache ships.

Evidence · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [FUTURE](../docs/FUTURE_DESIGN.md) · [CLI](../crates/mn-cli/src/main.rs)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Base tools on shared incremental facts, source maps and versioned debugger/profiler events. |
| Required dependencies | 06,11,43; stable origin maps |
| Implementation milestones | Incremental check and diagnostics, then debugger/profile events and optional frontends. |
| Acceptance criteria | Each tool reflects the current source revision and matches CLI meaning. |
| Correctness tests | Edit/cancel/reload fixtures in English/Mongolian and stale-cache rejection. |
| Performance tests | Measure edit latency, cache hit/miss, profile overhead and interactive response. |
| Security risks | Secrets in traces and stale remote artifacts. |
| Maintenance | DX owners version tool protocols and cache keys. |
| Compatibility | Negotiate capabilities and position encodings with clients. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Incremental builds | Core requirement | Planned | Compare edited and full source snapshots |
| Compilation caching | Experimental capability | Planned | Reject entries from mismatched compiler or configuration |
| Remote build caching | Experimental capability | Missing | Authenticate content and verify digests |
| Hot reload | Experimental capability | Missing | Reload a compatible function and reject incompatible state |
| Hot swapping | Long-term research | Research required | Define migration for live values and in-flight calls |
| Interactive notebooks | Optional framework | Missing | Evaluate ordered cells and invalidate dependants |
| REPL | Optional framework | Planned | Define persistent bindings and recover from an invalid entry |
| Source-level debugging | Optional framework | Planned | Map frames and variables to original UTF-8 spans |
| GPU debugging | External integration | Missing | Identify a failing kernel and recorded source origin |
| Time-travel debugging where practical | Long-term research | Research required | Bound recording and deterministic replay |
| Memory profiling | Optional framework | Missing | Attribute allocation and peak-live storage |
| CPU profiling | Optional framework | Missing | Associate samples with source regions |
| GPU profiling | Optional framework | Missing | Separate transfer, launch and kernel time |
| Performance flame graphs | Optional framework | Missing | Aggregate verified stack samples |
| Dependency visualization | Optional framework | Missing | Render a resolved dependency graph |
| Tensor graph visualization | Optional framework | Missing | Show checked shape/device information per node |

## 42. Mongolia Studio

**Planned · Optional framework**

Existing-editor/LSP integration is designed. No Studio executable or VS Code extension ships.

Evidence · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [FUTURE](../docs/FUTURE_DESIGN.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Deliver editor-independent protocols and existing editor support before a dedicated IDE. |
| Required dependencies | 41; LSP, visualization and optional model provider |
| Implementation milestones | Diagnostics/hover in one editor, tensor inspection, then evaluate standalone demand. |
| Acceptance criteria | A bilingual edit obtains accurate diagnostics/shape from the same compiler API. |
| Correctness tests | Unicode edits, rename, cancellation and workspace reload against CLI results. |
| Performance tests | Measure keystroke-to-diagnostic, large-file memory and trace update cost. |
| Security risks | Untrusted workspaces and code disclosure to remote completion providers. |
| Maintenance | Editor maintainers track protocol/client versions and extension permissions. |
| Compatibility | Feature negotiation keeps older clients usable. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Bilingual code editing | Optional framework | Planned | Edit both spellings against the same semantic diagnostics |
| AI-assisted code completion | Optional framework | Missing | Reject suggestions outside the active edition |
| Compiler integration | Optional framework | Planned | Bind an editor revision to current compiler diagnostics |
| Visual debugging | Optional framework | Missing | Inspect a paused frame with accurate source positions |
| Tensor inspection | Optional framework | Planned | Report a checked tensor shape at the selected expression |
| Neural network visualization | Optional framework | Missing | Render and inspect a typed model graph |
| GPU utilization monitoring | Optional framework | Missing | Display device-specific counters with timestamps |
| Interactive performance analysis | Optional framework | Missing | Link measured events to code |
| Package exploration | Optional framework | Missing | Validate registry provenance before displaying packages |
| Educational programming tools | Optional framework | Missing | Run equivalent bilingual learning exercises |

## 43. Universal Build System

**Partially implemented · Core requirement**

The compiler project has pinned Cargo builds and a checksummed source package. Mongolia has no build graph, linker frontend or package resolver.

Evidence · [CARGO](../Cargo.toml) · [BUILD](../PHASE_8_REPORT.md) · [CI](../.github/workflows/ci.yml)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Keep bootstrap Cargo and the proposed Mongolia build graph distinct; specify all inputs to an artifact. |
| Required dependencies | 06,07,39; manifests and content identity |
| Implementation milestones | Canonical graph/lock, local reproducibility, then multi-target caching and signing. |
| Acceptance criteria | Two clean builds using declared inputs agree to the promised reproducibility level. |
| Correctness tests | Offline/missing-dependency, target-key separation, tampered-cache and rollback cases. |
| Performance tests | Measure clean, incremental and remote-cache costs separately. |
| Security risks | Untrusted build scripts, cache poisoning and leaked signing keys. |
| Maintenance | Release owners maintain locks, SBOM, policy and runner images. |
| Compatibility | Version manifest, cache keys, target triple and artifact schema. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Cross-compilation | Core requirement | Planned | Build for a target different from the host |
| Target triples | Core requirement | Planned | Include target ABI and feature set in artifact identity |
| Build profiles | Core requirement | Partially implemented | Exercise Cargo debug/release for bootstrap and define Mongolia project profiles |
| Native dependency discovery | External integration | Planned | Resolve a pinned system/library ABI or fail |
| Hermetic builds | Experimental capability | Missing | Prove external inputs are declared |
| Reproducible builds | Core requirement | Partially implemented | Rebuild from pinned dependencies and compare promised artifact fields |
| Build caching | Experimental capability | Planned | Invalidate on semantic input, target or toolchain changes |
| Remote build execution | Experimental capability | Missing | Verify attested outputs against declared inputs |
| Multi-language workspaces | Experimental capability | Missing | Build a mixed-language graph with explicit crossing points |
| Platform-specific packaging | External integration | Missing | Install and uninstall on the declared target |
| Static and dynamic linking | External integration | Planned | Exercise loader failure and ABI mismatch |
| Software Bill of Materials | External integration | Missing | Enumerate every build/deployment component with provenance |
| Signed build artifacts | External integration | Planned | Verify release signatures and reject revoked keys |

## 44. Capability-Based Security

**Partially implemented · Core requirement**

Source, tokens, depth, tensor elements, work and output have limits. No permission broker, process wall-clock cap or sandbox exists.

Evidence · [SPEC](../SPEC.md) · [RUN](../crates/mn-core/src/engine/runtime.rs) · [SEC](../SECURITY_AUDIT.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Expose least-authority capabilities and distinguish interpreter budgets from OS-enforced isolation. |
| Required dependencies | 05,12,39; host isolation APIs |
| Implementation milestones | Inventory current effects, define permission manifest, then implement isolation adapters. |
| Acceptance criteria | A denied effect fails predictably; claimed hard quotas are tested at their enforcement boundary. |
| Correctness tests | Boundary values, exhausted budgets, nested calls and permission denial. |
| Performance tests | Measure quota accounting overhead and worst observed enforcement delay. |
| Security risks | Authority escalation, host OOM and leaked device handles. |
| Maintenance | Security maintainers review new effects and refresh denial fixtures. |
| Compatibility | Default-deny new privileged capabilities and version grants. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Sandboxed execution profiles | Core requirement | Planned | Test isolation at OS or VM enforcement boundaries |
| Filesystem permissions | Core requirement | Missing | Deny unauthorized path access |
| Network permissions | Core requirement | Missing | Deny unauthorized connections |
| Device permissions | Core requirement | Missing | Reject undeclared device access |
| FFI permissions | Core requirement | Missing | Refuse foreign calls without a grant |
| Process execution permissions | Core requirement | Missing | Reject launching unapproved child processes |
| Resource quotas | Core requirement | Partially implemented | Test existing source, work, tensor and output limits |
| CPU budgets | Core requirement | Partially implemented | Preserve step cap and separately specify hard CPU-time accounting |
| Memory budgets | Core requirement | Partially implemented | Test per-tensor limits and separately enforce process/allocator budgets |
| GPU budgets | Core requirement | Planned | Limit allocated and in-flight device memory |
| Execution time limits | Core requirement | Missing | Enforce a wall-clock deadline outside the interpreter |
| Secure plugin isolation | Core requirement | Missing | Prevent ungranted plugin access |
| WebAssembly-based sandboxing where appropriate | External integration | Research required | Validate isolation and host import policy in a selected engine |

## 45. High-Assurance Computing

**Partially implemented · Long-term research**

Static typing, checked arithmetic and deterministic reference rules cover a subset. No proof, safety-critical profile or certification exists.

Evidence · [SPEC](../SPEC.md) · [CHECK](../crates/mn-core/src/engine/checker.rs) · [TEST](../verification/phase8/command-03.log) · [SEC](../SECURITY_AUDIT.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Select a restricted, documented profile and make assumptions and assurance evidence explicit. |
| Required dependencies | 03,11,44; formal semantics and trusted base |
| Implementation milestones | Define invariants, translate a small core into a proof model, then validate equivalence. |
| Acceptance criteria | Each claimed property names the model, assumptions, version and independently reproducible evidence. |
| Correctness tests | Model small states, compare executable semantics, and validate runtime invariant failures. |
| Performance tests | Measure verifier and contract overhead on the declared profile. |
| Security risks | Unsound assumptions and reliance on uncertified components. |
| Maintenance | Assurance owners track proof artifacts, tool versions and review scope. |
| Compatibility | Certification is a separate process tied to a specific configuration. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Deterministic execution profiles | Core requirement | Partially implemented | Version an explicit deterministic subset and reference behavior |
| Formal verification interfaces | Long-term research | Research required | Export a machine-checkable semantic model |
| Contract checking | Experimental capability | Missing | Report precise violations at the specified pre/post condition |
| Static analysis | Core requirement | Partially implemented | Retain whole-program type/shape checks and add profile-specific analyses |
| Symbolic execution | Long-term research | Research required | Compare modeled branches with concrete executions |
| Model checking | Long-term research | Research required | Check a bounded-state invariant |
| Memory safety verification | Long-term research | Research required | Document trusted dependencies and verify lifetime rules |
| Concurrency verification | Long-term research | Research required | Check permitted memory-order interleavings |
| Runtime invariant checking | Core requirement | Partially implemented | Verify shape, allocation and work guards |
| Safety-critical coding profiles | Long-term research | Research required | Define independently reviewed rules and target assumptions |
| Audit-friendly compiler output | Core requirement | Partially implemented | Preserve stable diagnostic codes, locations and versioned evidence |

## 46. Observability and Fault Tolerance

**Partially implemented · Standard library capability**

The CLI reports localized runtime diagnostics and buffered output is discarded on failure. No service supervision or tracing pipeline exists.

Evidence · [DIAG](../crates/mn-core/src/diagnostics.rs) · [CLI](../crates/mn-cli/src/main.rs) · [RUN](../crates/mn-core/src/engine/runtime.rs) · [SEC](../SECURITY_AUDIT.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Define structured events and recovery contracts separately for compiler, runtime and services. |
| Required dependencies | 05,15,16,44; event schema |
| Implementation milestones | Typed diagnostics/events, bounded metrics, then supervised restart of a service fixture. |
| Acceptance criteria | Failures identify their origin; retry policy preserves declared side-effect semantics. |
| Correctness tests | Inject timeouts, partial operations, cancellation and restart with duplicate messages. |
| Performance tests | Measure instrumentation cost, latency percentiles and recovery duration. |
| Security risks | Sensitive trace data, retry storms and duplicate side effects. |
| Maintenance | Runtime/SRE owners maintain event schemas, retention and fault scenarios. |
| Compatibility | Version event fields and retry/recovery semantics. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Structured logging | Standard library capability | Missing | Emit versioned events with redaction |
| Metrics | Standard library capability | Missing | Bound label cardinality |
| Distributed tracing | Standard library capability | Missing | Propagate context across service boundaries |
| Health checks | Optional framework | Missing | Separate readiness and liveness |
| Crash reporting | Optional framework | Missing | Preserve actionable metadata without leaking source |
| Runtime diagnostics | Core requirement | Implemented and verified | Verify code, span, exit and output suppression on failures |
| Fault isolation | Optional framework | Missing | Contain a failed worker |
| Supervisory execution | Optional framework | Missing | Restart with a bounded retry strategy |
| Retry policies | Optional framework | Missing | Retry only under a stated idempotency contract |
| Circuit breakers | Optional framework | Missing | Test open, half-open and closed states |
| Graceful degradation | Optional framework | Missing | Report degraded capability explicitly |
| Automatic resource cleanup | Core requirement | Partially implemented | Drop host-owned buffers on normal/error paths and test external-resource cleanup |
| Recovery mechanisms | Optional framework | Missing | Restore committed state after interruption |

## 47. Universal Performance Benchmarking

**Partially implemented · Core requirement**

Linux startup, lexer/parser, checked matmul, RSS and size have raw data, Python/BLAS references and numerical checks. Other domains lack comparable implementations.

Evidence · [BENCH](../BENCHMARK_BASELINE.md) · [RAW](../verification/phase8/benchmark.json) · [ORACLE](../verification/phase8/command-10.log)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Match semantics, accuracy, workload boundaries and resource policy before comparing speeds. |
| Required dependencies | 11; executable features, reference programs and hardware |
| Implementation milestones | Retain host baseline, expand numerical shapes/hosts, then add domain-specific baselines as features execute. |
| Acceptance criteria | Publish runnable commands, hardware, raw samples, accuracy and limitations for each claim. |
| Correctness tests | Verify independent oracle outputs, failure cases and equivalence of work before timing. |
| Performance tests | Retain warm/cold data and tail distributions, include compilation, transfers and synchronization as applicable. |
| Security risks | Cherry-picked workloads, leaked inputs and hidden relaxed math. |
| Maintenance | Performance owners maintain datasets, reference versions and noise policy. |
| Compatibility | Version workloads and show trend discontinuities when methodology changes. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| C, C++, Rust and Zig for systems performance | Experimental capability | Missing | Match allocation, threading and safety settings |
| Go and Rust for networking | Experimental capability | Missing | Compare the same protocol, payload and connection model |
| Python and Julia for scientific workflows | Experimental capability | Partially implemented | Use the same numerical problem and reference tolerance |
| PyTorch and JAX for AI workloads | Experimental capability | Missing | Match dtype, graph warmup and synchronization |
| CUDA and Triton for GPU kernels | Experimental capability | Missing | Compare the same kernel, layout and transfer boundaries |
| BLAS and cuBLAS for matrix operations | Core requirement | Partially implemented | Retain the OpenBLAS baseline and add synchronized cuBLAS when available |
| TensorRT and llama.cpp for inference | Experimental capability | Missing | Match model, quantization and decode policy |
| Erlang for fault-tolerant systems | Experimental capability | Missing | Match delivery and restart semantics |
| Java and .NET for enterprise workloads | Experimental capability | Missing | Match service guarantees, warmup and GC policy |
| JavaScript engines for dynamic execution | Experimental capability | Missing | Match language workload and tier warmup |
| Execution latency | Core requirement | Implemented and verified | Reproduce the checked matmul and CLI elapsed-time samples |
| Throughput | Core requirement | Implemented and verified | Recompute lexer bytes/s and parser tokens/s |
| Tail latency | Core requirement | Partially implemented | Retain p95 and add larger independent runs for rarer quantiles |
| Compilation speed | Core requirement | Partially implemented | Measure check/frontend now and native compile time when available |
| Startup latency | Core requirement | Implemented and verified | Rerun the new-process startup sample |
| Peak RAM | Core requirement | Implemented and verified | Measure child peak RSS on the same fixture |
| Peak VRAM | Experimental capability | Planned | Measure reserved and active bytes at synchronized peaks |
| Energy efficiency | Experimental capability | Missing | Record energy per verified unit of work |
| Binary size | Core requirement | Implemented and verified | Measure the byte length of the audited executable |
| Numerical accuracy | Core requirement | Implemented and verified | Match dyadic matrix products to the exact independent oracle |
| Developer productivity | Experimental capability | Missing | Predefine task, cohort and completion criteria |
| Scaling efficiency | Experimental capability | Missing | Compare validated workloads across core/device counts |

## 48. Independent Mongolia Technology Ecosystem

**Planned · Optional framework**

Library domains have roadmap sketches. None of the nineteen named products is established by this repository.

Evidence · [ROAD](../ROADMAP.md) · [FUTURE](../docs/FUTURE_DESIGN.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Give each viable project a maintainer, package boundary, public API and independent release lifecycle. |
| Required dependencies | 39,43,44; package governance |
| Implementation milestones | Charter one project, demonstrate a minimal consumer, then publish after governance checks. |
| Acceptance criteria | Each named project requires a real artifact, documented API, tests and a responsible maintainer. |
| Correctness tests | Consumer integration, dependency compatibility and clean install checks. |
| Performance tests | Use a domain workload and record the cost of crossing the library boundary. |
| Security risks | Supply-chain ownership, abandoned packages and namespace confusion. |
| Maintenance | Named maintainers own releases, deprecation and advisories per project. |
| Compatibility | Publish semver/support policies and dependency ranges independently. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Mongolia AI | Optional framework | Missing | Publish one model consumer with versioned API |
| Mongolia Tensor | Optional framework | Missing | Run a tensor-package consumer independent of compiler internals |
| Mongolia Compute | Optional framework | Missing | Expose an execution API with a reference backend |
| Mongolia GPU | Optional framework | Missing | Qualify one device package against the CPU oracle |
| Mongolia Web | Optional framework | Missing | Run a browser/server compatibility fixture |
| Mongolia UI | Optional framework | Missing | Demonstrate an accessible widget |
| Mongolia Engine | Optional framework | Missing | Replay an engine scene |
| Mongolia OS | Optional framework | Missing | Boot a versioned system image |
| Mongolia Embedded | Optional framework | Missing | Run a supported board fixture |
| Mongolia Cloud | Optional framework | Missing | Deploy and recover one service |
| Mongolia DB | Optional framework | Missing | Commit and recover a transaction |
| Mongolia Chain | Optional framework | Missing | Replay a deterministic state transition |
| Mongolia Robotics | Optional framework | Missing | Run a sensor replay in simulation |
| Mongolia Quantum | Optional framework | Missing | Report a circuit simulation with execution provenance |
| Mongolia Studio | Optional framework | Missing | Demonstrate shared bilingual compiler diagnostics |
| Mongolia Hub | Optional framework | Missing | Resolve an authenticated package and verify provenance |
| Mongolia LLM | Optional framework | Missing | Compare a pinned model against a reference |
| Mongolia Secure | Optional framework | Missing | Validate a documented least-authority policy |
| Mongolia HPC | Optional framework | Missing | Reproduce a multi-worker numerical result |

## 49. Proprietary-Named Open-Source Core Technologies

**Planned · Experimental capability**

Eight proposed names map to design responsibilities. No named component implementation, ownership or naming clearance is asserted.

Evidence · [ARCH](../ARCHITECTURE.md) · [P7](../docs/PHASE_7_FUTURE_ARCHITECTURE.md) · [STATUS](../IMPLEMENTATION_STATUS.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Adopt a name only after establishing a distinct responsibility, compatibility contract and clearance. |
| Required dependencies | 07,11,13,34,39; naming and governance review |
| Implementation milestones | Map proposals to existing modules, evaluate duplication, then charter approved components. |
| Acceptance criteria | An accepted name has a documented boundary, owner, test surface and verified naming review. |
| Correctness tests | Trace each claimed responsibility to implemented tests before advertising it. |
| Performance tests | Measure the associated component using its real workload after implementation. |
| Security risks | Naming conflicts and artificial fragmentation of trusted boundaries. |
| Maintenance | Governance owns naming decisions; technical owners own resulting interfaces. |
| Compatibility | Keep public APIs independent of provisional internal brands. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| MACE — Mongolia Adaptive Compute Engine | Experimental capability | Planned | Verify scheduling against declared device constraints |
| MUSE — Mongolia Unified Semantic Engine | Core requirement | Planned | Demonstrate equivalent semantics across syntaxes before assigning this name |
| MMIR — Mongolia Multi-Level Intermediate Representation | Core requirement | Planned | Round-trip verified IR with source origins |
| MATO — Mongolia Adaptive Tensor Optimizer | Experimental capability | Planned | Compare fused and specialized plans against eager execution |
| MAM — Mongolia Advanced Memory Manager | Core requirement | Planned | Test host/device allocation and completion lifetimes |
| MAG — Mongolia Automatic Gradient Engine | Core requirement | Planned | Validate a compiler VJP and JVP against independent references |
| MUB — Mongolia Universal Bridge | External integration | Planned | Run bidirectional ABI and ownership fixtures |
| MAO — Mongolia AI-guided Optimization Engine | Long-term research | Research required | Evaluate verified model-selected options on held-out workloads |

## 50. Full Compiler Self-Hosting

**Research required · Long-term research**

Current compiler is Rust. Stage-0/1/2 and a Mongolia-written frontend/middle-end do not exist.

Evidence · [CARGO](../Cargo.toml) · [ROAD](../ROADMAP.md) · [ARCH](../ARCHITECTURE.md)

| Contract | Engineering decision |
|---|---|
| Specification and architecture | Bootstrap progressively through library/tool slices before moving the compiler frontend. |
| Required dependencies | 03,06,11,39,43; language coverage and reproducible toolchain |
| Implementation milestones | Self-host a library subset, port a tool, compile frontend/middle-end, then stage-compare the compiler. |
| Acceptance criteria | A stage compiler builds its successor; independent builds satisfy declared binary or normalized-IR equivalence. |
| Correctness tests | Run conformance on every stage and compare checked artifacts across independent builds. |
| Performance tests | Measure bootstrap wall time, peak RAM and size per stage. |
| Security risks | Compromised bootstrap seeds and trusting-trust risks. |
| Maintenance | Bootstrap team pins seeds and publishes reproducible independent verification. |
| Compatibility | Retain a documented stage-0 path and external ABI/LLVM compatibility where needed. |

| Source item | Engineering class | Audit status | Feature-specific acceptance fixture |
|---|---|---|---|
| Standard library written progressively in Mongolia. | Standard library capability | Planned | Compile one library module through its public API |
| Language formatter written in Mongolia. | Optional framework | Planned | Format the bilingual corpus idempotently |
| Package manager components written in Mongolia. | Optional framework | Planned | Resolve an identical graph with the ported component |
| Compiler frontend written in Mongolia. | Long-term research | Research required | Compare token/AST/error outputs with the Rust baseline |
| Compiler middle-end written in Mongolia. | Long-term research | Research required | Compare verified IR and analysis results |
| Self-hosted compiler capable of compiling its own source code. | Long-term research | Research required | Compile the compiler with its previous stage |
| Reproducible stage-0, stage-1 and stage-2 bootstrap builds. | Long-term research | Research required | Compare independent stage outputs under pinned inputs |
| Independent bootstrap verification. | Long-term research | Research required | Verify stage provenance with an independently produced seed |
| Reduce unnecessary dependencies on the original Rust implementation. | Long-term research | Research required | Document the trusted seed and remaining Rust dependencies |
| Retain justified interoperability with LLVM and external infrastructure. | External integration | Planned | Run interop conformance with a versioned external toolchain |
