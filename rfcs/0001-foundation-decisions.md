# ADR set: foundation

Status: accepted for the v0.1 subset; revisit through RFC, never silently.

1. **One bilingual frontend.** Normalize keyword spellings before parsing and identifier NFC
before name resolution. Consequence: English and Mongolian programs share checking and
execution; spelling remains in source spans for diagnostics. Translation vocabulary remains
provisional until native-speaker review. Separate runtimes would cause semantic drift.
2. **Two functional crates.** Keep core mechanisms separate from IO/CLI. Split core only when
stable interfaces and independent tests justify it; the target architecture is not a mandate
for empty modules.
3. **Validated AST first.** Minimize premature IR engineering while requiring complete static
checking before side effects. Typed IR is a prerequisite for AD and multiple backends.
4. **Exact scalar types.** No implicit i64/f64 promotion. Integer arithmetic traps on overflow;
f64 follows IEEE arithmetic. Tensor storage initially uses f64. Additional precisions require
specified rounding, promotion and accumulator policies.
5. **Static rank-2 shapes first.** Reject incompatible inner dimensions before running any
function. Dynamic dimensions and broadcasting are distinct future work.
6. **Safe Rust and bounded execution.** Avoid unsafe in the foundation. Limits provide
resource control, not a sandbox guarantee. Recoverable language faults become diagnostics.
7. **Private implementation ABI.** Neither AST layout nor bytecode is a stable external ABI.
Future C FFI has explicit layout, ownership and unwinding rules.
8. **Evidence over performance claims.** Reference matmul is for correctness. Compare against
optimized libraries before making speed claims. Release profile alone is not evidence.
