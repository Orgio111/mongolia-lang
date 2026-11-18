# Compatibility with v0.1.2

The parent source ZIP inventory passed verification. Its Git tree was 21dc90ea8b008ead24e6fd8c5027cbdc9b7fa147, identical to the Phase 10 tree imported on GitHub at 5332080fbffdbe0b9c6a5a168a0a46a9356e4cf7. The local reconstruction has independent commit metadata. The GitHub development branch descends from the actual main commit.

scripts/phase11_compat.py ran both release CLIs on the same paths: all 11 old example files, 11 additional valid/error/malformed fixtures, and 20 generated bilingual tensor programs. For each input it compared check and run in English and Mongolian. All 168 comparisons agreed byte-for-byte on stdout and stderr, including the numeric exit status. See verification/phase11/compatibility.json. The independent dyadic matrix oracle passed 192 debug/release CLI invocations. Newly valid explicit cast programs are intentionally outside the old grammar.

The checked Rust API retains check, validate and run and adds hir_dump(). The public AST enum gains a Cast variant; Rust clients exhaustively matching ExprKind need a new arm. Programmatically constructed AST spans must be ordered within their function for HIR verification. The numeric runtime rules for older source are preserved. Neither the internal HIR representation nor its text format is a stable serialization ABI.
