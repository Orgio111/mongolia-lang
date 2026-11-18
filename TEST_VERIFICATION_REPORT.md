# Phase 11 local verification

Environment: Linux x86_64, Rust 1.100.0. Exact invocations, exit codes and transcripts are in verification/phase11/commands.json and cmd-00.log through cmd-13.log. The Phase 10 baseline rebuilt locally and passed its existing test suite before the new checks. The final local development build passed these gates:

| Command | Exit | Result |
|---|---:|---|
| cargo build --workspace --locked | 0 | Debug build |
| cargo build --release --workspace --locked | 0 | Release build |
| cargo test --workspace --locked | 0 | 72 passed; two timing probes ignored |
| cargo fmt --all -- --check | 0 | Format check |
| cargo clippy --workspace --all-targets --locked -- -D warnings | 0 | Warnings denied |
| cargo test --release --workspace --locked | 0 | 72 passed; two timing probes ignored |

A preliminary Clippy check identified a method implementation below a test module. Moving it above the module resolved the warning. The negative log is retained alongside the final run.

- Seven added tests cover deterministic resolved calls and local slots, source-order inference, isolation, invalid returns, static and dynamic equality obligations, explicit conversion boundaries, malformed syntax, localized errors, an exact HIR snapshot and original decomposed-identifier byte spans. The corruption test injects eleven invalid HIR states. Generated differential tests compare equivalent English and Mongolian modules and their numerical output for 33 input values.
- The accepted foundation corpus, two new cast examples, clean installed CLI and exact numerical oracle passed. See cmd-07.log, cmd-08.log and cmd-09.log. See COMPATIBILITY_REPORT.md for the additional parent/child differential.
- Three ASan fuzz campaigns completed 50,000 runs each (seed 20261119, maximum 4096 bytes, timeout 10 seconds, detect_leaks=0). Summaries are in verification/phase11/fuzz-*.log. Leak checking remains open.
- The locked dependency inventories were verified. Two isolated offline release builds produced identical native executables (cmd-13.log). These results apply to the recorded toolchain and host. Hosted platform CI and an independent maintainer review are separate gates.
