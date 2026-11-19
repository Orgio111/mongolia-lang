# Phase 11 · typed semantics and HIR

Compiler 0.2.0-dev.1 is an internal development checkpoint on phase-11-hir. It implements resolved and typed HIR for the complete existing language and adds explicit numeric conversions in both keyword sets. Both normal CLI paths build and verify HIR before execution can begin. The interpreter continues to run the validated AST.

## Delivered

| Requirement | Implementation and evidence |
|---|---|
| Baseline | Phase 10 source tree checked against the published digest inventory; clean base tests; branch created from GitHub main at 5332080fbffdbe0b9c6a5a168a0a46a9356e4cf7. |
| Semantic analysis | Source-order function IDs, frame-relative slots, exact inferred types, whole-program signature checks, immutable initialization and return/reachability checks. SEMANTIC_ANALYSIS_ARCHITECTURE.md. |
| HIR | One module, typed postorder expression sequences, literal bits, resolved calls/reads, tensor dimensions, statement boundaries, source origins and canonical test dumps. HIR_SPECIFICATION.md. |
| Type and shape development | Explicit i64/f64 conversions. Checked static matmul and a Rust axis-equality interface for future symbolic/dynamic dimensions. TYPE_SYSTEM_SPECIFICATION.md and TENSOR_TYPE_SYSTEM.md. |
| Verification | Normal CLI acceptance, exact cast boundaries, cross-language HIR parity, direct invalid HIR, parser/Unicode properties, resource failures, packaged examples and independent matrix reference. TEST_VERIFICATION_REPORT.md and COMPATIBILITY_REPORT.md. |
| Costs and next phase | Paired Linux release measurements in PERFORMANCE_COMPARISON.md. PHASE_12_EXECUTION_PLAN.md gives the verifier, interpreter-parity and resource gates for a bytecode VM. |

Core and CLI stay in the existing two-crate workspace. Resolution, type checking and HIR construction are fused to preserve diagnostic order and avoid an intermediate copy. Independent structural verification follows. Source syntax for tuples, arrays, structs, enums, aliases, Option/Result, generics, assignments, matches and symbolic/dynamic tensor dimensions remains proposed. The checker implements exact monomorphic constraints. The Rust shape API can describe an outstanding equality guard; this interpreter accepts only fully static tensor programs. Detailed development limits are in KNOWN_LIMITATIONS.md. The stable branch and release artifacts are outside the development change.
