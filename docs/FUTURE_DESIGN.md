# Future language and runtime design

Everything in this document is **proposed**, not executable v0.1 syntax. No version is a
promise of availability. RFC acceptance, implementation and conformance tests precede support.

[Phase 7 architecture](PHASE_7_FUTURE_ARCHITECTURE.md) adds the implementation contracts, concrete failure cases and a reproducible numerical reference.

## Types and formal judgments

Future scalars: i8/i16/i32/i64, u8/u16/u32/u64, f16/bf16/f32/f64, bool, string, unit and never.
Compound types: arrays, slices, tuples, structs, tagged enums, Option, Result, references,
generic containers and tensors. Generic constraints are explicit; no implicit numeric widening.
Integer operations trap consistently across debug/release; explicit wrapping APIs may follow.
Float formats specify rounding at operations and conversions; NaN payload preservation is
not guaranteed, and total ordering is an explicit library operation. Float equality follows
IEEE comparisons. Checked casts return Result; lossy casts require an explicit operation.

Judgment `Γ ⊢ e : τ ! ε` means expression e has type τ and effect set ε.
Binding: if Γ⊢e:τ then Γ,x:τ⊢body is checked with x immutable unless declared mutable.
Application: if f:(τ1,...,τn)->τ and Γ⊢ei:τi, then Γ⊢f(e1,...,en):τ.
Return must match the enclosing result exactly. Arithmetic requires the same numeric type
on both sides. Branches must have a common exact type; never coerces to any result type.
Generic instantiation substitutes types/constants and discharges declared constraints.
Tensor construction checks element homogeneity, rectangularity and checked shape product.
Matmul: T[M,K] @ T[K,N] : T[M,N], with a guard when equality cannot be proved.
Device compatibility requires both operands on the same explicitly selected device.
Grad requires a pure differentiable function with scalar real output; result shape equals
the selected input shape. Effects are part of the type/checking result, not runtime guesses.

## Shape solver

Dimension terms are static natural numbers, symbols with scope IDs, or dynamic SSA values.
Use union-find for equalities and a separate constraint ledger for products/broadcast choices.
Never equate unrelated dynamic values merely because both use `?` syntax. Static conflicts
report both origins. Unknown constraints lower to runtime guards with source labels.

| Operation | Inference / guard |
|---|---|
| Matmul | [M,K]×[K,N] -> [M,N]; equate contraction dimensions |
| Batched matmul | Broadcast leading axes, then matmul trailing axes |
| Broadcast | Right-align; each pair equal or one; 0 with 1 gives 0; otherwise error |
| Reshape | Equal checked element products; at most one inferred axis; zero-product ambiguity errors |
| Transpose | Require a permutation; permute shapes/strides, preserve storage identity |
| Concatenate | Equal rank and non-axis dimensions; checked sum on concatenation axis |
| Reduction | Remove selected unique axes, or set to one with keepdims |
| Index/slice | Bounds guards; slice lengths from explicit start/stop/step; step cannot be zero |
| Convolution | Validate channels/groups and nonnegative output formula per axis; checked padding arithmetic |
| Attention | Validate batch/head broadcast, Q/K feature equality and K/V sequence equality |
| Einsum | Resolve labels, equate repeated-label extents, reduce labels absent from output |

Rank and shape are semantic; strides/layout are storage metadata. A view borrows storage;
cloning creates storage. Broadcast views have zero strides and are read-only unless uniquely
materialized. Check every stride/offset/byte-product arithmetic before allocation or access.
Empty dimensions are legal once specified per operation; allocation capacity is distinct
from logical shape. Dynamic guards run before allocation and before any dependent kernel.

Diagnostic example: `E_SHAPE: [2,3] @ [2,2]: inner dimensions 3 and 2 differ.`
Mongolian catalog draft: `E_SHAPE: [2,3] @ [2,2]: дотоод хэмжээ 3 ба 2 таарахгүй.`
Native-speaker review is required before these terms stabilize.

## Automatic differentiation

Differentiate typed tensor IR before bufferization. Reverse mode records primal intermediates
needed by registered VJP rules and accumulates cotangents by addition; forward mode propagates
JVP tangents. `grad(f)(x)` is restricted to a scalar real result. Non-scalar output requires an
explicit cotangent via VJP, or JVP seed. Multiple arguments use an explicit selection list and
return a tuple; inactive arguments are constants. Stop-gradient retains the primal and emits
zero tangent. Reject integer, Boolean, IO, mutation-through-alias, unsafe and external operations
unless a rule establishes valid semantics. Custom rules declare shape/type/device contracts and
are verified with the same checker as ordinary functions.

Control-flow AD follows the executed path, preserving predicates and loop iteration counts.
Checkpointing stores selected states and replays only deterministic pure regions. Randomness
must use explicit reproducible state. Higher-order AD requires differentiable derivative rules;
a first-order success does not imply higher-order support. Propose ReLU derivative zero at zero,
but require RFC approval; document every nonsmooth convention. Validate central finite differences
across multiple step sizes plus independent analytical/JVP-VJP dot-product checks. Checkpoints,
tapes, accumulation precision and recomputation costs must be measured, not called zero-overhead.

## Ownership and asynchronous device execution

A unique owner controls each allocation. Shared references permit read access; mutable references
require exclusivity. Views retain a borrowing relationship to backing storage. Explicit clone
allocates/copies. Reference-counted handles may hold shared external resources but do not imply
mutable alias safety. Host and device allocations are distinct typed resources. Device transfer
is explicit and returns a value or a transfer future; it never changes an existing value's device.

