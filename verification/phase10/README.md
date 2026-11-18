# Reproducing the evidence

Start from the source root and use Python 3.11+ and the pinned rustup toolchain. commands.json and audit-commands.json record the mandatory invocations and status. The environment file records versions. For the release tools on Linux:

```sh
cargo fetch --locked
cargo fetch --locked --manifest-path fuzz/Cargo.toml
python3 scripts/release/inventory.py
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
python3 scripts/release/build_binary.py x86_64-unknown-linux-musl dist
python3 scripts/release/build_binary.py aarch64-unknown-linux-musl dist
python3 scripts/release/verify_binary.py dist/mn-0.1.2-x86_64-unknown-linux-musl.zip
python3 scripts/release/verify_binary.py dist/mn-0.1.2-aarch64-unknown-linux-musl.zip --runner qemu-aarch64
python3 scripts/verify_reproducible.py
python3 scripts/release/verify_docs.py target/release/mn
```

The ARM command shown assumes a qemu-aarch64 executable on PATH; the local verification used an explicit path. The x64 and ARM package logs record the two compilations and final archive hashes. The installed logs include the binary/installer/CLI checks. tamper-test.log records rejection of a modified archive member. The actual benchmark raw data and positive/negative gate logs distinguish the measured timings from the synthetic threshold test. Reproduce with the Phase 9 parent and the commands in PERFORMANCE_COMPARISON.md, then run scripts/release/benchmark_gate.py on the resulting JSON. Fuzz logs specify ASan settings, seed and command. Local archive signatures are unissued; the signing dry-run covers only a disposable fixture.
