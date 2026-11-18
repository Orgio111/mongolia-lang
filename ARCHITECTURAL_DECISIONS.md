# Integration decision record

The decisions below govern the proposed expansion sequence. The foundation decisions remain in rfcs/ and the normative behavior remains in SPEC.md. Reopen a decision through a numbered RFC with a counterexample, measured cost or compatibility impact.

| ID | Conflict | Integration decision | Consequence and review test |
|---|---|---|---|
| I01 | Universal scope and a small auditable compiler | Put semantics, safety checks, IR verification and capability dispatch in core. Host/vendor integration and domain products have separate owners. | A new crate needs a functioning consumer and tests; evaluate the first consumer before splitting core. |
| I02 | Two surface languages and one meaning | Normalize keyword aliases once; NFC user names remain untranslated. Keep source origins outside canonical equivalence. | Compare normalized HIR and per-file spans for equivalent programs, and reject identifier collisions. |
| I03 | Existing AST execution and a canonical multi-IR pipeline | Retain the checked AST API as oracle. Introduce private resolved HIR for the already supported subset, then VM, then richer SSA. | Gate H accepts no grammar change; gate V requires observable parity including fuel. |
| I04 | Performance rewrites and strict semantics | Classify traps, output, resource charges and synchronization as observable. Strict math is default. Relaxed math uses explicit policy. | A dropped divide-by-zero or changed contraction result fails the relevant profile. |
| I05 | Static checking and dynamic AI workloads | Reject known-invalid shapes statically. Carry unresolved extents with runtime guards dominating allocation/use. | Static mismatch produces no output; failed dynamic guard cannot enqueue work. |
| I06 | Host value semantics and zero-copy device memory | Keep v0.1 copies/charges observable. New borrowing/ownership operations explicitly express aliases and lifetime. Async queues retain leases. | Test aliasing, early drop, lost device and failed submission before pooling buffers. |
| I07 | Explicit device choice and adaptive placement | Restrict optimizers to devices/capabilities permitted by the program/profile. Report transfers and require authorized fallback. | Disable a capability and check rejection or the documented fallback path. |
| I08 | Early VM and MLIR overlap | One type/effect/shape meaning; one adapter per materially distinct target. A second path needs a reproducible gap in latency, coverage or deployment. | Compare each adapter independently with the oracle on advertised operations. |
| I09 | AD and mutable training code | Transform pure typed regions. Keep optimizer updates outside differentiation. Externals need explicit derivative and effect rules. | Reject unsupported effects and test gradient sums/adjoint identity before training. |
| I10 | Stable C FFI and evolving implementation | Keep Rust-native layout, HIR IDs and machine register layouts private. Version explicit C ABI records, callbacks, errors and release functions. | An independently built consumer must round-trip ownership and failure. |
| I11 | Reproducibility and extensible packages | Separate source edition, language dependency lock, build input hashes and compiler bootstrap. Publishing has explicit trust and permissions. | Offline replay and corrupted-input rejection precede a registry release. |
| I12 | Measured progress and ambitious product names | Record names as proposals; profile support needs a feature contract, tested target, owner and evidence. | The eight internal names and nineteen ecosystem projects get no separate execution claim from the source register. |
| I13 | Host resource limits and isolation | Fuel is a language work budget. Enforce hard process time/memory and external authority through a separately tested host boundary. | Test semantic budget exhaustion independently from OS-level enforcement. |
| I14 | Near-term stable language and eventual self-hosting | Stabilize supported source/ABI contracts first, port bounded tools later, and retain a documented Rust stage-0 seed. | Independent stage bootstrap needs separate provenance and conformance evidence. |

[Dependency gates](FEATURE_DEPENDENCY_GRAPH.md) · [Implementation priorities](IMPLEMENTATION_PRIORITY_MATRIX.md) · [Risk owners](RISK_REGISTER.md)
