# Changelog

## 0.1.2 · prepared release candidate

Added reproducible platform ZIPs, verified local-file installers, checksums, production Cargo SBOM, dependency license notices, support templates, distribution CI and paired benchmark regression detection. Compiler and runtime semantics remain at specification 0.1 revision 1. [Release notes](RELEASE_NOTES.md).

## 0.1.1 · foundation stabilization

- Identifiers combining extended Latin and Cyrillic now receive E_SCRIPT after NFC normalization. Files mixing English and Mongolian keywords continue to work. A previously accepted mixed-script identifier can require renaming.
- Decimal leading zeroes now preserve the signed i64 minimum. Public AST validation enforces finite literal payloads. A return type error points to its operand.
- CLI diagnostics clip long lines and render Unicode line and paragraph separators visibly. The draft Mongolian catalog covers additional allocation, limit and confusable errors. Error codes and CLI exit categories remain stable.
- Tensor allocation checks run at the final reservation boundary. Frames and argument lists use fallible reservations. Matrix multiplication accesses contiguous rows while preserving each cell’s contraction order.
- A failed init attempts rollback of files it created in a trusted directory. Added regression tests, a runtime fuzz target, advisory checks and a reproducible-build job. [Verification](PHASE_9_REPORT.md).

## Phase 8 audit · 2026-11-18

The [Phase 8 report](PHASE_8_REPORT.md) records the corrected v0.1.0 Linux candidate, its release/debug verification, security findings, benchmark and remaining gates. The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) now covers all 50 official areas and 620 listed items.

## 0.1.0 foundation candidate

Initial bilingual frontend, static checker, rank-2 f64 tensor interpreter, CLI, conformance tests,
versioned specification and future architecture proposals. See BUILD_REPORT for verification.
Not a stable ABI or production GPU/AI release.
