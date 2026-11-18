# Build and verification report

## Phase 10 · compiler 0.1.2

See [BUILD_VERIFICATION_REPORT.md](BUILD_VERIFICATION_REPORT.md) for the current six mandatory commands, distribution checks and unexecuted gates.

## Phase 9 · v0.1.1

Both host profiles passed 65 tests each. The five requested Cargo checks, example and installed-CLI checks, and numerical oracle exited 0. The [regression report](REGRESSION_TEST_REPORT.md) links command logs. Two path-remapped offline release builds in independent directories produced identical bytes. See [Phase 9](PHASE_9_REPORT.md) for scope and remaining release gates.

## Phase 8 audit · 2026-11-18

The [Phase 8 report](PHASE_8_REPORT.md) records the corrected v0.1.0 Linux candidate, its release/debug verification, security findings, benchmark and remaining gates. The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) now covers all 50 official areas and 620 listed items.

## Phase 7 future architecture verification · 2026-11-18

Input SHA-256: c3a8dfcf3bf409e5b2ec456e79a1d5799c16dea65cca7080be8aadb7ec8a7b23. Verified the Phase 6 per-file inventory after extraction. Ran the locked Linux build, all 49 tests, formatting, Clippy and all 11 example checks with scripts/verify.py. See verification/phase7/workspace.log.

Executed python3 docs/phase7/mlp_reference.py for the 13-parameter Python oracle. The 39 central finite-difference checks and 200 SGD updates passed the fixture's checks. See verification/phase7/mlp-reference.json. The future subsystem designs and their proposed conformance tests are documented in docs/PHASE_7_FUTURE_ARCHITECTURE.md. This phase provides no runtime evidence for future targets.

## Phase 6 source distribution · 2026-11-18

The package input was mongolia-lang-v0.1.0-phase5.zip, SHA-256
`4a82356ec0c53a9325b19cac2bcfe7ed52060a7bb65b509182cb70f59ca73c1e`.
The archive carries the complete source workspace, the locked dependencies' metadata,
test fixtures, examples, fuzz harnesses and seed inputs, license texts, architecture and
specification documents. The per-file SHA256SUMS inventory covers the extracted contents.
Runtime and compiler sources match the Phase 5 input byte for byte.

A clean source export on Linux x86_64 passed the locked debug build, 49 tests, formatting,
Clippy and all 11 example checks. A release-mode Cargo install into a temporary prefix
succeeded with Rust 1.100.0. Verification invoked the installed binary by absolute path.
Version, help, init, check and run succeeded on the generated project. Both check and run
returned the recorded status, stdout and stderr for every bundled .mn example, including
the two intentional errors. Rust and dependency downloads were available in the host cache.

The transcripts from the package candidate are in verification/phase6/workspace.log and
verification/phase6/install.log. The delivered source ZIP was then extracted again and
its checksums and installed CLI verified independently. Use scripts/verify_install.py
to repeat the installed CLI checks. Phase 5 release-profile tests and fuzz records are
retained under verification/phase5/. Windows/macOS execution and leak checking remain open.

## Phase 5 verification · 2026-11-18

On Linux x86_64, a clean extraction of the Phase 4 ZIP passed its 48-test baseline. An additional Unix CLI test reproduced a partial scaffold. If main.mn was a dangling symlink, mn init returned exit 2 after creating mn.toml. The fix checks both directory entries with symlink_metadata before creating either file. Both create_new calls still protect existing entries at open time. The regression test confirms exit 2, empty stdout, a preserved link and no manifest in that case.

| Verification | Observed result | Evidence under verification/phase5 |
|---|---|---|
| Input baseline | Phase 4 build, 48 tests, fmt, Clippy and 11 CLI examples passed | baseline.log |
| Regression before/after | New test failed on baseline and passed after the fix | init-before.log, init-after.log |
| Corrected debug workspace | Locked build, 49 tests including A–E, fmt and Clippy passed | workspace.log |
| Release workspace | Locked release build and all 49 tests passed | release.log |
| Example outputs | All 9 positive and 2 negative examples matched exact exit status, stdout and stderr under both profiles | workspace.log, examples-release.log |
| Independent matrix oracle | 64 compatible and 32 incompatible shape pairs, each executed with debug and release CLI, 192 invocations passed | matrices.log |
| Coverage-guided fuzzing | 50,000 runs per target with seed 20261118 and ASan. Lexer and parser/checker campaigns completed | fuzz-lexer.log, fuzz-parser.log |
| Leak checking | Initial lexer campaign completed its iterations and failed during LeakSanitizer shutdown because tracing was restricted. Successful reruns disabled leak detection | fuzz-lexer-initial.log |

