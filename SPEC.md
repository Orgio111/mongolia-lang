# Mongolia v0.1 language specification

For compiler 0.2.0-dev.1 the normative LANGUAGE_SPECIFICATION_UPDATE.md extends the foundation below. grammar.ebnf includes explicit numeric conversions.

**Specification 0.1, revision 1.** Compiler 0.1.2 implements the same source language and observables as compiler 0.1.1. The historical foundation rules below are normative. The compiler and specification use separate version identifiers. See [VERSIONING.md](VERSIONING.md).

This document defines the implemented **v0.1 subset**. It is not a finalized specification for
all future Mongolia features. See docs/FUTURE_DESIGN.md for proposals and BUILD_REPORT.md for
verification. Vocabulary is provisional pending native-speaker review. `grammar.ebnf` is the
machine-readable grammar artifact; the Rust parser and conformance fixtures execute this subset.

## Lexical rules

Source must be UTF-8. Input is limited to 1 MiB and 100,000 tokens. Identifier start is underscore
or Unicode XID_Start; continuation is underscore or XID_Continue. Names normalize to NFC before
lookup and duplicate detection. Spans are half-open byte ranges into the original UTF-8 source,
never normalized text. The normalized identifier must not combine a Latin character and a Cyrillic character. Use the Unicode 17 Script property for Latin and Cyrillic, and also count every code point in the legacy U+0400–U+052F range as Cyrillic. This check uses the primary script property. Mixed
English/Mongolian keyword syntax remains legal. A conservative file-wide Cyrillic/Latin confusable
skeleton check rejects selected collisions. It is not a full Unicode UTS39 implementation and may
have false positives or miss attacks. Unicode versions follow locked dependency versions.

Whitespace separates tokens. `//` comments end at newline. `/* ... */` comments nest; unclosed
comments error. Documentation comments currently have ordinary comment semantics. Reserved words
are recognized after normalization and aliases map to a single canonical token. Future reserved
words do not imply implemented syntax.

| English | Mongolian | v0.1 role |
|---|---|---|
| fn | функц | Function declaration |
| let | хувьсагч | Immutable binding |
| return | буцаах | Return statement |
| print | хэвлэх | Print statement |
| true / false | үнэн / худал | Boolean literals |
| i64 / f64 | бүхэл64 / бодит64 | Supported scalar types |
| bool / string | логик / мөр | Supported scalar types |
| Tensor | Тензор | Supported rank-2 f64 tensor type |
| mut / const | өөрчлөгдөх / тогтмол | Reserved future syntax |
| if / else | хэрэв / үгүйбол | Reserved future syntax |
| loop / while / for / in | давтах / зуур / бүр / дотор | Reserved future syntax |
| break / continue | зогсох / үргэлжлэх | Reserved future syntax |
| struct / enum / trait / impl | бүтэц / тоочилт / шинж / хэрэгжүүлэх | Reserved future syntax |
| import / pub | импорт / нийтийн | Reserved future syntax |
| grad | уламжлал | Reserved future syntax |
| f32 / i32 | бодит32 / бүхэл32 | Reserved unsupported types |

Integer literals are decimal digits with no separators or exponent. Positive literals fit i64;
unary minus admits the minimum i64 value. Floating literals require digits on both sides of a
decimal point; exponent notation is not implemented. No implicit integer-to-float promotion.
Strings are UTF-8 between double quotes. Supported escapes are backslash, quote, `\n`, `\r`,
`\t`; raw newlines and unknown escapes are errors. Boolean literals have type bool.

## Grammar and binding

A compilation unit consists solely of function declarations. Each function has a unique normalized
name, explicitly typed parameters, an optional return type (omission means unit), and a statement
body. Function order does not affect calls. `main` must exist, accept no parameters and return unit.
There are no module imports, generics, closures, overloads or nested declarations.

Statements are immutable let declarations, print, return and expression statements. A binding's
initializer is checked before adding its name to the environment. Optional annotations must match
the inferred type exactly. Duplicate declarations and parameters are errors. Function locals do
not escape their invocation. No assignment, control flow or implicit last-expression return exists.
Non-unit functions require a valid explicit return. `return;` returns unit. Calls check every
argument's exact type and argument count. Every function body is statically validated, including
functions not reachable from main, before runtime output begins.

| Binding strength (higher first) | Operators | Associativity |
|---|---|---|
| Primary | literals, variables, named calls, parenthesized expressions, tensors | Structural |
| 30 | unary `-` | Right through recursive prefix parsing |
| 20 | `*`, `/`, `@` | Left |
| 10 | `+`, `-` | Left |

