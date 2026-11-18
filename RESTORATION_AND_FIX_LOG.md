# Restoration and fix log

1. Used the Phase 7 ZIP as the parent snapshot. Verified archive identity and its SHA256SUMS. Restored both workspaces, all examples, historical verification, license texts and project guidance. Preserved the v0.1.0 version and dependency locks.
2. Built the original release and observed two unexpected CLI behaviors in isolated temporary directories. `before-release.log` and `before-regressions.log` preserve the results. The old binary accepted an unknown init option and emitted raw CR/TAB in a source diagnostic.
3. Corrected init argument checking and diagnostic rendering in `crates/mn-cli/src/main.rs`; added public-CLI regressions. Updated SPEC.md. Users with a hyphen-prefixed directory can use an explicit path. The stderr display change preserves source spans.
4. Removed the parser's host-width-dependent product precheck. The checker performs the element-cap and checked product validation for every static tensor annotation (including externally assembled ASTs). Added a parse-then-check regression for the portability contract. Shared the private parser entry point with the timing probe.
5. Borrowed static literal strings and frame-name text from the validated AST. String variable reads keep deep-copy semantics, byte fuel charges and fallible reservation. Added a cross-call Unicode string test and all-30-keyword equivalence test.
6. Added benchmark fixtures, a stage timing probe, raw-data harness and Linux per-process RSS sampler. The initial signed-literal fixture exceeded the 256-entry parser cap. Its failure is recorded in benchmark-initial.log. Switched to deterministic nonnegative values and verified results against two external references; production limits were preserved.
7. Restored the requested LICENSE entry point, documented all aliases in llms.txt, linked current English/Mongolian setup instructions and enabled explicit CI release compilation. Audited both locked dependency graphs and their package hashes. No crate versions changed.
8. Reran build, test, fmt, Clippy, installation, example and differential checks; generated the Phase 8 reports, evidence and a fresh checksum list. Compare `verification/phase8/changed-files.tsv` to the Phase 7 manifest for the precise file delta (the delta and checksum list omit their own hashes from the delta).

The workspace contains compiler/CLI source, developer-only fixtures and evidence. Cargo target trees, the advisory DB clone and downloaded toolchains are excluded from distribution.

## Expansion audit addendum

The source archive audited for the 50-area crosswalk had SHA-256 ccf03b933af9691b7435f11e69e580dac7ae0f77b6a5b462717a9681d11bd528. The earlier changed-files.tsv describes the original Phase 7→8 delta. verification/phase8/v3/addendum-delta.tsv records this subsequent documentation/evidence update against the original Phase 8 ZIP.
