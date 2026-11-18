# Phase 10 entry decision

**Historical Phase 9 handoff.** The approved Phase 10 directive selected release engineering and moved the HIR proposal to PHASE_11_EXECUTION_PLAN.md. The current decision is in RELEASE_READINESS.md.

**Ready for an internal resolved-HIR implementation branch based on mn 0.1.1.** The tested AST interpreter remains the semantic reference. The v3 architecture named Gate H as its next compiler milestone. The subsequent Phase 9 directive prioritized stabilization, so Gate H has no implementation or verification claim in this release.

| Gate | Owner role | Acceptance evidence |
|---|---|---|
| Freeze the parent oracle | QA and frontend | Record v0.1.1 bilingual run/check outputs, error codes, spans and exit status for existing programs in both profiles, including the six repaired regressions. Preserve source/target separation for reproducible comparison. |
| Resolve functions and locals | Frontend | Give functions dense source-order IDs and each parameter/immutable binding a frame slot. Check forward calls, recursion, local/function namespaces and NFC names. Invalid programs retain their first diagnostic and no output. |
| Verify private HIR | Compiler core | Validate call signatures, type/shape facts, frame read initialization and original byte origins. Unit tests feed invalid IR directly and require ordinary errors. Add no public syntax or backend flag. |
| Differential gate | QA and runtime | Route both public check/run paths through validation. Prove English and Mongolian inputs yield equivalent normalized HIR and unchanged AST results. All five Cargo commands, numerical oracle, example checks and fuzz targets must pass. Independently review any measured compile-time or memory cost. |
| Public release | Release maintainers | Obtain actual hosted Linux, Windows and macOS results; retain leak-enabled fuzz evidence; complete native terminology and release identity/signature/disclosure/namespace checks. Record target-specific claims with logs. |

Gate H can be developed in the current two-crate workspace. Budget enforcement, the hardening regressions and deterministic contraction order are compatibility constraints. VM execution, AD, MLIR/LLVM and GPU work have later dependency gates. For reviewable increments see the [archived HIR design](PHASE_9_EXECUTION_PLAN.md), [roadmap](MASTER_ROADMAP.md), [current specification](SPEC.md) and [outstanding limits](KNOWN_LIMITATIONS.md).