Thus `a + b * c` means `a + (b * c)`, and `a @ b @ c` means `(a @ b) @ c`.
The parser limits expression nesting to 128 and expression entries to 256 per statement.
Trailing commas are not supported. Braces delimit function bodies; semicolons terminate statements.

## Scalar semantics

Implemented types are i64, f64, bool, string and unit `()`. i64 arithmetic uses checked operations:
overflow, integer division by zero, and MIN/-1 division fail with a runtime diagnostic. Integer
division truncates toward zero. Floating arithmetic uses Rust f64/IEEE behavior, including infinity
and NaN on relevant operations; floating division by zero is not an integer-style trap. No
fast-math reassociation is permitted. Scalars do not implicitly convert. Boolean/string arithmetic
is invalid. Equality, ordering, casts, arrays, references and collection APIs are not implemented.

## Tensor semantics

A tensor literal is a nonempty outer list of nonempty equal-length rows of f64 expressions.
Integer cells are not implicitly converted. Rank is exactly two. `Tensor<f64,[M,N]>` (and
corresponding Mongolian aliases) requires literal positive dimensions, each at most 1,000,000;
checked product and runtime allocation limits still apply. Empty and ragged literals error.
Tensor shape is inferred statically; an annotation must exactly match element type and shape.

For `A: Tensor<f64,[M,K]>` and `B: Tensor<f64,[K,N]>`, `A @ B` has
`Tensor<f64,[M,N]>`. Otherwise checking fails, reporting both shapes and incompatible inner
sizes at the expression span. Runtime must not start when any static error exists. Matmul is
row-major reference accumulation using f64. There are no views, devices, broadcasting, dynamic
or symbolic dimensions in v0.1. Tensor handles are implementation details, not a borrowing model.
Tensor `+`, binary `-`, `*` and `/` operate elementwise on two equal-shape tensors.
Unary `-` accepts only i64 and f64. Applying it to a tensor produces E_TYPE during checking. Scalar/tensor mixing is invalid. Floating division follows f64 semantics.
Values are deep-copied on environment lookup; there are no exposed shared views.
Allocation element/byte calculations must be checked before allocating. The runtime limits a
tensor to 1,000,000 elements, evaluation to 1,000,000 steps and enforces the shared depth counter described below. Output is buffered
up to 1,000,000 bytes and emitted only after successful completion. These bounds are implementation
resource policies, not a security sandbox or a total process-memory guarantee.

## Typing rules

Write `Γ ⊢ e : τ` for expression typing. Literals have their lexical types. Variable lookup requires
`x:τ ∈ Γ`. For `let x[:τ] = e`, infer e under Γ, check optional annotation, then extend Γ.
For f:(τ1,...,τn)->τ, application is valid iff argument count is n and Γ⊢ei:τi for all i.
For `+ - * /`, both operands must have the same supported scalar numeric type, or equal-shape f64 tensor type; result preserves it.
If Γ ⊢ e : τ and τ ∈ {i64, f64}, then Γ ⊢ −e : τ. Tensor construction verifies every cell is f64 and all row
lengths agree. Matmul uses the contraction equality above. Return matches the enclosing result
exactly. Print accepts a supported value and has unit statement semantics.

Errors carry stable category codes, byte spans and detail; localized rendering is a CLI concern.
Mongolian diagnostics may retain English technical detail where translation is not yet complete.
Source display must derive positions only at UTF-8 boundaries. Negotiated LSP encodings are future
work; a terminal source column is not automatically an LSP position.

## CLI contract

Supported commands are `mn run FILE`, `mn check FILE`, `mn init [DIRECTORY]`, `mn --help`, `mn --version`.
`--lang en` and `--lang mn` select diagnostics. `check` validates without execution; `run` validates
then executes main. Future commands such as build/fmt/add must fail explicitly. See README.md and
CLI integration tests for exact argument and exit-code behavior. There is no native build target,
package resolver, machine-readable diagnostic mode or stable compiler ABI in this subset.

## Normative details for the v0.1.1 foundation

The original source bytes determine diagnostic spans. Identifiers compare by case-sensitive NFC
text. For the file-wide collision check only, map each character in `АВЕКМНОРСТХаеорсух`
to the corresponding character in `ABEKMHOPCTXaeopcyx`. Other characters map to themselves.
Two distinct normalized non-keyword names with the same resulting string cause E_CONFUSABLE,
including across function scopes. Repeated occurrences of the same name are accepted lexically.
The scope checker separately detects redeclarations. Unicode data comes from the locked Rust
dependencies. Whitespace follows Rust char::is_whitespace. Line comments consume through LF
or end of file. Comments and whitespace may separate any tokens.

