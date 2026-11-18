# Roadmap changes from the foundation audit

## Current milestone sequence

1. 0.1.2 local release candidate — reproduce and exercise Linux packages, archive source, verify notices and dependency locks. Evidence in PHASE_10_REPORT.md.
2. Public 0.1.x — release owner closes native platform, leak, identity, license/signature, disclosure and terminology gates in RELEASE_READINESS.md before publishing approved digests.
3. Phase 11 / proposed 0.2 development — resolved, typed HIR with independent verification and AST parity; evaluate advanced type rules by RFC. VM and native lowering follow conformance.

The remaining Phase 8 roadmap is retained for traceability.

Use [ROADMAP.md](ROADMAP.md) for the version dependencies, estimates, risks and complete v0.1–1.0 milestone table. Changes below are gates for moving this snapshot forward. Estimates are planning ranges, not dates.

| Order | Owner role | Work and dependency | Evidence for exit | Estimated effort |
|---|---|---|---|---|
| 1 | Release/CI | Run the packaged source in hosted macOS and Windows CI, add AArch64 and 32-bit semantic checks where supported. Schedule separated fuzz and advisory checks. | Archived toolchain/platform logs, identical exit codes on acceptance cases, oversized annotation E_LIMIT, passing lockfile review. | 1–2 engineer-weeks excluding runner procurement |
| 2 | Language, security, native reviewer | Review every provisional alias and fallback message, define script coverage and display-width/encoding behavior. | Vocabulary RFC approval, structured message catalogue, multilingual fixtures with original byte spans and expected terminal/LSP positions. | 2–4 weeks plus review availability |
| 3 | Runtime/release | Define failed-init recovery policy, memory isolation requirements, allocator fault tests, leak checks, end-to-end signing and private disclosure contact. | Fault-injection results, incident owner, verified source provenance and signed release candidate. | 2–4 weeks |
| 4 | Performance | Repeat baseline on dedicated hardware and cover additional rectangular shapes and input sizes before proposing optimizations. | CPU/flags, raw data, scalar and BLAS oracles, peak RSS and variability across machines. | 1–2 weeks |
| 5 | Compiler | v0.2 first slice maps existing identifiers to function IDs and local slots, retaining source origins and the AST oracle. | Entire v0.1 conformance set retains outputs, errors, spans and work-budget behavior. Then add VM verification and differential execution. | Included in v0.2 estimate |
| 6 | Roadmap owner | Completed the [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) using the official 01–50 list. | All 50 areas classified; each listed item has a status, delivery class and acceptance fixture. | Complete for this snapshot |

After VM and effect/type invariants exist, proceed to v0.3 reverse AD. CPU lowering follows numeric contracts. CUDA and portable backends follow device lifetime tests. Phase 8 creates no new language edition or additional backend.
