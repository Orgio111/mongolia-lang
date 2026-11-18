# Benchmark protocol

Measure lexer bytes/s, parser tokens/s, full check latency, process startup, scalar operations,
rank-2 matmul and peak resident memory for v0.1. VM, broadcasting, AD, GPU and training benchmarks
are blocked on those implementations. Report failure/resource-limit cases rather than dropping them.

Record CPU model, cores, RAM, OS/kernel, toolchain, git revision/archive SHA256, Cargo.lock,
release flags, governor/power mode, thread count, input seed and dimensions. Separate parse/check
from runtime; report end-to-end CLI as another measurement. For each case run 10 warmups and at
least 30 samples, retain raw measurements, report median, p5/p95, mean and standard deviation.
Include cold-process latency separately. Never infer timing from unit-test wall time.

For matmul compare both an independent scalar reference and optimized BLAS with explicit library,
version and thread settings. Compare equivalent f64 operations and tolerances before timing.
Record whether parsing, validation, copies, allocations and output are included. Resource-limited
interpreter results must not be extrapolated to larger matrices. The [Phase 8 baseline](../BENCHMARK_BASELINE.md) includes fixture definitions, raw samples and a rerun command.

Future GPU timing must synchronize before/after regions and report host/device transfer, kernel
compilation, launch and execution separately. Record driver/toolkit, GPU model, clocks, precision,
fast-math, tensor-core mode, peak allocated/reserved VRAM, warmup and repeat variance.
