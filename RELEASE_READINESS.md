# Release gate decision · HOLD for public publication

**The local Linux candidate and its source are verified for engineering evaluation.** The public release gate remains open. A review of the Phase 8 findings, Phase 9 hardening evidence, current Rust files, grammars, probes and both dependency locks identified no new critical/high compiler defect in this scope. A successful local build does not close the independent release controls below.

| Gate | Status | Disposition |
|---|---|---|
| Intake, mandatory Cargo commands, production/fuzz dependency audit and source/spec consistency | PASS | Fresh command logs. Verified the 18 Rust implementation/test files and grammar against Phase 9. No source-language change in 0.1.2. |
| Linux source and packaged binary evidence | PASS within documented scope | Native x64 and QEMU AArch64 CLI, clean builds, hashes, positive/negative example checks and install failures verified. |
| Regression and performance | PASS within measured corpus | Both profiles, matrix oracle, bounded ASan runs and paired timing gate. |
| Leak-enabled testing | BLOCKED | Local sanitizer shutdown lacks tracing support. Run scheduled/manual CI on a compatible Linux machine and retain a successful log or investigate any reported defect. |
| Hosted native platforms | NOT EXECUTED | Release maintainer must retain actual supported Windows/macOS and native ARM results before claiming those targets. Recompute per-target artifact digests in the authorized release build. |
| Public provenance, disclosure and name ownership | OPEN | Appoint release/security owners; verify name/registry identities, private reporting route and license review. Publish trusted signing identity and verify a maintainer signature on the final index. |
| Vocabulary and external review | OPEN | Obtain native-speaker review of provisional words and diagnostic fallback; record any approved change through the RFC process. |
| Public distribution | NOT EXECUTED | No GitHub release, registry upload or package manager submission was authorized. Approve exact digests before publication. |

Withhold any affected artifact if a mandatory check fails, an untriaged critical/high defect exists, a required source file is missing, a documented example misbehaves or either build/checksum comparison fails. Move to Phase 11 development against this local oracle while the public release controls are completed. See PLATFORM_SUPPORT_MATRIX.md for the execution boundary of each target.
