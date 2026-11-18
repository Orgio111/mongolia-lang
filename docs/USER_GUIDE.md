# First programs, tensors and diagnostics

From the source directory with mn installed, run `mn run examples/hello.mn`. Run `mn run examples/hello_mn.mn` for the Mongolian version. Both are executable source files. [Mongolian quick start](GETTING_STARTED_MN.md) · [Installation](../INSTALLATION_GUIDE.md).

## Syntax reference

| Construct | English | Монгол | Constraint |
|---|---|---|---|
| Function | fn | функц | Typed parameters, optional result after ->. main takes no parameters and returns unit. |
| Local binding | let | хувьсагч | Immutable. An annotation must match the inferred type. |
| Return | return | буцаах | A non-unit function must return its declared type. |
| Output | print | хэвлэх | Statement with one expression in parentheses. |
| Scalar types | i64, f64, bool, string | бүхэл64, бодит64, логик, мөр | No implicit numeric conversion. |
| Rank-2 tensor | Tensor | Тензор | f64 cells, positive static dimensions. |

Run `mn run examples/functions.mn` and `mn run examples/functions_mn.mn` to see two typed versions of addition produce 42. The mixed-syntax example produces 30. Operators *, / and @ bind more tightly than + and -. Parentheses override precedence. Statements end with a semicolon. The exact grammar and numeric limits are in [SPEC.md](../SPEC.md).

## Tensor programming

Open examples/matmul.mn: A has two rows and two columns, B has two rows and two columns, and A @ B produces [[19.0, 22.0], [43.0, 50.0]]. The Mongolian equivalent is examples/matmul_mn.mn. First use `mn check examples/matmul_mn.mn` to validate shapes, then `mn run examples/matmul_mn.mn` to compute the result. To inspect a shape error, run `mn check examples/negative/shape_mismatch_mn.mn --lang mn`. The left inner dimension is 3 and the right inner dimension is 2. You should see E_SHAPE, a source location and exit 1.

## Troubleshooting and FAQ

- **Why did check succeed on a division by zero?** The integer expression can be well typed. The interpreter detects a zero divisor during run. A runtime error discards buffered program output.
- **Why does a tensor of 1 and 2 fail?** Tensor cells require f64. Use 1.0 and 2.0. Rows must have equal length.
- **Why is my name rejected?** Put Latin and Cyrillic in separate identifiers. E_CONFUSABLE can also flag similar names in separate scopes. Mixed-language keywords are supported in one file.
- **Why does init return 2?** Choose a writable, trusted directory without an existing mn.toml or main.mn. Exit 2 also covers I/O and command usage. Exit 1 covers source or evaluation errors.
- **Why is mn missing after installation?** Invoke the installed binary by its full path, then add its parent directory to PATH. Check OS/architecture and the binary SHA-256 if execution fails.
- **Can mn build a native application or use a GPU?** The available commands are run, check, init, --help and --version. The shipped binary contains the CPU AST interpreter. Future examples carry a .proposal suffix.
