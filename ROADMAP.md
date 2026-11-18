# Conditional roadmap and risk register

**Phase 10 milestone.** Release engineering for 0.1.2 is recorded in [PHASE_10_REPORT.md](PHASE_10_REPORT.md). [Phase 11](PHASE_11_EXECUTION_PLAN.md) starts with verified HIR for existing types.

These are milestone proposals, not dates or staffing commitments. Estimates are engineering
person-month ranges for experienced contributors, exclude ecosystem adoption, and require
re-estimation after each prototype. Parallel staffing does not linearly reduce elapsed time.

| Version | Dependencies | Measurable exit criterion | Effort | Principal risk |
|---|---|---|---|---|
| 0.1 | Specification agreement | All mandatory acceptance and negative tests, build/fmt/clippy, bilingual equivalence; documented resource limits | 3–6 | Unicode/security and edge-case semantics |
| 0.2 | Verified frontend | Resolved HIR, module cycle errors, generic fixtures, typed VM agrees with interpreter on conformance corpus | 6–12 | Type inference and module complexity |
| 0.3 | Typed IR/effects | Reverse AD passes finite difference and dot-product suite; deterministic MLP learns on both spellings | 8–18 | Control flow, aliasing, higher-order AD |
| 0.4 | Verified SSA/backend interface | x86_64/AArch64 CPU conformance; measured BLAS/SIMD comparisons; memory plan verified | 9–18 | MLIR version coupling and numerical drift |
| 0.5 | Device ownership and CPU baseline | CUDA kernels agree within declared tolerances; async lifetime, OOM and device-loss tests | 12–24 | Hardware CI cost and driver behavior |
| 0.6 | Capability model | Vulkan/WebGPU supported subsets pass conformance on two vendors and explicit unsupported-operation tests | 12–24 | Feature divergence and portability |
| 0.7 | Incremental frontend | LSP encoding/rename tests, formatter idempotence, editor integration, measured edit latency budget | 6–12 | Incremental invalidation and protocol compatibility |
| 0.8 | Governance and security | Reproducible locked offline builds; checksum/signature failures; registry incident rehearsal | 9–18 | Supply-chain compromise |
| 0.9 | AD/runtime/packages | Interop round trips, selected transformer workloads, distributed failure tests and profiling evidence | 18–36 | Broad scope, performance and distributed reliability |
| 1.0 | All claimed features validated | Stable spec/API/ABI scope ratified; release audit, compatibility corpus, supported-platform CI | 12–24 | Premature stability promises |

Full compiler self-hosting is a separate RFC and long-term project. Selected self-hosted library
modules do not imply a self-hosted compiler. LSP work may overlap earlier versions if frontend
interfaces are stable; roadmap sequencing reflects dependencies, not a prohibition on overlap.

The [Phase 7 implementation sequence](docs/PHASE_7_FUTURE_ARCHITECTURE.md) defines the first HIR slice and workstream-specific exit tests. The v0.3 MLP milestone also needs structs, loops, borrowing, broadcast/reduction rules, optimizer state and a pure parameter-tree derivative contract. Start with scalar differentiation while those prerequisites are being designed.

## Risk register

| Risk | Mitigation | Release gate / owner role |
|---|---|---|
| Confusable identifiers and translation errors | NFC, XID, script policy, UTS39 review, native-speaker RFC review | Language/security maintainers approve vocabulary |
| Malicious nesting or recursion | Input/depth/fuel/allocation limits, fuzzing, process isolation guidance | Runtime maintainers demonstrate bounded failure |
| Shape or size overflow | Checked products and independent matrix tests | Numerical maintainers review every allocation path |
| Divergent backends | Shared semantics, conformance corpus, declared tolerances | Backend maintainers publish evidence |
| Unsafe FFI and async use-after-free | Narrow audited wrappers and completion leases | Security/runtime review with fault injection |
| Unstable upstream APIs | Pinned versions, compatibility adapters, upgrade CI | Toolchain maintainer records supported versions |
| Scope inflation | Versioned proposal docs and feature gates | Release manager refuses unsupported claims |
| Unavailable name/namespace | Check registries, binary/package collisions and trademarks before publishing | Governance approval; no availability claim yet |
| Insufficient maintainers/hardware | Narrow supported targets, explicit maintenance commitments | No unsupported production promise |

See BUILD_REPORT.md for actual evidence and STATUS.md for current remaining work.