The matrix oracle uses Python Fraction arithmetic for products of small dyadic rational entries. These inputs, products and sums are exactly representable in f64. It compares full result shapes and values, checks rejection of incompatible contractions and compares both binaries' output streams. Use the following commands for reproduction, followed by the two campaigns in fuzz/README.md.

```sh
python3 scripts/verify.py
cargo test --workspace --release --locked
cargo build --workspace --release --locked
python3 scripts/check_matmul.py target/debug/mn target/release/mn
```

Windows and macOS runs, leak detection on a supported host and longer fuzz campaigns remain open. Debug and release share the AST interpreter. The differential runs compare optimization profiles of that implementation. Native IR, AD and GPU backends remain future verification targets.

## Phase 4 implementation verification · 2026-11-18

Ran scripts/verify.py on Linux x86_64 with the pinned Rust toolchain. Locked workspace build, 48 tests including one compile-fail doctest, rustfmt check and locked Clippy with warnings denied passed. All 11 CLI examples matched exit code, stdout and stderr. The Unix-specific path test ran on this host. See verification/phase4/workspace.log for the actual commands and results.

## Phase 3 repository verification · 2026-11-18

The Phase 2 source export was the input. Cargo metadata reported the mn-core library and mn-cli package, which builds the mn executable. The packaged workspace includes Cargo.lock, the pinned toolchain, governance and license files, specifications, contributor instructions and examples.

| Check | Observed result |
|---|---|
| Workspace verification on Linux x86_64 | Locked build passed, 42 tests passed, format check and Clippy passed |
| Example conformance | All 11 programs matched their expected exit code, stdout and stderr |
| Clean source checkout | Built with an empty target directory using cached toolchain/dependencies |
| README install instructions | Local Cargo install succeeded into a temporary install root. Installed binary reported mn 0.1.0 |
| Installed CLI | English and Mongolian matmul printed the same expected matrix. A new project from mn init printed Hello, Mongolia! |
| Negative check | Mongolian diagnostic E_SHAPE, exit 1, empty stdout |
| CI coverage | Configured for Ubuntu, macOS and Windows. The execution evidence here is Linux only |

Raw records are in verification/phase3/workspace.log and verification/phase3/checkout.log. The project version is 0.1.0. Phase 3 denotes the repository engineering milestone.

## Phase 2 verification · 2026-11-18

On Linux x86_64 with the pinned Rust toolchain, all four required Cargo commands passed. The test run included 25 core unit tests, 7 CLI tests, 7 new specification tests and 3 stress/golden tests (42 total). All 11 example outcomes matched the foundation reference, including exit status, stdout and stderr. Read the fresh command transcripts in verification/phase2 and the requirement matrix in docs/PHASE_2_ACCEPTANCE.md.

## Original foundation verification

Date: 2026-11-17. Status: verified foundation candidate; public release gates remain open.

Rust was initially absent. It was installed successfully, and all results below are actual
executions against the packaged source. Toolchain: rustc 1.100.0 (0ef1bc386 2026-11-09).
Host: Linux x86_64. Exact version/platform output and command transcripts are in `verification/`.

| Command | Exit | Result |
|---|---:|---|
| `cargo build --workspace --locked` | 0 | Pass |
| `cargo test --workspace --locked` | 0 | 35 tests passed; 0 failed |
| `cargo fmt --all -- --check` | 0 | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | Pass |

The 35 tests comprise 25 core unit tests, 7 CLI integration tests and 3 stress/golden tests.
Generated cases include 2,048 Unicode inputs, 2,048 valid-source mutations, 1,024 scalar
reference comparisons, 100 matrix dot products and matrix identity properties. These are
bounded deterministic tests; they do not establish absence of bugs or replace sustained fuzzing.

Acceptance A/B produce `[[19.0, 22.0], [43.0, 50.0]]`; C produces `30`; E produces `42`.
D rejects `[2, 3] @ [2, 2]`, identifies incompatible inner dimensions 3 and 2, highlights
its expression, returns exit 1 and emits no program output. Both diagnostic languages checked.

All 9 executable `.mn` examples ran successfully. Both negative shape examples failed as
expected. Full stdout/stderr captured in `verification/examples.json`. Future `.mn.proposal`
files are intentionally not executable and were not tested as implemented features.

No native CPU, VM, GPU, AD, package registry or LSP verification occurred: those implementations
do not exist. No throughput, latency, comparative speed or training convergence claims are made.
Cross-platform CI configuration is supplied but its remote jobs were not run in this session.
Coverage-guided fuzz campaigns, official Unicode conformance suites, sanitizers, dependency audits,
external security review and independent Mongolian terminology review remain release work.

Cargo.lock is generated by Cargo and records registry checksums. A locked build still requires
access to missing dependencies on a fresh machine. The archive does not include build outputs
or vendored third-party source. This report is evidence for this snapshot, not future revisions.
