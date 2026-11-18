# Phase 8 · Foundation verification

**Linux foundation gate passed.** Both Cargo profiles build and the installed mn CLI executes the supported v0.1 language. The corrected tree includes nine audit deliverables and the corresponding raw evidence. Part 6 has been reviewed against the executable and proposed surfaces. The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) closes the original audit intake with 50 area decisions and 620 item-level contracts.

| Verification | Actual result | Evidence under verification/phase8 |
|---|---|---|
| cargo build --workspace --locked | exit 0 | command-01.log |
| cargo build --release --workspace --locked | exit 0 | command-02.log |
| cargo test --workspace --locked | 54 passed; 1 timing probe ignored | command-03.log |
| cargo fmt --all -- --check | exit 0 | command-04.log |
| cargo clippy --workspace --all-targets -- -D warnings | exit 0 | command-05.log |
| cargo test --release --workspace --locked | 54 passed; same timing probe ignored | command-06.log |
| Cargo dependency graph, all 11 example expectations and isolated install | exit 0; 9 successful programs and 2 expected static errors | command-07/08/09.log |
| Exact matrix oracle | 96 distinct pairs, 192 CLI runs, both profiles matched | command-10.log |
| Fetch locked fuzz dependencies and check both package inventories | exit 0, 13 SHA-256 verifications | command-11.log and dependencies.json |
| Lexer and parser/checker fuzzing | 50,000 cases per target, bounded ASan, leaks disabled | fuzz-lexer.log and fuzz-parser.log |
| Release benchmark | Explicit timing probe passed; CLI/RSS/reference samples retained | benchmark.json and benchmark.log |

## Audit deliverables

| File | What it contains |
|---|---|
| [AUDIT_REPORT.md](AUDIT_REPORT.md) | Findings, risk/impact, source inspection and architectural duplication |
| [REQUIREMENTS_MATRIX.md](REQUIREMENTS_MATRIX.md) | Original sections 0–30, 25 deliverables, supplied Part 6 and completed v3.0 crosswalk |
| [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) | Current verified, unverified, partial, planned, missing and research capabilities |
| [SECURITY_AUDIT.md](SECURITY_AUDIT.md) | Safety controls, Unicode, budgets, dependencies, fuzzing and residual risks |
| [BENCHMARK_BASELINE.md](BENCHMARK_BASELINE.md) | Latency, throughput, matrix workload, memory, binary size and reproduction |
| [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md) | Language, security, platform and public-release limits |
| [RESTORATION_AND_FIX_LOG.md](RESTORATION_AND_FIX_LOG.md) | Input, concrete fixes, failed measurement fixture and packaging delta |
| [UPDATED_ROADMAP.md](UPDATED_ROADMAP.md) | Prioritized work, dependencies, responsible roles and measurable exit gates |
| PHASE_8_REPORT.md | Verification outcome and audit index |

On Linux extract the ZIP, enter mongolia-lang, run `sha256sum -c SHA256SUMS`, then run the five commands above. Install with `cargo install --locked --path crates/mn-cli`. Both `mn run examples/matmul.mn` and `mn run examples/matmul_mn.mn` print `[[19.0, 22.0], [43.0, 50.0]]`. The standalone test script checks all expected stdout, stderr and status values. The package uses the current toolchain pin and existing lockfiles.

Public release signing, external platform validation and full Unicode review remain open. Future compiler backends and AD require their own conformance gates.

## v3.0 crosswalk verification

The official headings 01–50 have been reconciled. Of the 50 broad areas, 16 have partial implementation, 13 have plans, 14 lack matching implementation or substantive prior design, and 7 need feasibility research. No area satisfies its entire v3.0 scope. The corrected Phase 8 compiler source, fixtures, dependencies and manifests are byte-identical to the original Phase 8 package. verification/phase8/v3/workspace.log records a fresh locked release build, debug build, tests, formatting, Clippy and 11 example checks. verification/phase8/v3/release-tests.log records the release tests.
