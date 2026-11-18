# Measured comparison · v0.1.0 integrated baseline and v0.1.1

The largest observed change was in the 32-product matrix fixture. Its paired median elapsed time fell 69.1%. The candidate visits contiguous output and right-hand tensor rows while keeping the original accumulation order for each element. Parsing moves token payloads into the AST. The lexer reuses already normalized spellings. The table measures the combined patch. It does not isolate the contribution of each edit.

| Measurement | Parent | Candidate | Median paired change | Range across six pairs |
|---|---:|---:|---:|---:|
| Lex the frontend fixture | 174.29 µs | 125.36 µs | −28.3% | −33.4% to +15.8% |
| Parse an existing token stream | 89.07 µs | 49.50 µs | −44.4% | −51.1% to −39.7% |
| Execute 32 checked 15×15 f64 products | 152.41 µs | 48.08 µs | −69.1% | −74.0% to −65.6% |
| Start mn and check the frontend fixture | 1.196 ms | 1.169 ms | −3.4% | −9.9% to +3.7% |
| Peak RSS of the check process | 1,603 KiB | 1,645 KiB | +2.8% | +2.0% to +5.6% |
| Standard release executable | 806,296 B | 851,512 B | +45,216 B (+5.6%) | File sizes |

One Linux x86_64 host, affinity CPU 0, Rust 1.100.0, ordinary Cargo release profile. Compare the same two on-disk fixtures and interleave six paired batches, reversing order on alternate batches. For each binary in each batch, discard ten warmups and keep 30 samples. Columns show medians of batch medians. Relative change is the median of six paired ratios. Ranges are descriptive. Latency includes process launch for the CLI row only. RSS uses wait4 in a separate run. Lexer/AST destruction and the restoration of parser input are outside those two timers. The tensor time includes runtime setup, variable copies and teardown. Compilation is outside all timers.

Exact fixture outputs and the matrix oracle passed. One lexer batch reversed direction. The CLI timing also changed direction across batches. Use the raw distributions when choosing an optimization or making a user-facing claim. At this stage the increased RSS and binary size are measurable costs. There is no independent faster-than-BLAS, GPU or whole-language result here.

Reproduce on Linux using Python 3, a C compiler, two source trees, separate Cargo targets and the same CPU affinity. From the candidate tree run `taskset -c 0 python3 scripts/phase9_compare.py /path/to/parent /tmp/mn-parent-target /tmp/mn-patch-target > paired.json`. Use a permitted equivalent CPU on hosts with a different cpuset. The parent archive digest is recorded in PHASE_9_REPORT.md. [Raw batches and toolchain](verification/phase9/performance.json) · [Build transcript](verification/phase9/performance-build.log) · [Earlier benchmark protocol](BENCHMARK_BASELINE.md).
