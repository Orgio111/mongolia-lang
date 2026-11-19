# Phase 12 · typed bytecode VM

1. Lower verified HIR postorder operations into a private instruction stream. Encode function IDs, frame-local slots, typed constants and source origins. Freeze no public bytecode ABI.
2. Verify instruction bounds, stack effects, signature arguments, frame initialization, conversion domains, return consistency and checked tensor sizes before dispatch. Corrupt each invariant in negative tests.
3. Implement immutable frames, forward/recursive calls, guarded i64 arithmetic, numeric casts and buffered output. Preserve AST fuel/depth charges and increasing-k matmul.
4. Compare AST and VM per program on exit, output, first diagnostic and span in both languages. Include resource limits, NaN, signed zero, casts, malformed input and the full Phase 10 corpus. Human review is required before selecting the VM by default.
5. Measure lowering separately from execution and gate performance by workload. Dynamic shape obligations require runtime checks before allocation and separate RFC approval before source syntax.
