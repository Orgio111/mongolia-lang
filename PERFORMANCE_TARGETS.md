# Performance evidence and proposed qualification targets

The numbers below come from Phase 8 on one Linux x86-64 host. The original raw samples, executable/fixture hashes, CPU affinity, compiler and BLAS details are preserved. Integration rechecks correctness. It makes no new timing claim. [Raw data](verification/phase8/benchmark.json) · [Method](BENCHMARK_BASELINE.md)

| Phase 8 measurement | Observed median | Timed boundary |
|---|---:|---|
| Startup | 0.831 ms | Fresh mn --version process with warm filesystem cache |
| CLI check | 1.497 ms | New process checking the fixed bilingual frontend fixture |
| Lexer | 233.65 µs, 61.89 MiB/s | 15,162 bytes including token construction |
| Parser | 105.10 µs, 26.85 M tokens/s | Owned pre-tokenized input, 2,822 non-EOF tokens |
| Matrix execution | 180.66 µs for 32 products | Checked interpreter, 15×15 f64, includes reads/allocation/teardown |
| Peak RSS and size | 1,472 KiB median RSS; 806,296-byte executable | Per-child whole matrix CLI run; release ELF as built |

## Proposed targets and decision procedure

| Scope | Proposed target | Workload and evidence needed |
|---|---|---|
| Phase 9 semantic parity | Zero mismatches in acceptance output, diagnostics, budget boundaries and checked numeric results. | Stable baseline plus new resolution fixtures. Correctness is a hard gate. |
| Phase 9 frontend cost | Investigate repeatable >10% median increase in end-to-end check time or >15% median RSS increase on the same machine. | Paired parent/candidate builds, same fixture hashes, release flags and affinity. Measure frontend-fixture RSS separately since the historical RSS fixture exercises matmul. Record absolute change and uncertainty. These initial thresholds trigger review. |
| v0.2 VM | 100% functional parity on supported interpreter corpus. Demonstrate the size, call throughput and steady execution cost of both engines on the same checked input. | Scalar call chains, depth limits, tiny and rectangular tensors. Report verification/compile and execution separately. Set a numeric speed goal only after the first matched VM measurements. |
| v0.3 AD | For smooth small f64 fixtures, initial tolerance proposal: absolute error ≤1e-7 or relative error ≤1e-5 versus analytic reference. Check directional derivatives at three step sizes. | Use combined absolute/relative comparison around zero, explicitly mask defined nonsmooth points, compare JVP/VJP dot products and peak tape bytes. Investigate failures before changing tolerances. |
| v0.4 CPU | Zero mismatches on exact dyadic corpus in strict mode; publish latency/throughput relative to scalar and optimized BLAS per shape. | Same dtype/layout/threads/output validation; separate allocation, copying, compile and kernel. For inexact workloads publish conditioning and per-operation tolerance. |
| v0.5–v0.6 GPU | Every advertised dtype/op passes its predeclared accuracy contract; all unsupported fixture cases report capability errors. | Measure synchronized host-to-device, kernel and device-to-host intervals, launch cost, peak allocated/reserved VRAM and end-to-end time. Compare strict and opt-in relaxed modes separately. |
| v0.7–v1.0 tools and AI | Zero stale diagnostics across ordered edit fixtures and zero corrupted-cache acceptances. Report p50/p95 edit/check times and model reference errors by version. | Large bilingual files, offline cache, small pinned model and device/host matrix. Set workload-specific latency budgets before qualification. |
| v1.x–v3.x services and optimization | Measure p50/p95/p99 latency at stated load, failure recovery, memory, cost/energy where available and scaling efficiency. Tuning promotes only with held-out benefit after amortizing search. | Match competitor semantics, data, correctness, thread count, precision and transfer boundaries. State sample sizes and workload coverage. Claims apply only to tested profiles. |

For regression attribution use at least three independent paired batches per revision, interleave run order, 10 warmups and at least 30 retained samples per timing case. Report median, dispersion and confidence interval for the paired difference. Use substantially larger samples for rare tails. A noisy shared host can establish an exploratory baseline. A new public comparative claim needs reruns on controlled hardware. Reproduce from the repository root using the harness in BENCHMARK_BASELINE.md. Reference matrices timed as one native library call cannot be divided directly into the interpreter's full program timing to claim a language-wide speed ratio.
