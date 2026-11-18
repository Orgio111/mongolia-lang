# Phase 11 · advanced type system and resolved HIR

Start with compiler 0.1.2 as the oracle. Phase 10 introduced release artifacts. HIR, new source types and a VM are future work. Keep the existing frontend/checker and both surface languages on the same validation path.

| Order | Work in mn-core | Exit and evidence |
|---|---|---|
| 1 | Freeze observable behavior using the 0.1.2 corpus, all 30 aliases, forward calls, normalized names, static tensor errors, resource limits and runtime traps. | Baseline outputs, exit status, diagnostic codes, origin spans and evaluation order in both profiles. |
| 2 | Lower validated functions to private function IDs and immutable local slots with explicit scalar or static tensor types. Carry original byte origins; separate function and value namespaces. | Canonical snapshots for equivalent bilingual programs; no uninitialized reads or out-of-frame slots. |
| 3 | Add an independent HIR verifier for call signatures, slot initialization, return types, expression result types, matmul dimensions and origins. | Inject malformed private HIR and require diagnostic Results. Differential frontend corpus and packaged CLI still pass. |
| 4 | Submit an advanced types RFC defining type variables, inference boundaries, unification with occurs check, shape constraints, finite solver budgets and behavior for ambiguous programs. | Principal examples, contradictory constraints, cost bounds and bilingual errors reviewed before exposing syntax. Stage generic instantiation or symbolic dimensions independently after approval. |
| 5 | Benchmark checking/lowering separately and pass distribution, fuzz, numerical and two-build gates. | Domain reviewer and independent QA review deliver signed-off evidence. Defer VM, AD and codegen until typed invariants and compatibility tests are established. |

If lowering changes the first diagnostic, its span or numeric evaluation order, isolate the mismatch and keep the existing interpreter as the release path. Apply VERSIONING.md to every proposed source-language change. See PHASE_9_EXECUTION_PLAN.md for the historical Gate H verifier design.
