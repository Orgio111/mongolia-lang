# Build and verification · compiler 0.1.2

Local environment: Linux x86_64, kernel 6.18.57, Rust 1.100.0 (0ef1bc386059596dada8fb65b50425aab2ea637a), Cargo 1.100.0, LLVM 23.1.5. The native build target is x86_64-unknown-linux-gnu. The two packaged targets use musl. Full stdout/stderr and exit status are under verification/phase10. [Environment](verification/phase10/environment.txt) · [Commands](verification/phase10/commands.json) · [Audit commands](verification/phase10/audit-commands.json).

| Mandatory command | Exit | Observation |
|---|---:|---|
| cargo build --workspace --locked | 0 | Debug workspace built |
| cargo build --release --workspace --locked | 0 | Release workspace built |
| cargo test --workspace --locked | 0 | 65 passed, one timing probe intentionally ignored |
| cargo fmt --all -- --check | 0 | Formatting matched |
| cargo clippy --workspace --all-targets --locked -- -D warnings | 0 | Warnings denied |
| cargo audit | 0 | 1,533 advisories loaded; no reported matching vulnerability |

## Additional evidence

| Check | Observed result |
|---|---|
| cargo test --release --workspace --locked | 65 passed, one timing probe ignored |
| python3 scripts/verify.py and scripts/verify_install.py | Both exit 0. All 11 bundled .mn files matched the oracle, including two expected static errors. Clean local Cargo installation passed. |
| python3 scripts/check_matmul.py DEBUG RELEASE | 192 CLI runs over 64 compatible and 32 incompatible matrix pairs; both profiles matched the independent rational oracle. |
| python3 scripts/release/verify_docs.py RELEASE | README program and both complete llms.txt programs matched their expected outputs. New user guides refer to the packaged .mn fixtures. Proposal files and specification fragments are marked separately. |
| python3 scripts/release/inventory.py and cargo audit --file fuzz/Cargo.lock | Both exit 0. 14 registry inputs and declared license expressions verified. |
| Two offline native builds | 851,464-byte x86-64 GNU binaries had identical SHA-256 427e43159bc03a5c4995c257f2e518124e4f975e664ea83a607ebface2ba248e. |
| Scripts in scripts/release for packaging | Each musl target was built twice and compared before ZIP creation. Verified every archive member, rejected a wrong binary hash and an overwrite, installed to a path containing spaces and ran all public commands. Missing files/commands produced exit 2 with stderr. ARM CLI executed under QEMU. |
| Paired comparison against 0.1.1 | Six paired batches (10 warmups, 30 measured samples each). No lexer/parser/small-matmul stage exceeded the 20% elapsed-time review budget in at least five pairs. This threshold is a regression signal for these fixtures. |
| Coverage-guided campaigns | Each of lexer/parser/runtime completed 50,000 ASan runs with detect_leaks=0, seed 20261118. The three leak-enabled attempts each exited 1 at LeakSanitizer shutdown under the tracing restriction. Leak verification is **BLOCKED**, not passed. |
| CI and signing | actionlint 1.7.9 passed both workflow files. OpenSSH disposable-key signature and tamper tests passed. GitHub Actions, real maintainer signing, Windows and macOS execution: **NOT EXECUTED**. |

[Release package logs](verification/phase10/x64-package.log) · [ARM package log](verification/phase10/arm-package.log) · [x64 installation](verification/phase10/x64-installed.log) · [ARM installation](verification/phase10/arm-installed.log) · [Paired samples](verification/phase10/performance.json) · [Fuzz statuses](verification/phase10/fuzz-results.json) · [Reproducibility](verification/phase10/repro.json).
