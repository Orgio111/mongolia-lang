# Versioned examples

Run supported examples from the repository root with `cargo run --locked -p mn-cli -- run examples/matmul.mn` (or install `mn` and use `mn run ...`).

| Showcase | English | Mongolian | Minimum version/status |
|---|---|---|---|
| Hello World | `hello.mn` | `hello_mn.mn` | v0.1; greetings intentionally localized |
| Scalar arithmetic | `scalar.mn` | `scalar_mn.mn` | v0.1; outputs 14, 3.5, true |
| Typed functions | `functions.mn` | `functions_mn.mn` | v0.1; output 42 |
| Matrix multiplication | `matmul.mn` | `matmul_mn.mn` | v0.1; output [[19.0, 22.0], [43.0, 50.0]] |
| Shape rejection | `negative/shape_mismatch.mn` | `negative/shape_mismatch_mn.mn` | v0.1 negative test; must fail before execution |
| Broadcasting | `future/broadcasting.mn.proposal` | `future/broadcasting_mn.mn.proposal` | v0.3 target proposal |
| Automatic differentiation | `future/autodiff.mn.proposal` | `future/autodiff_mn.mn.proposal` | v0.3 target proposal |
| Two-layer neural network | `future/mlp_training.mn.proposal` | `future/mlp_training_mn.mn.proposal` | v0.3 target proposal |
| Optimizer training loop | Same complete MLP file | Same complete MLP file | v0.3 target proposal |
| Host/device transfer | `future/gpu_transfer.mn.proposal` | `future/gpu_transfer_mn.mn.proposal` | v0.5 target proposal |
| C FFI call | `future/ffi.mn.proposal` | `future/ffi_mn.mn.proposal` | Future, earliest v0.4 proposal |
| Package/module imports | `future/imports.mn.proposal` | `future/imports_mn.mn.proposal` | Modules v0.2; registry packages v0.8 proposal |

`mixed_syntax.mn` demonstrates English and Mongolian keyword interchangeability and prints 30. The `main` entry point has the same exact name in both syntaxes. Do not run `future/` with v0.1: the `.proposal` suffix deliberately distinguishes these designs from executable source.

Bilingual numerical pairs use identical values, operator order, parameter initialization, and iteration count. Boolean printing is canonical `true`/`false` in both syntaxes. Translating source keywords does not localize runtime values. Language tags on diagnostics only affect diagnostics.

Full future API assumptions and a companion C definition are documented in `docs/SHOWCASES.md`. No future example has been compiled, trained, benchmarked, or verified by a future backend. Version labels specify the earliest proposed feature milestone, not a promise of availability.