A decimal token with magnitude 9223372036854775808 immediately following prefix minus denotes i64::MIN, including when written with leading zeroes.
The unnegated magnitude and that magnitude enclosed in parentheses exceed the positive range.
Decimal float literals must round to a finite f64. Runtime operations can produce infinities,
signed zero and NaN. Integer division truncates toward zero and MIN / −1 raises E_OVERFLOW.

Functions and variables have independent lookup tables. Expressions in call position name a
function. Arguments evaluate left to right in the caller before the callee receives its frame.
Binary operands evaluate left to right. Statements execute in source order. A return terminates
the invocation. A statement following a return is a static error even in an uncalled function.
A unit-returning call is a value of type (). There is no standalone unit literal in expressions.

For an m-by-k times k-by-n product, the checker verifies that m*n fits the element cap before
execution. For each output cell the interpreter starts at +0.0 and accumulates products in
increasing contraction-index order. It checks the work cost before allocating the output.
Tensor reads return independent row-major buffers. String reads copy the UTF-8 bytes.
A shape annotation is an exact type assertion. Elementwise shape differences are E_TYPE,
incompatible inner dimensions for @ are E_SHAPE, and ragged literals are E_SHAPE during parsing.

### Foundation limits and observables

| Observable | Reference rule |
|---|---|
| Source and tokens | At most 1,048,576 source bytes, at most 100,000 non-EOF tokens. Comments and whitespace do not count as tokens. |
| Parser expression budget | Reset per statement. Charge each entry to the expression parser, including grouped subexpressions and right operands. Accept at most 256 entries and 128 simultaneously active entries. |
| Static expression depth | Inference starts at depth 0 and rejects a subexpression at depth greater than 128. |
| Runtime depth | main enters call depth 0. Evaluation of a statement operand, descent into a subexpression, and entering a call from its call expression each increase depth by 1. Reject call entry at depth ≥128 or expression entry at depth >128. |
| Work | Initial budget 1,000,000 per run. Charge 1 per call, statement and expression. Additionally charge element count for a tensor variable read or elementwise operation, UTF-8 byte count for a string variable read, and m*k*n for matmul. A charge may consume the final unit. |
| Tensor storage | Each positive dimension is at most 1,000,000. The checked element product is at most 1,000,000. Check the byte product before allocating f64 storage. |
| Printing | Append the rendered value and one LF. Strings emit their contents, bools use true/false, unit uses (), integers are decimal, floats use Rust f64 Debug formatting. Format tensor rows and cells with brackets and comma-space separators. |
| Output limit | 1,000,000 UTF-8 bytes including LF characters. Flush program output only on success. Any runtime error discards earlier print output. |
| Diagnostics | First reported error terminates the command. Display one-based line and Unicode scalar column. Underline the first line of the original span with 1–80 carets. Malformed UTF-8 reports a zero-based byte offset before parsing. |
| Exit status | 0 success, 1 source/check/runtime error, 2 command usage or file I/O error. Successful check emits no program output. |

For Mongolian diagnostics the catalog translates known exact and parameterized messages.
The catalog includes selected confusable collisions, resource limits and allocation failures. Untranslated details fall back to English under the localized category. The CLI localizes the byte offset for malformed UTF-8 and the source size limit.
File paths use the host OS representation. Source text uses UTF-8. The golden cases fix selected localized messages.
Native-speaker review remains open.

Diagnostic excerpts display a tab as one space and replace control characters and Unicode
bidirectional formatting controls and U+2028/U+2029 with U+FFFD. When the column exceeds 161, show the final 80 preceding scalars. Display up to 160 scalars from the error position and up to 256 scalars of detail. Ellipses indicate omitted text. Line/column and caret counts still refer to
the original Unicode scalars. `mn init` rejects unknown option-like arguments with status
2. Use `./-name` for a relative directory whose name starts with a hyphen.

A mismatched return value labels its expression. The public AST validation API also rejects nonfinite floating literals. In a trusted project directory, init uses exclusive creates, synchronizes each file and attempts to remove its newly created files after an ordinary I/O error. The new directory itself can remain. Power failure and hostile concurrent directory changes are outside this guarantee.