Queued commands retain allocation leases through completion events, including error paths.
Dropping the user handle releases only its lease. Pools recycle memory after all events complete.
Pinned memory also remains live through transfers. Cross-stream hazards create event dependencies;
host reads await completion. OOM and device-loss are recoverable errors where the driver allows;
poisoned devices reject new work. Allocation pools have budgets, trim hooks and telemetry.
Cancellation may abandon results but cannot invalidate in-flight buffers. Borrow checking cannot
eliminate OOM or malicious resource exhaustion.

## Hardware lowering

CPU: structured tensor IR -> linalg -> loop/vector/memref -> LLVM; target features are queried
and recorded. BLAS dispatch checks layout, dimensions, integer ABI sizes and threading policy.
CUDA: linalg tiling/fusion -> GPU mapping -> NVVM/PTX; a driver module loads kernels and binds
explicit streams. cuBLAS/cuBLASLt/cuDNN adapters require capability and numerical-policy checks.
Tensor-core selection requires supported precision, alignment, dimensions and accumulator type.
Vulkan: lower an allowed IR subset to SPIR-V compute with descriptor layouts and synchronization.
WebGPU: generate WGSL or a verified supported translation path; enforce bounds and API limits.
No assumption that CUDA features map directly to portable backends. AMD, Metal, Intel, RISC-V
vectors and Wasm require separate feasibility RFCs and conformance budgets.

Each backend publishes supported dtypes, rank/size limits, math modes, atomics and synchronization.
Unsupported operations error or use a user-enabled fallback; fallback copies and synchronization
must be visible in diagnostics/profiling. Kernel cache keys include compiler, IR, target features,
math mode, driver compatibility and specialization constants. Validate cache inputs and artifacts.

## C FFI

`extern "C"` is a future grammar feature. Only fixed-width scalars, explicit repr(C) structs,
opaque handles and pointer-length pairs are ABI-compatible; strings and tensors never pass
as Rust-native layouts. State alignment, padding, calling convention and symbol naming per target.
Pointers declare ownership, mutability, length and lifetime; borrowing cannot outlive a call unless
an owned registration handle is returned. Callbacks retain environment lifetimes until explicit
unregistration and completion. External unwinding must stop at the wrapper boundary; convert
failures to status/result values. Static/dynamic linking is explicit and target-specific. Rust
interop uses C wrappers, never the native Rust ABI. Derivatives need explicit rules.

## Packages and reproducibility

Future mn.toml package/version/edition fields use semantic versioning; editions name accepted
language compatibility sets. Deterministic dependency resolution records exact versions, registry
identity, checksums and feature selections in mn.lock. Offline builds require all verified cache
entries; missing content errors. Yanked versions remain usable from an existing verified lockfile
but are excluded from fresh resolution. No arbitrary installation scripts by default. Native build
steps need a declared permission model, pinned inputs and auditable outputs. Publishing requires
namespace ownership, immutable versions, provenance policy, revocation and incident response.
Signatures supplement checksums rather than replacing origin trust. Registry governance, naming
availability and security advisory processes must precede public distribution.

## Language tooling

An incremental database keys source revisions, tokens, syntax, symbols, types and shape facts.
Cancellation and stale-result suppression are mandatory. mn-lsp negotiates position encoding;
UTF-16 is supported by counting code units from valid byte boundaries, not bytes or graphemes.
Expose diagnostics, hover, definitions, references and rename before completion and refactors.
Renaming rechecks NFC/confusables and all affected scopes. Formatting operates on lossless syntax
and offers en/mn/preserve keywords without changing identifiers or semantics. Semantic tokens
classify canonical roles equally in both spellings. VS Code extension launches mn-lsp, registers
.mn, declares both keyword sets and exposes target/language configuration. Tree-sitter is a separate
editor parser tested against shared syntax fixtures, not the language's semantic authority.

## Standard library and AI infrastructure

| Module | Initial proposed API | Ownership/error/device/numerics/complexity contract |
|---|---|---|
| core | Option, Result, assert | Value semantics; explicit errors; all devices where lowered; O(1) wrappers |
| math | sqrt, exp, log | Pure scalar/vector ops; domain behavior specified per IEEE; approximation policy explicit |
| tensor | zeros, reshape, transpose, matmul | Owned allocation or declared view; checked shape/OOM errors; explicit device; documented FLOPs/bytes |
| nn | dense, relu, norm, embedding, convolution, attention | Parameter borrowing; shape errors; backend capabilities; dtype and accumulation specified |
| optim | SGD, Adam | Exclusive parameter/state updates; shape/device checks; O(parameter count) work/storage |
| io/fs/net | streams, files, sockets | Owned handles; Result errors; host-only effects; blocking/async explicitly distinguished |
| time | monotonic, duration | Host clock effect; no assumptions about wall-clock monotonicity |
| random | seeded generator | Explicit state; algorithm/version pinned for reproducibility; device sequence guarantees declared |
| device | transfer, synchronize, memory_stats | Event-bound ownership; OOM/device-loss errors; explicit synchronization |
| ffi | raw pointers and handles | Unsafe boundaries; caller/owner contracts; no automatic derivative |
| collections | vector, map | Owned elements; checked capacity; amortized and worst-case costs documented |
| serialization | load/save | Versioned formats, bounded parsing; host IO errors; tensor bytes/endian/layout explicit |

Rust implements initial primitives. Migrating modules to Mongolia does not self-host the compiler.
Model serialization, safetensors/DLPack/ONNX/tokenizers require external-format adapters, validated
ownership and version compatibility. Mixed precision and quantization require compiler/runtime
numerical policies. Distributed parallelism needs collectives, failure semantics and deterministic
ordering options. Flash-attention-style kernels and advanced fusion need specialized optimization
and evidence. KV caches, batching and serving belong above the compiler, with explicit memory
budgets. None is implemented by the foundational matmul interpreter.
