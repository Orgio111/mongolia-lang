# Project status — v0.1 foundation candidate

**Current compiler 0.1.2.** Phase 10 prepared the release kit. [Release readiness](RELEASE_READINESS.md) is the current gate decision. Earlier entries below record earlier snapshots.

**Compiler v0.1.1 · architecture edition v3.0.** Phase 9 stabilized the validated AST interpreter. See the [Phase 9 results](PHASE_9_REPORT.md), [release limits](KNOWN_LIMITATIONS.md) and [Phase 10 entry criteria](PHASE_10_READINESS.md).

## Phase 8 audit · 2026-11-18

The [Phase 8 report](PHASE_8_REPORT.md) records the corrected v0.1.0 Linux candidate, its release/debug verification, security findings, benchmark and remaining gates. The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) now covers all 50 official areas and 620 listed items.

Phase 7 (Future Architecture) completed on 2026-11-18. All seven foundation phases now have deliverables. See [the Phase 7 design](docs/PHASE_7_FUTURE_ARCHITECTURE.md) for scope, choices, dependencies, test cases and open work. HIR/VM, native AD, hardware backends, device ownership, packages and LSP remain proposed. The next implementation milestone begins with resolved IDs and slots for existing v0.1 programs. Windows/macOS execution, leak validation, terminology review and public release checks remain open.

Phase 6 (Packaging) completed on 2026-11-18. The deliverable is mongolia-lang-v0.1.0-phase6.zip.
The English README and docs/GETTING_STARTED_MN.md cover extraction, checksums, locked builds,
installation and running the examples. scripts/verify_install.py checks the installed binary.
The historical Phase 1 review is included in docs/PHASE_1_ARCHITECTURE.md. BUILD_REPORT.md
records the package checks and known platform limits. Phase 7 concerns future architecture.

Phase 5 (Verification) completed on 2026-11-18. The corrected Linux snapshot passed the locked build, debug and release test suites (49 tests each), formatting, lint and all example expectations. The new init regression, matrix reference comparisons and bounded ASan fuzz campaigns have reproducible commands and transcripts in BUILD_REPORT.md and verification/phase5/. Changes are confined to the CLI preflight, its regression test, verification scripts, the isolated fuzz workspace and reports. The Phase 5 exit criteria for this Linux environment are met. Phase 6 can package this verified snapshot. Cross-platform and leak validation remain pending.

Phase 4 (Implementation) completed on 2026-11-18. The two-crate workspace executes the Phase 2 subset through the validated AST. The CLI retains the whole-program check result for execution. Lexer, syntax nodes, parser, checker, runtime and shared tensor checks now have separate source modules. Both language spellings share these modules.

The phase exit checks cover acceptance A–E, all eleven example outputs, locked build, formatting and lint. All passed on Linux x86_64. The implementation map and remaining limitations are in docs/PHASE_4_IMPLEMENTATION.md. The fresh verification transcript is in verification/phase4/workspace.log.

Phase 3 (Repository) completed on 2026-11-18. Cargo metadata identified mn-core and mn-cli as the two workspace members. The Phase 2 lockfile, compiler sources and grammar remain the baseline. The README maps the repository and links an executable Mongolian getting-started guide. Contributors and the three configured CI platforms use scripts/verify.py, which checks every example against its recorded exit status, output and diagnostic. Git text attributes preserve LF for fixtures and source files.

The repository exit checks passed on Linux x86_64. A clean source export built with the pinned toolchain, installed the mn binary, ran both matmul examples, created and ran a new project, and diagnosed an invalid shape in Mongolian. The complete workspace run passed all 42 tests and all 11 example comparisons. See verification/phase3/ and BUILD_REPORT.md. Remote macOS and Windows CI execution and the public release checks below remain open.

Phase 2 completed on 2026-11-18. SPEC.md and grammar.ebnf now pin down the v0.1.0 subset, and llms.txt reflects the corrected unary operator rule. The acceptance matrix and phase outcome are in docs/PHASE_2_ACCEPTANCE.md. All 42 tests passed in the new verification run.

Original foundation checkpoint follows.

Working interpreter verified on Linux on 2026-11-17. This is a coherent engineering checkpoint,
not completion of the future ecosystem or certification as production ready.

| Phase | Objective / files | Decisions and status | Verification / exit |
|---|---|---|---|
| 1 Architecture | ARCHITECTURE, ADRs | Two crates; safe Rust; validated AST first | Acyclic dependency direction documented |
| 2 Specification | SPEC, grammar.ebnf, formal rules | Narrow normative v0.1; future syntax separately proposed | Acceptance semantics tested; native-speaker review remains |
| 3 Repository | Cargo manifests/lock, toolchain, policy docs | Functional mn-core and mn-cli only | Locked workspace builds |
| 4 Implementation | frontend, diagnostics, engine, CLI | NFC/XID, exact scalar types, immutable values, f64 tensors | A–E pass; no AD/GPU promises |
| 5 Verification | tests, golden fixtures, verification logs | Deterministic generated tests and CLI checks | 35 tests; build/fmt/clippy pass |
| 6 Packaging | README, examples, source archive | Source and reproducible build commands | 9 positive/2 negative examples verified |
| 7 Future architecture | design docs, ROADMAP, risk register | Typed IR/AD/MLIR/GPU/ownership/packages/tooling proposals | Design deliverables; implementations remain future work |

## Remaining work and next checkpoint

1. Submit the reconciled vocabulary and diagnostic wording for independent native-speaker review.
2. Expand Unicode security to a versioned full-script policy and audited confusable tables.
3. Extend the bounded fuzz runs with long-running corpus regression jobs; run CI on all platforms.
4. Improve structured diagnostics (multiple labels, full localization, display-width positions).
5. Establish maintainers, provenance, release signing and namespace/name availability.
6. Start v0.2 only after the foundation gate: resolved HIR, explicit module identities, and VM
   semantics with interpreter differential tests. Do not add empty crates for future features.

Known limits: docs/LIMITATIONS.md. Actual evidence: BUILD_REPORT.md and verification/.
All future milestone versions are proposals, not delivery promises. Complete future language
semantics, ABI guarantees and backend numerical conformance require further RFCs and engineering.
