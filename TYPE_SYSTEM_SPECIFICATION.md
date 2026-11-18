# Development type contract

Local inference determines one closed type per expression. An annotation, argument or return imposes exact type equality. A function signature has ordered parameter types and one result. Functions resolve by declaration identity; first-class function values remain proposed. Scalars comprise i64, f64, bool, string and unit. Tensor types fix f64 and two positive static axes. Arithmetic, overflow and f64 order follow SPEC.md. Let introduces an immutable slot after its initializer succeeds.

The new primary expressions i64(e) and f64(e) accept the canonical English or Mongolian type spelling and exactly one numeric operand. Identity conversions preserve bits. Integer to binary64 rounds to nearest, ties to even. Binary64 to integer truncates toward zero and requires a finite value in [-2^63, 2^63). A bad source type yields E_CAST during checking. A bad value yields E_CAST_RANGE during execution before any buffered output is published.

## Proposed extensions

| Feature | Contract to review before syntax enters the language |
|---|---|
| Tuples and arrays | Structural, ordered products; homogeneous arrays carry a checked length. Test singleton/unit distinction and indexing errors. |
| Structs and enums | Nominal declaration IDs, uniquely named fields or alternatives. Reject infinite-size recursive layouts. Match must cover every alternative. |
| Aliases | Transparent expansion under a cycle detector and depth budget. |
| Option and Result | Ordinary parameterized enums. Test exhaustive matching, payload identity and propagation before stabilization. |
| Generic inference | Instantiate fresh variables per call; unify equal constructors recursively. Reject arity mismatches and occurs-check cycles. Bound solver work and report unresolved variables at the call site. |
| Mutation | Writable places require explicit declaration permission and alias checks. The current grammar only binds immutable names. |
