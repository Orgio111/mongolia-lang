# Mongolia (.mn) — v0.2.0-dev.1 · Phase 11

**Development branch.** Read [the Phase 11 report](PHASE_11_REPORT.md) and [language update](LANGUAGE_SPECIFICATION_UPDATE.md) for typed HIR, explicit numeric conversions and verification. Build this checkout with Cargo for the development compiler. The distribution information below refers to the stable Phase 10 candidate.

**Compiler v0.1.2 · language specification 0.1 revision 1 · architecture edition v3.0.** [Install mn](INSTALLATION_GUIDE.md), [start writing programs](docs/USER_GUIDE.md), or read the [platform results](PLATFORM_SUPPORT_MATRIX.md) and [release gate](RELEASE_READINESS.md).

The Phase 2 specification is in [SPEC.md](SPEC.md). The [acceptance matrix](docs/PHASE_2_ACCEPTANCE.md) connects each mandatory requirement to a test. [BUILD_REPORT.md](BUILD_REPORT.md) records the verification results for each foundation phase.

Native AI. Native Performance.

A working Rust reference interpreter and static checker for a small bilingual language.
This is an engineering foundation, not a production AI compiler. AD, native code,
GPU execution, borrowing, modules, packages and LSP are future designs only.

The [Phase 8 audit](PHASE_8_REPORT.md) links the requirements, corrections, verification transcripts and measured Linux baseline.

The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) compares the universal expansion plan with the current compiler and links the evidence for each area.

## Repository

The workspace has two members. The CLI depends on the compiler library.

| Location | Contents |
|---|---|
| crates/mn-cli | The mn binary, file loading, CLI diagnostics and end-to-end tests |
| crates/mn-core | Lexer, parser, type and shape checker, interpreter, diagnostic catalog and conformance tests |
| SPEC.md and grammar.ebnf | Executable v0.1 language rules |
| examples | English, Mongolian and mixed programs. The negative subdirectory holds expected errors. |
| verification/examples.json | Expected exit status, stdout and stderr for each executable example |
| rfcs and docs | Architecture decisions, proposals, acceptance criteria and phase reports |

[Монгол хэлээр эхлэх заавар](docs/GETTING_STARTED_MN.md) · [Contributor guide](CONTRIBUTING.md)

The [Phase 7 future architecture](docs/PHASE_7_FUTURE_ARCHITECTURE.md) describes the HIR and VM migration, AD rules, backend contracts, ownership, package resolution and language tooling. The reference MLP fixture runs separately with Python 3.

## Phase 10 source package

Extract `mongolia-lang-0.1.2-source.zip` into a new directory. The top-level
`mongolia-lang/` directory contains the complete Cargo workspace, both lockfiles
(core and fuzz), both license texts, the executable examples, and earlier verification
records. The Phase 1 design review is in docs/PHASE_1_ARCHITECTURE.md. SPEC.md
defines current behavior. The files in examples/future have the .mn.proposal suffix.

On Linux, verify the extracted file checksums from inside mongolia-lang with
`sha256sum -c SHA256SUMS`. This checks every packaged file except the checksum
list itself. The SHA-256 digest of the ZIP accompanies the download.

## Build and install

Install rustup (https://rustup.rs) and the native linker for your Rust host target.
The workspace pins Rust 1.100.0 with rustfmt and Clippy. From the directory that
contains the extracted mongolia-lang folder, run the following commands.

```sh
cd mongolia-lang
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo install --path crates/mn-cli --locked
mn --version
mn run examples/matmul.mn
mn run examples/matmul_mn.mn
mn check examples/negative/shape_mismatch.mn --lang mn
```

The final check intentionally fails. The toolchain is pinned in rust-toolchain.toml;
initial build requires network access to download Rust and four locked registry crates.
Cargo installs mn under its bin directory, which must be on PATH for the plain mn command.
To run directly from the source tree, use `cargo run --locked -p mn-cli -- run examples/matmul.mn`.
For an optimized local build use `cargo build --workspace --release --locked`. The binary
is target/release/mn (mn.exe on Windows) with Cargo's default target directory.

For the full developer checks run `python3 scripts/verify.py`. To exercise installation
in an isolated temporary directory, run `python3 scripts/verify_install.py`. It checks
the installed binary against each example's expected exit status, stdout and stderr.

```mn
функц main() {
    let a = [[1.0, 2.0], [3.0, 4.0]];
    хувьсагч б = [[5.0, 6.0], [7.0, 8.0]];
    хэвлэх(a @ б);
}
```

Output: `[[19.0, 22.0], [43.0, 50.0]]`.

## Current surface

- `mn run FILE`, `mn check FILE`, `mn init [DIRECTORY]`, `mn --help`, `mn --version`.
- `--lang en|mn` selects diagnostics. Mongolian catalog is a draft with English fallback details.
- Types: `i64`, `f64`, `bool`, `string`, `()`, `Tensor<f64, [rows, cols]>`.
- Immutable `let`, typed functions, calls, explicit return, print, rank-2 tensors.
- Exact arithmetic types; no implicit integer-to-float conversion. Tensor cells must be f64.
- NFC identifiers with XID characters; conservative Latin/Cyrillic security checks.
- Whole-program validation before execution. All output buffered until successful completion.

Exit codes: 0 success; 1 source/type/runtime error; 2 usage or I/O failure.
`check` does not evaluate arithmetic: `1 / 0` is well-typed but fails under `run`.
Unsupported commands explicitly fail. `init` creates scaffolding; it does not resolve packages.

Read [SPEC.md](SPEC.md), [ARCHITECTURE.md](ARCHITECTURE.md),
[ROADMAP.md](ROADMAP.md), [STATUS.md](STATUS.md), and [BUILD_REPORT.md](BUILD_REPORT.md).
The public release gate remains open: see [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md).
No performance superiority is claimed. Project license: MIT OR Apache-2.0.
