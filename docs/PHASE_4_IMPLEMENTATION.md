# Phase 4 · Mongolia v0.1.0 implementation

The Phase 3 archive supplied the working CPU interpreter and the Phase 2 language contract. This phase connects checking and execution through an immutable checked-program handle and enforces the output limit during rendering. The package version remains 0.1.0.

| Stage | Source under crates/ | Responsibility |
|---|---|---|
| File input and CLI | mn-cli/src/main.rs | Read bounded source bytes, report the UTF-8 error offset, select diagnostic language and command |
| Lexer | mn-core/src/frontend/lexer.rs | Produce canonical bilingual words and original byte spans |
| Syntax tree | mn-core/src/frontend/ast.rs | Own functions, statements, expressions and declared types |
| Parser | mn-core/src/frontend/parser.rs | Apply precedence, parse functions, rank-2 literals and tensor annotations |
| Semantic checker | mn-core/src/engine/checker.rs | Resolve signatures and locals, check every function, validate return types and shapes |
| Tensor size and allocation | mn-core/src/engine/tensor.rs | Check positive extents, result sizes, contraction dimensions and byte counts |
| Reference interpreter | mn-core/src/engine/runtime.rs | Execute expressions and calls, run row-major tensor kernels, account for work, buffer output |
| Public interfaces and diagnostics | mn-core/src/frontend.rs, engine.rs, diagnostics.rs | Expose syntax, checking and execution APIs and render localized diagnostics |

## Execution contract

Parsing yields an owned AST. Validation borrows it and returns a CheckedProgram after checking all bodies, including unused functions. The handle owns the validated signature table and holds shared references to the functions. Rust prevents mutation of the borrowed syntax while the caller can still execute the handle. Clients can invoke checked.run() repeatedly. Each run creates new frames, a fresh work counter and a fresh output buffer. The existing check and run entry points preserve their signatures.

Shape checking and runtime matrix multiplication use the same contraction rule. The checker rejects incompatible operands before main begins. Before allocating a result, the interpreter checks element count, work and byte size. Printing appends delimiters, scalar text and the final newline against the remaining output budget. A failed run drops its buffered output.

## Verification and remaining work

The implementation passes the Phase 2 acceptance matrix, including both bilingual matrix outputs and the static shape failure. Additional coverage exercises checked handle reuse, Rust borrow enforcement, invalid external tensor annotations, tensor formatting at the output limit, localized input diagnostics and Unix file names containing non-UTF-8 bytes. The transcript is in verification/phase4/workspace.log.

The next verification phase can use this source as its input. Windows and macOS execution, sustained coverage-guided fuzzing, complete Unicode confusable coverage and native-speaker review remain open. AD and accelerator backends retain their future-design status.
