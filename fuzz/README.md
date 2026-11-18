# Coverage-guided frontend checks

This separate Cargo workspace uses libFuzzer and has its own Cargo.lock. The foundation CLI keeps its stable toolchain and existing dependency graph. The lexer harness converts arbitrary bytes to a valid UTF-8 string with replacement characters and checks ordered spans against character boundaries. CLI integration tests cover rejection of malformed byte streams. The parser harness parses valid UTF-8 inputs, checks every accepted AST and verifies parser/checker diagnostic span boundaries.

Run from the repository root on Linux with a C++ compiler. Install the tools once.

```sh
rustup toolchain install nightly-2026-11-12 --profile minimal
cargo install cargo-fuzz --version 0.13.2 --locked
mkdir -p fuzz/corpus/phase5-lexer fuzz/corpus/phase5-parser
cargo +nightly-2026-11-12 fuzz run lexer fuzz/corpus/phase5-lexer fuzz/seeds/lexer -- -runs=50000 -seed=20261118 -max_len=4096 -timeout=10
cargo +nightly-2026-11-12 fuzz run parser fuzz/corpus/phase5-parser fuzz/seeds/parser -- -runs=50000 -seed=20261118 -max_len=4096 -timeout=10
```

The first corpus directory is writable. Use fresh empty output directories to start with just the packaged seed files. Corpus growth and coverage vary with toolchain and platform. Keep a crashing artifact for replay with `cargo +nightly-2026-11-12 fuzz run TARGET PATH_TO_ARTIFACT`.

The Phase 5 host restricted the tracing operation required by LeakSanitizer at shutdown. The two successful campaigns used `ASAN_OPTIONS=detect_leaks=0` with AddressSanitizer enabled. On a host supporting LeakSanitizer, use its default checks. The initial failed shutdown and both subsequent runs are recorded in verification/phase5.

## Phase 9 runtime target

The runtime harness parses UTF-8, validates the complete program and executes each accepted program twice. It compares complete results and checks any runtime diagnostic interval against the original UTF-8 string. Seeds for all three targets ship in fuzz/seeds. For a fresh campaign:

```sh
for target in lexer parser runtime; do
  mkdir -p "fuzz/corpus/$target"
  cargo +nightly-2026-11-12 fuzz run "$target" "fuzz/corpus/$target" "fuzz/seeds/$target" -- -runs=50000 -seed=20261118 -max_len=4096 -timeout=10
done
```

The Phase 9 host ran each target with ASan and leak detection disabled after a failed LeakSanitizer shutdown. See verification/phase9/fuzz-*.log. Scheduled CI requests leak detection on a compatible Linux runner. Generated corpora and fuzzer build products are local working files.
