# Phase 9 · foundation stabilization and production hardening

**Decision:** mn v0.1.1 passes the local foundation gate on Linux x86_64. Both build profiles passed 65 tests. All 11 example expectations, a clean CLI installation and 192 differential matrix invocations passed. The two-crate AST interpreter is the implementation delivered by this patch. Public release approval remains open.

## Provenance, scope and compatibility

Input was the integrated Phase 8/v3 source archive, SHA-256 `cb4fda5d02f427c49e819f8e54f4838e90f69aa0e6078969fd0757fe19b71582`. Its per-file checksums were checked at intake. The archived Phase 8 audit listed concrete fixes and residual risks without an open critical/high severity register. This phase reviewed those dispositions against the code and reproduced six additional observable regressions on the parent. The explicit stabilization directive superseded the earlier proposal to use Phase 9 for HIR.

Compiler patch level is 0.1.1. Mixed-language programs, exact scalar types, rank-2 tensors, increasing-k matmul, static-before-runtime validation and the existing public compiler/CLI entry points remain supported. A mixed extended Latin/Cyrillic name may now require renaming. The signed decimal minimum accepts leading zeroes and programmatic literal ASTs follow the same finite-value rule as source. New projects created by mn init still start with the example package version 0.1.0, independently of compiler version.

## Work and evidence

| Deliverable | Result |
|---|---|
| Compiler, parser and diagnostics | Corrected identifier policy, padded minimum, checker spans and nonfinite literal validation. Bounded CLI excerpts, inert separators and expanded Mongolian messages. [Regression evidence](REGRESSION_TEST_REPORT.md). |
| Runtime and tensor storage | Added final element/byte guards, fallible call storage, measured allocation failure, scaffold cleanup, contiguous matrix traversal. [Security evidence](SECURITY_HARDENING_REPORT.md). |
| Fuzzing and malformed input | Three completed ASan campaigns, 150,000 runs total, plus CLI malformed-byte tests. Leak validation remains open. |
| Continuous integration | Configured hosted Linux/macOS/Windows checks, per-profile tests, clean installation, Linux reproducibility and separate scheduled/manual advisory and leak-enabled fuzz jobs. Local command evidence only. [Workflow](.github/workflows/ci.yml). |
| Build reproducibility | Two independent source/target paths on the same host, locked offline dependencies, path remapping, incremental compilation disabled and a fixed SOURCE_DATE_EPOCH produced identical 851,448-byte release binaries. SHA-256 `51656724433cdba9f9115b2f8f6d7b1abbf1a484b40c0c8a70b69c9c2d9ea669`. [Result](verification/phase9/repro.json) · [Transcript](verification/phase9/repro-build.log). |
| Performance | Paired results and costs for lexer, parser, small matmul, CLI, RSS and file size are in [PERFORMANCE_COMPARISON.md](PERFORMANCE_COMPARISON.md). |
| Handoff | [Changelog](CHANGELOG.md), [limits](KNOWN_LIMITATIONS.md) and [Phase 10 gates](PHASE_10_READINESS.md). Historical audit and expansion requirements are preserved. |

To repeat the host checks, run the commands in [REGRESSION_TEST_REPORT.md](REGRESSION_TEST_REPORT.md) after installing the pinned stable toolchain. For the two-clean-tree comparison run `cargo fetch --locked`, then `python3 scripts/verify_reproducible.py`. For installation, run `cargo install --locked --path crates/mn-cli` then `mn run examples/matmul_mn.mn`. The [README](README.md) includes the complete setup. Archive inventory uses SHA256SUMS. Neither the archive nor the binary carries a maintainer signature.
