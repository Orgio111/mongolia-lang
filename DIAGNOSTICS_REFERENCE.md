# Semantic diagnostics

| Code | Stage and origin | Repair |
|---|---|---|
| E_NAME / E_DUPLICATE | Unresolved read or callee at its expression; duplicate declaration in its local/function namespace | Check normalized spelling, declaration order for locals and function scope. |
| E_MAIN / E_ARGUMENTS | Invalid entry or wrong arity; call diagnostics include expected and actual counts | Declare main() with unit result; match the declared signature. |
| E_TYPE | Wrong argument, annotation, return operand or arithmetic domain | Match exact types; use an explicit numeric conversion when appropriate. The catalog renders expected/actual types in both languages. |
| E_CAST | Non-numeric conversion operand, with source and target types at the conversion expression | Use an i64 or f64 operand. Монгол: бүхэл64 эсвэл бодит64 төрлийн утга хэрэглэнэ үү. |
| E_CAST_RANGE | Runtime f64 to i64 conversion outside its finite representable interval | Constrain the input and check division or NaN-producing arithmetic. |
| E_SHAPE / E_LIMIT / E_ALLOCATION | Ragged rows, inconsistent matmul inner dimensions, bounded work/depth or allocation failure | Read both reported shapes; reduce size/work or correct the contraction. |
| E_RETURN / E_UNREACHABLE | Missing result or statement after return | Supply a matching return and remove unreachable statements. |
| E_SCRIPT / E_CONFUSABLE / E_UTF8 | Identifier security checks or invalid source bytes | Use consistent scripts per identifier and valid UTF-8. |
| E_HIR | Internal structural verifier failure with the nearest retained origin | Record compiler version and minimal input for a compiler bug report. |

CLI --lang mn selects the draft Mongolian catalog. Original UTF-8 byte spans drive scalar-based CLI columns. All source phases run before interpretation. A runtime trap discards buffered program output. SPEC.md and SECURITY.md describe the remaining lexical/runtime categories and limits.
