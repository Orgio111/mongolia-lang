# Dependency-driven master roadmap

**Scheduling update for v0.1.1.** The executed Phase 9 delivered foundation stabilization. Gate H retains its technical dependencies and moves to consideration at the next milestone. The current handoff is [PHASE_10_READINESS.md](PHASE_10_READINESS.md).

This roadmap proposes qualification windows. v0.1.0 identifies the runnable interpreter. v3.0 identifies this architecture edition today and a possible future language release when its gates pass. Versions after v1.0.0 are planning choices made during integration. No delivery dates or unsupported hardware guarantees follow from the numbering.

## From past phases to Phase 9

| Engineering phase | Exit evidence or acceptance |
|---|---|
| 1 · Architecture | One bilingual pipeline, working crate boundaries, explicit future backend contracts and ADRs in ARCHITECTURE.md and docs/PHASE_1_ARCHITECTURE.md. |
| 2 · Specification | Normative grammar plus A–E and negative-case mapping in docs/PHASE_2_ACCEPTANCE.md. |
| 3 · Repository | Locked, acyclic Rust workspace builds from a clean checkout; dependency graph is in verification/phase8/command-07.log. |
| 4 · Implementation | Five mandatory language programs pass; invalid matrix shapes fail before output. |
| 5 · Verification | Both profiles, fmt, Clippy, exact matrix reference and bounded fuzz runs have command transcripts. |
| 6 · Packaging | Clean extraction, checksum validation, isolated installation and execution of the example corpus. |
| 7 · Future architecture | HIR/AD/device/ABI designs record inputs, failure policy and tests. Python MLP oracle separately establishes a mathematical fixture. |
| 8 · Audit | Reviewed compiler, security, dependency and benchmark evidence; all 50 areas and 620 items classified against the foundation. |
| Expansion integration | Ten named documents; preserved inherited code and audit evidence; complete 50-area priority/gate assignment, acyclic gate graph and validated archive manifest. |
| 9 · Resolved HIR | All gates in PHASE_9_EXECUTION_PLAN.md pass with original AST observables and public CLI/API behavior. This is one increment within v0.2.0. |

## Release train

Owners below are roles. Before starting a shipping workstream, assign a person and runner budget to its role. Original v0.1–v1.0 estimates are preserved in ROADMAP.md. Effort for later versions requires discovery and a funded target matrix.

| Version | Direct prerequisites → output | Measurable exit criteria | Owner / blocking risk |
|---|---|---|---|
| v0.1.0 foundation | A | Locked debug/release builds, bilingual A–E, all required negatives, exact example transcripts and documented host scope. Linux reference gate passed. | Release lead / unverified hosts and public provenance. |
| Phase 9, pre-v0.2 | A → H | Cover every supported expression/statement with private resolved HIR; zero unresolved calls/reads in accepted HIR; canonical bilingual snapshots and all baseline tests pass. | Frontend / error-order and span drift. |
| v0.2.0 language expansion | H → V, L | Verified typed register code for v0.1 corpus; identical outcomes on both engines, module cycle diagnostic, bounded generics fixtures, core primitive API documented. | Compiler / drift in value and budget semantics. |
| v0.3.0 differentiation | V, L → T, D | Add scalar, elementwise and matmul derivative rules, reduction rules and checked training data structures/control flow. Match analytic, central-difference and adjoint cases; reproduce both MLP proposals' 200-step trajectory within a predeclared tolerance. | Numerical team / ill-conditioned references and mutation across AD tapes. |
| v0.4.0 native CPU | T, V → M, C | Verify buffer IR, overflow/alias cases, scalar reference kernels, CPU output parity on x86-64 and AArch64. Record synchronized reference/BLAS comparisons and math modes. | CPU/runtime / unsafe library ABI and numerical drift. |
| v0.5.0 NVIDIA | M, C → N | On named CUDA hardware upload, multiply, download and compare. Queue-drop, allocation failure, device-loss and unsupported-dtype tests pass. | Device team / runner and driver variance. |
| v0.6.0 portable compute | M, C → W | Vulkan and WebGPU each publish a tested operation/capability subset; verify supported results and rejection on at least two vendors in the combined matrix. | Portable backend / different layout and feature limits. |
| v0.7.0 developer tools | H, L → E | UTF-8/UTF-16 round trips, revision cancellation, rename validation, semantic tokens, hover and formatter idempotence pass on both spellings in a real editor. | Tooling / stale caches or invalid edit boundaries. |
| v0.8.0 package ecosystem | L → K | Rebuild a locked graph offline; reject checksum tampering, incompatible versions and unauthorized scripts; rehearse yank, key revocation and recovery before publishing. | Registry/release / supply-chain incidents. |
| v0.9.0 AI infrastructure | M, C → F; D, K, F → I | Foreign buffer/callback ownership fixture passes, versioned model round-trips and matches reference outputs; collect profiling and distributed fault-fixture evidence. | AI/interop / model drift, lifecycle bugs. |
| v1.0.0 stable profile | V, L, D, C, N, W, E, K, F, I → S | Ratify syntax, semantics, supported ABI, deprecation and release policy. Execute full compatibility corpus on claimed CPU/GPU targets, verify signed release provenance, security review and selected Mongolia-authored std module consumer. | Maintainers / premature stability claims. |
| v1.x capability releases | S, M → Q; S, F → X; K, F → B | Qualify structured cancellation/ownership under failure, one additional ABI/target runner, and one independently versioned domain package with a real consumer. Record capability denial and SDK versions. | Runtime/platform/library owners / fragmented support. |
| v2.0.0 multi-runtime release | Q, K, I → U; B and X for claimed packages/targets | Two-node workload recovers or returns its specified error after loss/duplication; compare with a single-node oracle. Ship protocol negotiation fixtures, migration guide and independently exercised package/target matrix. | Distributed/platform / partial failure and dependency drift. |
| v2.x optimization and domain releases | T, C → O; U, X, B for affected profiles | One opt-in verified specialization portfolio preserves outcomes under cold, warm and forced guard-failure runs. Reproduce accuracy, tuning cost and held-out benefit; qualify each domain package independently. | Optimizer and domain owners / overfitting. |
| v3.0.0 qualified multi-profile release | U, X, O, B → J | Publish complete supported-profile matrix, clean build/rollback and migration from v2 for every advertised profile. Two independently provisioned runners reproduce conformance results. Publish performance with accuracy and uncertainty for each promoted capability. | Release council / unverifiable universal claims. |
| Beyond v3.0.0 | J → Z; individual research RFCs | Demonstrate a Mongolia-authored tool, port one compiler stage, then compare independent bootstrap stages and rerun conformance. Quantum, FPGA, advanced type theory and learned optimization each need their own feasibility decision and consumer. | Research/bootstrap / trusted seeds and hardware access. |

**Scheduling.** Tooling and local package prototypes can start after H/L. Portable GPU and CUDA share M/C and can progress independently once runner access exists. Stage Q or X does not require a simultaneous delivery of every ecosystem project. Promotion of one package never implies that all items in its expansion area passed.

**Release decision.** A missed gate keeps the affected capability experimental or delays the release if that capability defines it. Record the failed fixture, target, owner and proposed resolution. A stable-profile or scope change requires RFC approval and a published compatibility impact statement.
