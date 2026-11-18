# Regression verification · mn 0.1.1

Both debug and release completed 65 tests on Linux x86_64 with Rust 1.100.0. Each profile skipped one explicitly ignored timing probe, which the performance harness ran separately. The Phase 8 suite had 54 tests. Six added regression checks were run against the parent and failed in their intended assertions. The remaining added checks exercise defenses and compatibility. [Pre-fix results](verification/phase9/before-regressions.log) · [Debug](verification/phase9/debug-tests.log) · [Release](verification/phase9/release-tests.log).

| Component | Regression or invariant | How to find the check |
|---|---|---|
| Lexer | Extended Latin/Cyrillic combinations and decomposed variants are diagnosed at the original bytes. Separately named Latin and Cyrillic variables work in a mixed program. | stabilization.rs, script_checks_use_normalized_unicode_scripts |
| Parser | Padded signed minimum evaluates to i64::MIN. The unnegated magnitude and next negative magnitude fail. | stabilization.rs, padded_minimum_has_the_same_integer_value |
| Checker | Returning a bool in an i64 function highlights the expression, including its parentheses and Cyrillic text. | stabilization.rs, return_mismatch_labels_the_returned_expression |
| Public validation | Programmatically inserted NaN and positive/negative infinity literals fail checking. A valid division can still produce infinity. | stabilization.rs, externally_supplied_nonfinite_literals_are_rejected |
| Matmul | A rectangular cancellation-sensitive f64 example checks increasing-k accumulation. The exact oracle additionally compares randomized shape acceptance and numbers across profiles. | stabilization.rs, rectangular_matmul_preserves_contraction_order; scripts/check_matmul.py |
| Translation | Confusable names are retained in the Mongolian diagnostic. A recursive call gets a localized depth message. | stabilization.rs, mongolian_limits_and_confusable_messages_identify_the_problem |
| CLI diagnostics | Long Unicode lines retain their true column; rendering is bounded. Unicode separators become inert. Truncated UTF-8 reports its original byte offset. | Three CLI integration tests in crates/mn-cli/tests/cli.rs |
| Project creation | Inject failure in first write, partial second write and an existing second path. Check cleanup, preservation of a prior entry, then successful retry. | scaffold.rs, init_rolls_back_only_files_it_created_and_allows_retry |
| Tensor allocator | Check element cap, usize byte overflow, largest valid area and excessive result product. | tensor.rs, buffer_allocator_defends_the_final_reservation_boundary |

## Commands and outputs

All rows ran from the workspace root. Exit codes and exact commands appear in [commands.json](verification/phase9/commands.json).

| Command | Result |
|---|---|
| cargo build --workspace --locked | Exit 0 |
| cargo build --release --workspace --locked | Exit 0 |
| cargo test --workspace --locked | Exit 0, 65 passed |
| cargo fmt --all -- --check | Exit 0 |
| cargo clippy --workspace --all-targets -- -D warnings | Exit 0 |
| cargo test --workspace --release --locked | Exit 0, 65 passed |
| cargo fmt --manifest-path fuzz/Cargo.toml -- --check | Exit 0 |
| python3 scripts/verify.py | Exit 0, 11 examples matched exit/stdout/stderr |
| python3 scripts/verify_install.py | Exit 0, temporary release install, help/version/init and both check/run on every example |
| python3 scripts/check_matmul.py DEBUG_BINARY RELEASE_BINARY | Exit 0, 64 compatible and 32 incompatible pairs, 192 CLI executions |

The bilingual acceptance programs produce the matrix [[19.0, 22.0], [43.0, 50.0]], mixed-syntax sum 30 and function result 42. The incompatible [2,3] and [2,2] example returns exit 1 with E_SHAPE before program output. The independent Python oracle uses exact rational arithmetic on dyadic inputs and verifies both the results and the diagnostic contract. [Examples](verification/phase9/examples.log) · [Installed binary](verification/phase9/install.log) · [Numerical oracle](verification/phase9/numerical.log).

The core suite also passed on emulated i686 and AArch64 Linux musl targets, 49 passes per target including the compile-fail doctest, with one ignored measurement. A native i686 invocation failed with an unsupported executable format. QEMU provided the userspace runner. These runs cover word-size and architecture behavior in core. Platform-native CLI and filesystem CI are separate gates. [i686 attempt](verification/phase9/i686.log) · [i686 emulation](verification/phase9/i686-emulated.log) · [AArch64 emulation](verification/phase9/aarch64-emulated.log).
