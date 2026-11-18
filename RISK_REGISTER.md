# Unified risk register

Scores are planning judgments. L and I represent likelihood and impact on a 1–3 scale. Owners are roles requiring a named assignee before promotion.

| Risk | L/I | First observable trigger | Control and evidence | Owner / gate |
|---|---|---|---|---|
| Resolution changes diagnostic precedence or binds a different symbol | 2/3 | First differing code/span or unresolved slot in accepted HIR | Run through original validator first during introduction; verify ID bounds, caller scope and NFC cases. Keep AST oracle available. | Frontend / H |
| Unicode ambiguity or inadequate localization | 3/2 | Spoof false negative, wide-column or catalog regression | Retain original spans; version spoof policy, add reviewed bilingual fixtures, negotiate editor encoding. | Language and security / E, S |
| VM or optimizer alters traps, signed zero, output or fuel | 2/3 | Differential mismatch at boundary or guard fallback | Attach origins and semantic charges, verify effects; disable offending path until minimized fixture passes. | Compiler / V, O |
| Unsound symbolic shape or dtype inference | 2/3 | Allocation or dispatch precedes a necessary guard | Scoped dimension terms and checked products, verifier for guard dominance, negative shape corpus. | Type/numerics / T |
| AD changes primal effects or computes incorrect gradients | 2/3 | Analytic, adjoint or finite-difference failure | Restrict pure region, reject missing derivative rules, compare with independent numerical references. | AD owner / D |
| Device buffers recycled before completion | 2/3 | Pending operation accesses retired resource | Per-queue completion leases, fault injection, high-water and leak measurements on named drivers. | Runtime / M, N, W |
| FFI/SDK ABI drift or unsafe ownership | 2/3 | Alignment mismatch, lost callback, uncaught foreign exception | Narrow versioned ABI, external consumer tests, audited unsafe adapter and unwind boundary. | Interop / F, X |
| Resource exhaustion exceeds language budgets | 3/3 | Process RSS/time grows beyond advertised profile | Separate semantic fuel from host sandbox; cap allocations where possible; document remaining host OOM and test quotas under isolation. | Security/runtime / A, Q |
| Native/portable support asserted without device evidence | 3/3 | Advertised profile lacks runner, device ID or negative capability test | Maintain target capability manifest and publish tested configurations; defer promotion for untested target. | Backend/release / C, N, W, X |
| Supply-chain, naming or signing ownership unresolved | 2/3 | Missing provenance, key owner or private disclosure route | Pin dependencies, review advisories, reproduce cache tamper rejection and validate signature/revocation policy. Perform namespace review before public adoption. | Release/security / K, S |
| Partial writes or retries duplicate external effects | 2/3 | Interrupted init leaves a partial scaffold or remote operation replays | Document existing two-file init limitation; test idempotency, bounded retries and recovery at durable commit boundaries. | CLI/distributed / Q, U |
| Benchmarks reward numerical drift or unmatched scope | 3/2 | Inconsistent dtype, stale warmup or unmatched transfer costs | Freeze fixture/hash and accuracy predicate; keep all samples; rerun equivalent workloads before publishing. | Performance / A, O, J |
| Tooling uses stale revisions or invalid text offsets | 2/2 | Diagnostics for an old document or invalid UTF-16 span | Revision-aware caches, cancellation and round-trip Unicode property tests. | DX / E |
| Product sprawl, incompatible paradigms or empty crates | 3/3 | New core API has no real consumer, semantics or ownership | Use primary delivery class, RFC and independent consumer criteria; track rejected/deferred work. | Architecture / B, S |
| Bootstrap or research results lack independent verification | 2/3 | Stage outputs or claimed hardware path cannot be reproduced | Pin stage seeds/toolchains, independently rebuild/replay; label simulations and distinguish device results. | Research / J, Z |

A triggered risk creates a reproducible fixture or evidence gap and a gate decision. Review open risks at each release boundary. The Phase 8 [security audit](SECURITY_AUDIT.md) and [known limitations](KNOWN_LIMITATIONS.md) retain the verified scope and unresolved external checks.
