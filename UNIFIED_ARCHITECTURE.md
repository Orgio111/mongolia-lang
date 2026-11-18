# Unified compiler, runtime and ecosystem architecture

## Running foundation and migration boundary

The live dependency direction is mn-cli → mn-core → unicode-ident, unicode-normalization and unicode-script, with tinyvec transitive. The CLI owns files, UTF-8 decoding, rendering and process exit. Core owns tokens, byte spans, the owned AST, structural types, validation and execution. A checked handle retains an immutable borrow and function signatures. Each run allocates fresh state. The isolated fuzz workspace owns its separate lockfile and harnesses. The cargo-built release binary executes a validated AST on the host.

| Producer → consumer | Contract and ownership | Verification and invalidation |
|---|---|---|
| Source → lexer/parser | Retain original bytes and half-open spans. Canonical aliases and NFC names feed a single grammar. Future lossless trivia lives beside semantic tokens. | Validate source limits and UTF-8 boundaries. Any source edit invalidates positions in that revision. |
| AST → resolved HIR | Snapshot-local function IDs and frame slots, source-origin map, explicit binding and typed call signatures. Keep parser AST independent of backend storage. | Check ID range, slot initialization and distinct namespaces. A declaration/interface edit invalidates dependent resolution. |
| HIR → typed tensor IR | Typed values, dimension symbols scoped to functions/modules, explicit guards and effects. Preserve an origin for each operation that can fail. | Check types, definitions, dominance, constraints and guard dominance. Shape/call edits invalidate dependent proofs. |
| Typed IR → AD → SSA/MIR | Functional tensor SSA carries shape and purity before bufferization. AD emits typed primal/adjoint operations. MIR makes control flow, storage lifetimes and dispatch explicit. | Verify before/after transformations. CFG changes invalidate dominance/liveness; memory/effect changes invalidate alias and scheduling proofs. |
| HIR/MIR → VM | First VM can lower the existing checked static subset directly to typed registers. Calls define fresh frame ownership. Adopt the shared IR path as it becomes available. | Verify initialized reads, register types, callee signatures, returns and fuel accounting before dispatch. |
| Typed operations/MIR → native backends | Common shape, effect, numeric and ownership facts precede target legalization. Each adapter describes capabilities, calling convention, kernels and cache inputs. | Reject unsupported capabilities before dependent allocation or launch. Key caches by compiler, target, ABI, numerical policy, specialization and compatible driver. |

The foundational AST remains the differential oracle during migration. The canonical pipeline is shared semantically; a simple interpreter or static VM may bypass an IR stage when its verified subset has equivalent checks. Optimizations must retain work charges and first-trap origins, including operations whose result is unused.

## Runtime, memory and device boundaries

The VM owns typed frames and fuel. The storage layer owns buffer handles, validated shape/stride metadata and a storage identity. A future view retains its backing allocation; mutable access requires exclusive ownership of the accessed region. Validate offset, size and stride arithmetic before access. Negative strides and zero-sized shapes need explicit per-operation rules. The foundation's positive static shapes remain valid without conversion.

For GPU submission, validate operation legality and reserve memory before enqueue. Submission retains all read/write buffers and pinned transfer memory. A completion event transfers them from in-flight to reclaimable after all dependent queues finish. Cancellation suppresses delivery to the caller while preserving outstanding driver ownership. Device loss marks the device unusable, fails pending work, then retires resources according to the driver contract. Mock queues test failure and completion order before a device adapter ships. Target memory quotas complement host-process isolation; ordinary allocation can still fail.

Explicit host/device transfers are effects. Device placement is part of the checked execution contract; cost-based placement can select among allowed, equivalent destinations. References crossing the C ABI carry layout, length, alignment, ownership and release contracts. Contain unwinding inside each host runtime. Differential state, debug origins and ABI handles use separate identity spaces.

## Numeric transformation and backend paths

- For v0.1 matmul, accumulate each output over increasing contraction indices. Future strict backends preserve specified rounding, exception behavior and signed zero. Fusion or BLAS substitution requires a proof of equivalence or a separately selected numerical mode.
- Differentiate requested pure typed regions before bufferization. Scalar grad requires a real scalar output. VJP/JVP require explicit seeds where appropriate. Preserve execution-path choices. Stop-gradient is an explicit boundary. Custom derivatives declare types, effects, devices and shape behavior. Validate independently with finite differences and adjoint identities.
- Native CPU lowering uses structured tensor/loop/vector/memory operations, then LLVM and the target ABI. CUDA uses distinct GPU/NVVM/PTX lowering and a driver adapter. Vulkan uses validated SPIR-V; WebGPU uses its supported WGSL/capability subset. A shared verifier precedes target-specific legalization. Device support requires actual hardware evidence.
- JIT, OSR, Cranelift, C emission, Wasm, FPGA, vendor accelerators and learned optimization are separable experiments. Their first gate is a measured use case the existing path cannot satisfy. The trusted verifier remains independent of learned candidate selection.

## Toolchain and ecosystem

Module resolution owns normalized module identities. Package resolution owns content identities, trusted sources and a lockfile for Mongolia programs. Cargo.lock continues to identify the Rust bootstrap. Editor tooling consumes revisioned semantic facts, negotiates positions and suppresses stale results; formatting retains trivia and honors keyword style. Diagnostic localization changes presentation without changing semantic identities.

Standard modules define portable value/error semantics. Platform adapters own vendor APIs and foreign lifetimes. Services, registries, Studio and domain frameworks have independent packages and owners. A capability grant scopes host effects, including device, network and foreign calls. Stability claims attach to a supported profile and dependency versions. The [50-area matrix](IMPLEMENTATION_PRIORITY_MATRIX.md) locates each proposed workstream and its first gate.
