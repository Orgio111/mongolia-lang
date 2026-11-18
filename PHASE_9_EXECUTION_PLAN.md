# Phase 9 execution plan · resolved HIR for v0.1 programs

**Archived integration proposal, written against v0.1.0.** The subsequent Phase 9 mandate selected foundation stabilization. Gate H is deferred. The implementation record is [PHASE_9_REPORT.md](PHASE_9_REPORT.md). The proposal below is retained for the next architecture review.

**Deliver gate H.** Convert the validated, single-file v0.1 program to a private high-level intermediate representation in which every call identifies a function and every variable read identifies a frame slot. Run the verifier through the normal check/run validation path. Keep execution on the current AST oracle until a separate VM milestone can prove parity. This document is a ready-to-execute plan. The integration package contains the v0.1.0 compiler.

## Starting point

Work from the current integrated archive and its SHA256SUMS. Read SPEC.md, the Phase 8 audit, and the existing checked API before editing. Core already collects forward-call signatures and checks every body. Let initializers are inferred before a binding enters the environment. Expr and Function carry byte spans; Stmt carries operands without a separate statement span. The checked handle borrows immutable syntax. Today the checker returns expression types during inference and does not retain a typed HIR.

| Increment | Expected changes in phase 9 | Exit criterion |
|---|---|---|
| 9.1 · Freeze observables | Record baseline CLI outcomes with both languages and both profiles. Add fixtures for a forward sibling call, self-recursion resolution, NFC names, distinct variable/function namespaces, and errors in an unused function. | Baseline outputs, first code, origin span, stdout/stderr and exit agree with published semantics. |
| 9.2 · Function identities | Add a private core HIR module. Assign dense function IDs in source order during signature collection. Bind call targets by canonical name. Keep the declared entry function ID. | Source-order IDs are stable for repeated checks of one source revision, independent of hash iteration. Forward and recursive calls resolve; duplicate functions still report the original error first. |
| 9.3 · Local identities and types | Assign parameter slots in parameter order and let slots after successful initializer/type validation. Reuse checked expression types as facts. Lower all literal, read, call, binary, negate and tensor expression forms and all four statement forms. | Every accepted read has one in-frame initialized slot and type. Tensor nodes preserve shape and ordering. Rejected bindings keep the previous error order and span. |
| 9.4 · Verify and integrate | Retain the resolved program privately on the validated handle. Verify indices, per-function slot/type domains, call arity/signatures, entry point, return types, tensor products and original origins. Keep AST execution accessible through existing public APIs. | Existing mn check and mn run both traverse validation and HIR verification. Malformed synthetic HIR is rejected via Result. No public flags, syntax or crate are needed. |
| 9.5 · Prove parity and package | Run existing suites, new private verifier tests, bilingual structural snapshots and paired performance checks. Record target/toolchain, diffs, first failure and archive digest. | Gate H sign-off by frontend and QA reviewers. Defer VM-specific performance claims until a VM executes the same corpus. |

## Representation and verifier contract

Use distinct private FunctionId and LocalSlot types. Each function stores parameter types, return type, slot type table and a statement sequence. References to a function point into the current program. Slots are meaningful only in the current frame. Carry a function owner on slot references during verification, or enforce this distinction by construction in frame-scoped builders. Store source origins with operations; preserve literal values and operator order, including whether a value can trap. Compare finite f64 literal payloads by bits in canonical snapshots so signed zero survives. Keep diagnostic spellings in an origin/name side table where needed.

A verifier starts each frame with parameter slots initialized. For each let, validate its initializer against the previous slot state and then mark precisely the declared slot. For each call, check callee bounds, arity and operand types without recursively traversing the callee; recursion is legal and runtime depth limits remain effective. For binary operations, reuse the same type/shape rules or consume checked proofs with an independent invariant check. Validate result type, original error site and execution order for return and print. Keep inference and lowering in a single checked traversal or pass checked facts to the lowerer. Avoid a second public type checker.

For parsed source, every source-backed origin must refer to the appropriate source revision and valid byte interval. Externally assembled ASTs are supported by the current public API without a source string. Preserve their available spans and validate structural/semantic invariants without assuming source bytes exist. Whole-function diagnostic spans stay whole-function unless a separately reviewed diagnostic change narrows them.

## Concrete tests and boundaries

- Lower the existing bilingual add program. Both snapshots resolve the same add function ID and two parameter slots, and the existing CLI prints 42. For equivalent English/Mongolian sources compare the operational payload after removing spellings and locations; test each location separately against its own source.
- Lower a local with the same spelling as a function, a forward call and a recursive call. Resolve each expression according to its syntactic namespace. An undefined name in an unused function still prevents any stdout.
- Compare negative shape checks for [2,3] @ [2,2] with the existing golden error and underline in both languages. Include ragged literals, scalar/tensor mismatches, duplicate declaration, malformed syntax, invalid UTF-8, integer overflow and zero divide. Static-only checking must still accept well-typed runtime traps.
- Construct invalid private HIR in tests: unknown callee, out-of-range/wrong-frame slot, read before initialization, mismatched result type and incompatible tensor product. Require deterministic verifier failures without panic. Keep the compiler's externally visible language errors on their original paths.
- Run exact threshold fixtures for source, tokens, expressions, runtime depth/fuel and buffered output. Test multiple runs of one immutable checked handle; each run gets independent state. Compare old/new construction and checking costs at the same input size and host configuration.
- Preserve current external entry points for frontend parsing, check, validate, run and CheckedProgram::run. Start in crates/mn-core; touch mn-cli only if a demonstrated compatibility issue requires it. A test-only canonical dump is sufficient for snapshot comparison.

## Reproduction and handoff

From the repository root, use the pinned rust-toolchain.toml and an external target directory if desired. With Cargo's default target directory:

```sh
cargo build --workspace --locked
cargo build --release --workspace --locked
cargo test --workspace --locked
cargo test --release --workspace --locked
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
python3 scripts/verify.py
python3 scripts/check_matmul.py target/debug/mn target/release/mn
```

Record each exit status and preserve failed fixtures before changing behavior. The last command is written for Linux. Use the platform executable names on Windows. For check/RSS regressions follow PERFORMANCE_TARGETS.md. Deliver source, verifier, tests, original/candidate evidence, limitations and the new package checksum. A behavior difference blocks routing the public path through the new representation until reviewed. Keep the last verified archive usable during investigation.

**Next boundary.** After gate H, gate V can lower function IDs and local slots to typed register instructions. Symbolic shapes, public modules, GPU work and language-level AD have later prerequisites in the dependency graph.
