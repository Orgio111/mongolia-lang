# Contributing

Read SPEC.md and the current BUILD_REPORT.md before changing behavior. Keep implemented and future
features separate. Submit focused changes with a problem statement, semantic effect and evidence.
Run workspace build/test with --locked, rustfmt check and clippy with warnings denied. Add positive
and negative cases for new semantics; bilingual syntax must have equivalent behavior.

Language changes require an RFC: motivation, exact grammar, typing/effects, errors, compatibility,
security, examples, alternatives, implementation plan and measurable acceptance tests. No silent
numeric conversion, overflow-policy or tensor-layout change. Native speakers review vocabulary.
Compiler maintainers review source spans and invariants; runtime maintainers review allocations,
resource limits and effects. Unsafe code requires a written safety contract and independent review.
Do not add empty crates, fabricated benchmark claims, TODO implementations behind supported APIs,
or future syntax to executable examples. Record actual command results, including failures.

## Check a checkout

From the repository root run `python3 scripts/verify.py` (Windows: `py -3 -X utf8 scripts/verify.py`).
The same script runs in CI on Ubuntu, macOS and Windows. It builds with the pinned Rust toolchain,
runs the test suite, checks formatting and Clippy, then compares every executable example with its
reviewed output in verification/examples.json. For a new .mn example, add its expected stdout,
stderr and exit code to that file. Mongolian examples ending in _mn.mn run with --lang mn.

For a focused change use `cargo test --locked -p mn-core` for language behavior or
`cargo test --locked -p mn-cli` for the command-line interface. Cargo.lock and rust-toolchain.toml
belong at the workspace root. New compiler modules start in the crate that owns their data.

For release changes run both Cargo profiles, the release inventory check, the packaging verifier and the corresponding installer smoke test. Record target, runner and exit status in verification. Attach paired measurements when changing an optimization. Distribution PRs require the checks in RELEASE_READINESS.md.
