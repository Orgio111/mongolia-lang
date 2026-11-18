# Compiler and release security audit

Scope is the v0.1.0 source package and its two Cargo lockfiles. The CLI reads a user-selected file, and init creates a local project. The language has no network API, native FFI, package installation or GPU driver. External dependencies, Rust/OS libraries and the build host are separate trust boundaries.

| Boundary | Observed control and test | Residual risk |
|---|---|---|
| Rust memory safety | Both production crate roots forbid unsafe_code. Reviewed the lexer/parser, AST constructors, validator, tensor buffer operations and evaluator. No production todo!, unimplemented! or explicit panic on user input found. Indexed parser accesses rely on the private EOF sentinel; tensor indexing follows validated dimensions and area checks. | Rust std and dependencies have their own unsafe implementations. Resource exhaustion, host aborts and bugs outside the crates remain possible. The owned AST API assumes a trusted Rust caller for arbitrary preconstructed input size. |
| Numeric and storage overflow | Checked i64 operations, explicit zero-divisor handling, checked element and byte products, fallible reserve before tensor resize, checked matmul work charge before result allocation. Debug/release boundary tests and exact matrix oracle passed. | Scalar f64 permits NaN and infinity by specification. General allocator failures do not all become recoverable language errors. |
| Unicode and diagnostic display | Byte cap before UTF-8 decode, exact invalid offset, XID/NFC, local confusable skeleton, mixed ASCII Latin/Cyrillic policy, original byte spans. New display sanitization covers C0/C1 controls via is_control and bidi marks/embeddings/isolates. CLI regressions cover CR and tab in both language modes. | Full-script confusables, grapheme/terminal width, font behavior and all display ambiguities remain open. String program output is data. |
| Malformed input/depth | 1 MiB, 100,000 non-EOF tokens, bounded expression entries/depth, fallible syntax and type checks. Stress tests exercise arbitrary Unicode and mutations; fuzz logs below. | No global heap/CPU/wall-clock cap. Environment/AST storage, diagnostics and output allocations may raise peak memory. |
| Runtime/resource effects | 1M steps including matrix work and value reads, 1M elements/tensor, 1M UTF-8 output bytes, bounded call/eval depth. Output buffering discards earlier prints when execution fails. | Fuel models work units rather than time or all allocations. A process supervisor is required for untrusted workloads. |
| Filesystem and release | Exclusive creation of scaffold files, dangling-link tests, unknown-option regression, archive checksum list, exact hashes for both registry dependency sets. | Two-file init is nontransactional; a trusted working directory is assumed. Unsigned checksums do not authenticate the author. CI actions use version tags; immutably pin and audit them before a signed public release. |

## Dependency vulnerability review

Compared every registry package from Cargo.lock and fuzz/Cargo.lock against the package directories in the RustSec advisory database at commit `233155b4025b854c68a4b199c0e32d7fd53291a2` (2026-11-17). All 13 distinct cached package archives matched their lockfile SHA-256. The production graph has three external packages: unicode-ident 1.0.22, unicode-normalization 0.1.24 and tinyvec 1.13.3. The snapshot contained no advisory record under their crate names.

The fuzz graph also includes arbitrary, cc, cfg-if, find-msvc-tools, getrandom, jobserver, libc, libfuzzer-sys, r-efi and shlex. The only matching package record was RUSTSEC-2024-0006 for shlex. Its patched range is >=1.3.0; the fuzz lock selects 2.0.1. No matching affected version was identified in this snapshot. This was a package-name join, current-version comparison and cached-archive hash verification. It covers published RustSec crate records in that revision. It does not cover the OS, CI action code, Rust toolchain, system C++ compiler, other databases or undisclosed defects. Refresh before publication.

[Database snapshot](https://github.com/RustSec/advisory-db/tree/233155b4025b854c68a4b199c0e32d7fd53291a2) · [Matched advisory](https://rustsec.org/advisories/RUSTSEC-2024-0006.html) · [Machine-readable inventory](verification/phase8/dependencies.json)

To reproduce, fetch both locked dependency sets, check out the database at the recorded commit and run `python3 scripts/phase8_dependencies.py /path/to/advisory-db`. Inspect each returned advisory range against its locked version. The helper assumes Cargo's default cache location.

## Security testing

The stable test suite includes ordinary user-input failures and defensive bounds. Fresh ASan/libFuzzer campaigns for lexer and parser/checker each completed 50,000 inputs using seed 20261118, maximum input 4096 bytes, timeout 10 s and fresh writable corpora. See `verification/phase8/fuzz-lexer.log` and `fuzz-parser.log`. Used nightly-2026-11-12, cargo-fuzz 0.13.2, and ASAN_OPTIONS=detect_leaks=0. Leak detection was excluded. fuzz/README.md gives the harness commands. Fuzz results are bounded evidence, and the parser harness does not execute arbitrary programs. CLI tests separately cover invalid byte sequences.

Security owners should track allocator fault injection, longer continuous fuzzing, private disclosure and release signing as separate acceptance gates in UPDATED_ROADMAP.md.
