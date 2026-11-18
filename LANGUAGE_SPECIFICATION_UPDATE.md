# Development specification 0.2 revision 0

The new primary expression is an i64 or f64 type name followed by exactly one parenthesized expression. The canonical aliases бүхэл64 and бодит64 work in the same positions. Conversions evaluate their operand once, consume an ordinary expression fuel unit and use exact scalar types. See TYPE_SYSTEM_SPECIFICATION.md for rounding, bounds and static/runtime errors. There is no implicit promotion.

Run examples/conversions.mn and examples/conversions_mn.mn. Both convert 3.9 to integer 3, convert it back to 3.0 for a tensor cell and print [[6.0]]. Existing type, shape, precedence, buffering and evaluation rules in SPEC.md continue to apply. The grammar change uses two previously reserved words in primary position. No previously valid identifier acquires a new meaning.
