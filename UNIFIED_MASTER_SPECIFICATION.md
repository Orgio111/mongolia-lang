# Mongolia unified master specification v3.0

**Current implementation:** compiler 0.1.2, specification 0.1 revision 1. Phase 10 delivered release engineering; proposed HIR is scheduled for Phase 11. Architecture edition v3.0 remains the long-term design.

**Architecture edition v3.0. Executable compiler v0.1.1.** The expansion requirements and dependency gates below were adopted at v0.1.0. The subsequent Phase 9 directive selected foundation stabilization. Gate H (resolved HIR) remains a proposed increment after stabilization. The current decision, verification and handoff are in [PHASE_9_REPORT.md](PHASE_9_REPORT.md) and [PHASE_10_READINESS.md](PHASE_10_READINESS.md).

## Authorities and release decisions

| Question | Controlling artifact |
|---|---|
| What does this compiler accept and execute? | [SPEC.md](SPEC.md), [EBNF](grammar.ebnf), and the [acceptance corpus](docs/PHASE_2_ACCEPTANCE.md). A disagreement between implementation and published behavior is a tracked defect pending decision. |
| What has actually passed? | [Phase 8 report](PHASE_8_REPORT.md), [integration verification](verification/integration/linux.log), and the recorded host/command details. |
| What does the project intend to build? | [Integrated requirements](EXPANSION_REQUIREMENTS_MATRIX.md), [proposed architecture](UNIFIED_ARCHITECTURE.md), [decisions](ARCHITECTURAL_DECISIONS.md), and [priority](IMPLEMENTATION_PRIORITY_MATRIX.md). |
| When can it ship? | [Gate dependencies](FEATURE_DEPENDENCY_GRAPH.md), [master roadmap](MASTER_ROADMAP.md), [performance targets](PERFORMANCE_TARGETS.md), and [risks](RISK_REGISTER.md). |
| What work is authorized by the next phase plan? | [Phase 9 execution plan](PHASE_9_EXECUTION_PLAN.md). A proposal's presence in this specification establishes scope for review, not delivery authority for every feature. |

## Observable semantics

A compilation accepts source bytes, edition and diagnostic configuration. For the current CLI the edition is implicit and there is a single source file. Decode UTF-8 before lexing. Normalize identifiers to NFC for lookup, canonicalize keyword aliases, and keep original byte intervals for diagnostics. The v0.1 lexical policy uses XID and a restricted Latin/Cyrillic spoof check. Distinct user-defined names are never translated between languages.

Resolve functions across the entire file. Within each function, declare parameters first, check each let initializer before inserting its immutable binding, and type-check every statement in source order. All function bodies must validate before any program output. Calls and variable reads have separate name spaces. English and Mongolian spellings produce equal semantic operations for corresponding programs, modulo origins and binder renaming. Diagnostics render against each program's original bytes.

For a backend and supported profile, the observable result consists of validation outcome, first language error and origin, exit status and ordered program output. Runtime errors discard buffered output. Changes to internal names, evaluation storage or instruction scheduling must preserve that contract. Within the foundation profile, fuel and depth accounting, output limits, signed zero, integer traps and formatting are observable. Changing the order of two failing expressions can change the reported error. Preserve operand and call-argument order even where algebra permits rearrangement.

| Domain | v0.1.0 contract | Future extension condition |
|---|---|---|
| Values | i64, f64, bool, UTF-8 string, unit and rectangular rank-2 f64 tensors. Local inference is exact. Bindings are immutable. | Each new scalar/compound type defines construction, conversion, equality, layout, effects and error boundaries. |
| Arithmetic | Checked signed operations, truncating integer division, zero-divisor and overflow diagnostics. f64 follows specified IEEE behavior. | Specify dtype, accumulator, rounding, exceptional values and numeric mode for each new operation. |
| Tensors | Literal rows are nonempty, equal width and f64. Checked element/byte products, exact static annotations, equal-shape elementwise operations. For [M,K] @ [K,N] infer [M,N], reject mismatches before execution. | Introduce distinct symbolic binders and runtime dimension values. Unproved equality becomes an explicit guard ahead of dependent allocation/access. |
| Storage | Host row-major tensor values have value semantics. Variable reads are charged and copy. Independent calls get fresh frames and run budgets. | Immutable sharing or views require equal observables or a new explicitly specified ownership API. |
| Functions and effects | Direct calls, explicit returns, print. No mutation or implicit numeric promotion. Checked AST is the reference executable representation. | Track IO, traps, mutation, allocation, randomness, device and foreign effects before reordering or differentiating instructions. |
| Limits and diagnostics | Exact bounds, scalar display columns, original spans, stderr localization and exit 0/1/2 are defined in SPEC.md. | Additional profiles may state different quotas. Positions for protocol clients require negotiated encoding. |
| Hardware | Only audited interpreter host execution is evidenced. | Each target publishes precise ABI, dtypes, operations, tested device, numeric modes, allocation failures and synchronization rules. |

## Rules for promotion

1. An RFC identifies the grammar/type/effect changes, dependencies, compatibility impact and release owner. Existing programs receive explicit migration/edition treatment for observable changes.
2. A feature owner supplies a positive and negative example, an independent correctness fixture, and scoped performance/security tests. Proposed syntax lives in proposal material until the parser, validator and appropriate executor agree.
3. A backend proves parity with the reference profile before advertising support. Relaxed math requires a separate, explicit contract and error envelope. A strict backend must reject unsupported semantics or take a declared, user-authorized fallback.
4. The package/project, device, FFI, editor, application framework and compiler internals have independent version surfaces. Source edition, Rust toolchain, integration document edition and backend ABI version are recorded separately.
5. Public release needs named maintainers, verified names, reviewed bilingual terminology, release provenance and a supported-platform matrix. The [Phase 8 limitations](KNOWN_LIMITATIONS.md) remain applicable.

**Traceability.** The original mandate covers design and implementation across sections 0–30. Its foundation and future requirements remain individually inventoried. The expansion contributes 50 workstreams and 620 item-level contracts. The integrated design reuses their identities, evidence and acceptance fields. Optional product brands and research areas have no effect on the foundation's executable grammar.
