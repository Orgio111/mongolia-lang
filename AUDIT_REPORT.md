# Independent audit of the Mongolia v0.1.0 foundation

**Decision.** The source package passes the Linux foundation gate. It implements a bilingual, statically checked CPU interpreter. A public stable release still needs cross-platform execution evidence, terminology review, project ownership and release provenance. The supplied expansion Part 6 is audited. The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) records all 50 official areas against this snapshot.

## Chain of evidence

The input was the actual Phase 7 archive with SHA-256 `09168b5f85889a3a492d56cc78d068740fbf38ff7a5e86180effdb642828fa3c`. The extracted files were checked against their manifest. This audit examined both Cargo workspaces and lockfiles, all compiler and CLI modules, test fixtures, grammar/specification, ADRs, roadmap, release policies, archived build evidence, workflows, executable examples and future showcases. A separate build directory was used for the corrected release. The package inventory and hashes distinguish new, changed and inherited files.

## Findings and resolution

| Finding | Impact | Resolution and evidence |
|---|---|---|
| Unknown option passed to init became the scaffold directory | A typo caused an observable filesystem write and exit 0 | Reject option-like directory operands before filesystem access. Old binary observation and CLI regression recorded. Explicit relative paths remain usable. |
| Diagnostic source excerpts contained raw terminal controls | Displayed text could obscure the diagnostic position | Render controls and bidi formatting marks inert in stderr excerpts. Tab occupies one displayed scalar. Original byte span, localized message and normal program output retain their contracts. Regression tested in both catalog modes. |
| Duplicate tensor dimension product check in the parser | Large valid dimension tokens could take different failure stages/codes depending on host pointer width | Checker owns shape product/element cap. Parser owns token validity and single-dimension bounds. Linux regression fixes the phase contract; 32-bit execution remains outstanding. |
| Runtime cloned arbitrary-size literal strings on evaluation, and copied static identifier text into every invocation frame | Avoidable heap allocation under repeated calls. The fuel model charges variable reads, while literal evaluation has constant fuel cost | Literals borrow immutable AST text; variable string reads still charge bytes and create an owned copy. Frames borrow names. The checked AST outlives execution. String return/call and work-budget tests pass. |
| Packaged performance protocol had no measured baseline | No repeatable basis for throughput or memory comparisons | Added a release-only stage probe and a Linux harness. Percentiles, raw samples, BLAS and scalar references shipped. |
| Root LICENSE path and full mapping in llms.txt absent | Required entry points needed additional lookup | Added license-selector file pointing to both complete texts and inlined all reserved pairs in assistant guidance. |
| CI did not explicitly build release; user-facing download instructions named an earlier archive | Regressions in release builds and stale installation guidance | Updated workflow and English/Mongolian instructions. Local release build and isolated install verified. Hosted workflows remain unobserved. |

## Compiler and runtime inspection

The two workspace members form an acyclic graph. The lexer consumes UTF-8 on scalar boundaries and tracks byte offsets into original text. The line-comment byte scan only resumes tokenization at ASCII LF or EOF. Identifier normalization precedes keyword/identifier lookup. The Pratt parser bounds recursive entries and work per statement, consumes an EOF-terminated stream, and produces row-major rank-2 literal syntax. A single private tokens-to-AST entry point supports both parsing and isolated timing.

The checker first collects function signatures, then checks all bodies, including unused ones. Environments are local to a function. Return types, arity, duplicate declarations, unbound names, unreachable statements and numeric operand domains are checked. Matmul contracts the inner dimensions and validates output area before evaluation. The interpreter accepts a borrowed, immutable checked program. It uses checked i64 arithmetic, bounded work/depth/output, fallible tensor storage, deterministic left-to-right evaluation and increasing-k f64 accumulation. Only successful runs publish buffered output.

The tests include all five acceptance programs, localized shape/position goldens, exact arithmetic properties, malformed UTF-8, invalid programs, Unicode normalization and budget boundaries. The external rational oracle tested 64 compatible and 32 incompatible matrix pairs against both build profiles. The installation test exercises help, version, init and all example files with the installed release binary.

## Duplication and remaining design debt

- The source excerpt renderer lives in the CLI while localization lives in core. They serve different callers. The CLI repeats the source byte cap to bound file reading before decoding. A future public limit definition would reduce drift.
- Canonical spellings appear in lexer, reservation predicate, EBNF, spec and assistant guidance. The new all-alias test covers 30 pairs. An RFC-generated vocabulary table is appropriate when native review freezes the list.
- Shape product ownership was consolidated. The engine keeps a runtime check as protection at the allocation boundary.
- Historical design, policy and benchmark documents overlap. Retained historical reports preserve provenance. The current points of entry are SPEC.md, PHASE_8_REPORT.md and BENCHMARK_BASELINE.md. There are no unwired placeholder backend crates.
- Some errors label a whole function or initializer because the AST lacks declaration-level spans. Mongolian detail fallback, extended Unicode handling, structured diagnostics and editor coordinates remain open.
- CLI init uses exclusive file creation but has no multi-file transaction. See KNOWN_LIMITATIONS.md for interrupted writes and concurrent directory changes.

See [requirements](REQUIREMENTS_MATRIX.md), [security](SECURITY_AUDIT.md), [evidence](verification/phase8/commands.json) and [measurements](BENCHMARK_BASELINE.md).
