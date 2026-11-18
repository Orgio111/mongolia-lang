# Phase 2 · v0.1.0 specification and conformance

The normative deliverables are SPEC.md and grammar.ebnf. The Rust parser recognizes the syntax, the checker enforces the typing judgments, and the CLI provides the observable acceptance interface. The future grammar in docs/future-grammar.ebnf describes proposals.

The Phase 1 review identified a discrepancy in unary minus. The foundation specification now restricts unary minus to i64 and f64. Binary tensor subtraction accepts two tensors of identical shape. The test negation_and_subtraction_have_distinct_domains exercises both sides of this distinction and verifies the byte span of a rejected Cyrillic operand.

## Acceptance matrix

Run all test entries with `cargo test --workspace --locked`. Names in this table are Cargo test names. The new public-API conformance tests live in crates/mn-core/tests/phase2.rs. End-to-end tests invoke the mn binary from crates/mn-cli/tests/cli.rs.

| Requirement | Observable result | Executable evidence |
|---|---|---|
| A · English matmul | [[19.0, 22.0], [43.0, 50.0]] | acceptance_a_b_c_e |
| B · Mongolian matmul | Same numerical output and layout as A | acceptance_a_b_c_e |
| C · Mixed keywords | 30 | acceptance_a_b_c_e |
| D · Incompatible inner dimensions | check and run both exit 1 with empty stdout. Both languages name [2, 3] and [2, 2], explain 3 versus 2, and mark exactly A@B | shape_error_prevents_output |
| E · Typed call | 42 | acceptance_a_b_c_e |
| UTF-8 identifiers, NFC, aliases | Equivalent spelling resolves, normalized duplicate rejected, mixed keywords run | lexical_rules_and_normalized_scope, nfc, acceptance_a_b_c_e |
| Script policy | Mixed name and file-wide skeleton collision rejected | lexical_rules_and_normalized_scope |
| Invalid encoding | Exit 1 with E_UTF8 | invalid_utf8 |
| Syntax and missing delimiters | Compilation fails before execution | malformed, negative_programs |
| Function arity and undefined names | Invalid references and argument lists rejected, including unused bodies | errors, validate_unused_functions, negative_programs |
| Duplicate declarations | Same-scope rebinding rejected, including NFC-equivalent names | negative_programs, lexical_rules_and_normalized_scope |
| Invalid tensor literals and shape annotations | Ragged rows, integer cells, dimension zero, excessive product and wrong annotation rejected | errors, negative_programs, tensor_dimensions_and_result_capacity |
| Precedence and associativity | Multiplication before addition, equal-precedence division groups left, minus binds before division | precedence, arithmetic, minimum_integer_and_left_associativity |
| Zero division and overflow | i64 traps on zero and overflow, f64 follows IEEE arithmetic | checked_integers, minimum_integer_and_left_associativity, float_division_is_ieee |
| UTF-8 spans and localization | Half-open byte spans select original source; golden details match English and Mongolian messages | unicode_spans, negation_and_subtraction_have_distinct_domains, bilingual_diagnostic_catalog_matches_golden_files |
| CLI exit codes and effects | Success 0, source/runtime errors 1, unsupported command 2. check accepts a well-typed 1/0 without evaluating it | acceptance_a_b_c_e, validation_is_not_execution, unsupported_command |
| Forward calls, evaluation order, return | Separate name spaces, ordered argument effects, unreachable statement rejected | forward_calls_and_argument_effect_order |
| Work, output, source and token limits | Exact limit succeeds and limit plus one fails | exact_work_and_output_caps, exact_source_and_token_caps |
| Depth and allocation | Deep input, recursion and overflow return diagnostics | depth_limit, recursion_is_bounded, allocation_size_overflow_is_rejected |
| Independent numeric checks | Scalar checked arithmetic, small dot products, identity matrices | integer_arithmetic_matches_checked_reference, differential_small_matrices, tensor_identity_and_shape_properties |

## Phase outcome

The source baseline is the v0.1.0 foundation archive with SHA-256 6caee8f0cf1896684cc1b9a2adf510e87f2b7dbb7eb29ceadb148d4c63fa9980. Phase 2 revises the specification and EBNF, extends CLI acceptance D and adds the phase2.rs conformance tests. Compiler behavior and package version follow the foundation baseline. See STATUS.md and verification/phase2/ for the verification record.

Phase 2 closes when the normative subset agrees with the implemented operator domains, the matrix covers A–E and all additional mandatory cases, and the workspace passes build, test, formatting and lint checks. Terminology review, broader Unicode spoofing coverage and cross-platform release validation remain open. The next repository phase can reuse the two working crates and their lockfile.
