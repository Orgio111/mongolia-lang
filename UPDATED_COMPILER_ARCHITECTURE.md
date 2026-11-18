# Phase 11 compiler architecture

The two-crate workspace remains acyclic. mn-cli reads bytes, validates UTF-8 and renders errors. mn-core::frontend produces normalized tokens and a spanned AST. engine::checker collects signatures, resolves source names and emits expression operations as it infers their types. engine::hir owns backend-independent typed bodies. The verifier reconstructs result types and checks static shape facts before constructing CheckedProgram. The checked handle borrows the source AST for execution and string payloads, retains the HIR and exposes hir_dump(). Each run creates fresh interpreter state.

Resolution and type inference are logical stages fused in checker::lower. There is no separate allocation for an untyped HIR. Running an exhaustive resolution prepass could move a later undefined-name error ahead of an earlier type error. Tensor checks use the same checked arithmetic in the checker, verifier and final allocation. The verifier independently checks HIR operands and control/slot structure but shares scalar and tensor operation typing rules.

The VM can traverse the private module inside mn-core in the next phase. A new crate boundary should follow an actual second consumer. Direct AST interpretation provides the differential reference while the VM develops. MIR, SSA, MLIR and LLVM remain downstream designs.
