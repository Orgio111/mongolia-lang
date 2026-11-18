# Phase 10 · release engineering and distribution

**Result.** Mongolia 0.1.2 is prepared as an unsigned source and Linux binary candidate. The previous releases in this repository are 0.1.0 and 0.1.1. The new patch packages the verified interpreter, preserves language specification 0.1 revision 1 and retains the MIT OR Apache-2.0 project license. Public release is on hold pending the gates in [RELEASE_READINESS.md](RELEASE_READINESS.md).

## Review and changes

The Phase 9 archive was verified against its checksum inventory. Review covered its stabilization report, regression report, security and performance evidence, changelog, limitations, Phase 10 handoff, both Cargo locks, lexer/parser/checker/runtime and CLI, specification and unified v3.0 architecture. The Phase 9 handoff proposed resolved HIR as the next technical step. The subsequent release-engineering mandate schedules HIR for Phase 11. All 18 Rust sources/tests and the grammar are byte-identical to Phase 9. The version comes from workspace metadata. The only compiler build change is the patch version.

| Deliverable | Location |
|---|---|
| Release version, history and compatibility | VERSIONING.md, RELEASE_NOTES.md, CHANGELOG.md, SPEC.md |
| Executables, sources, digests and build details | RELEASE_MANIFEST.md, kit SHA256SUMS, per-ZIP BUILD.json or source SHA256SUMS |
| Installers, public CLI, documentation | INSTALLATION_GUIDE.md, installers/, docs/USER_GUIDE.md, docs/GETTING_STARTED_MN.md, examples/ |
| Open-source project preparation | README.md, both licenses, CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, GOVERNANCE.md, SUPPORT.md, assistant guidance, .github templates |
| CI, build and platform results | .github/workflows/, BUILD_VERIFICATION_REPORT.md, PLATFORM_SUPPORT_MATRIX.md, verification/phase10/ |
| SBOM, license and security review | SUPPLY_CHAIN_SECURITY_REPORT.md, release/SBOM.spdx.json, release/third-party/, scripts/release/ |
| Handoff | UPDATED_ROADMAP.md, PHASE_11_EXECUTION_PLAN.md |

All six mandatory verification commands succeeded locally. The installed bilingual CLI passed every packaged example in both Linux executable packages. Controlled memory-pressure limitations, Unicode security boundaries, diagnostic language review, sanitizer limits and missing native platform evidence remain documented. No production performance ranking is asserted. The release kit contains the implemented AST interpreter. Native .mn code generation, HIR, AD, GPU and the broader v3.0 expansion remain future work.
