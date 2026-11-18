# Implementation status after Phase 8

**Current patch level is v0.1.1.** See [Phase 9](PHASE_9_REPORT.md) for new verification. The remainder records the Phase 8 capability inventory.

| Classification | Capabilities | Verification boundary |
|---|---|---|
| Implemented and verified | Locked stable-Rust workspace, mn run/check/init/help/version, canonical EN/MN keywords, NFC/XID, byte spans, typed functions, scalar expressions, immutable bindings, checked i64, IEEE f64, strings, booleans, unit, rectangular f64 tensors, static type/shape checking, elementwise operations, matmul, bounded interpreter | Local x86_64 Linux. command-01 through command-10, golden files, numerical oracle and benchmark fixture. |
| Implemented but unverified | CI definitions for hosted Windows and macOS runners; selected host allocation failure branches | Workflow exists; local Linux execution and checked-size tests cannot demonstrate every platform or allocator failure. |
| Partially implemented | Bilingual diagnostics, confusable protection, release integrity and resource controls | Fallback details, partial script table, unsigned archive and process-level OOM remain. |
| Planned | HIR, symbolic/dynamic shapes and guards, typed Tensor IR, VM, modules, generics, AD, MLIR/LLVM, GPU, borrowing, FFI, standard modules, packages, mn-lsp and formatter | Designs and proposal programs. Requires stage-specific implementations and conformance evidence. |
| Missing | Published registry, appointed security contact, public namespace clearance, stable ABI, remote CI evidence in this package | Release gates. |
| Research required | Higher-order AD through effects, async ownership across heterogeneous devices, portable numerical parity, specialized attention, multi-node recovery, future hardware targets | Phase 7 and ROADMAP describe experiments and exit tests. |

The Python MLP oracle checks a mathematical proposal; no current Mongolia program can run that training example. A Rust release build optimizes the interpreter executable. The CLI accepts no native language compilation target.

The [v3.0 compatibility matrix](EXPANSION_V3_MATRIX.md) applies this status vocabulary to all 50 official expansion areas.
