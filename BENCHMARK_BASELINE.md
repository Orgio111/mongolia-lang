# Linux reference baseline

Mongolia v0.1.0, audited Phase 8 source. One instrumented Linux host measures small, repeatable workloads. Release CLI size was **806,296 bytes** (as built). Its SHA-256 was `210eb4ffe9646b296b69e22c009d37153db6aea67eb036da521833fc81e4e3f5`. Raw timings, fixture hashes, lockfile hash and environment are in [benchmark.json](verification/phase8/benchmark.json).

| Measured region | Unit | Median | P5 | P95 | Mean | Sample SD |
|---|---|---:|---:|---:|---:|---:|
| New process, mn --version | µs | 830.98 | 750.57 | 1,114.75 | 869.86 | 119.46 |
| New process, mn check on frontend fixture | µs | 1,496.53 | 1,335.75 | 1,694.83 | 1,504.92 | 118.47 |
| Lex original bilingual source | µs | 233.65 | 227.44 | 357.31 | 267.63 | 76.02 |
| Parse pre-tokenized input | µs | 105.10 | 103.92 | 129.65 | 118.60 | 60.44 |
| Checked runtime, 32 matrix products | µs | 180.66 | 179.56 | 333.85 | 211.67 | 74.02 |
| Python scalar reference, one product | µs | 185.34 | 182.05 | 262.78 | 202.43 | 30.33 |
| NumPy/OpenBLAS, one product | µs | 1.18 | 1.16 | 1.25 | 1.19 | 0.03 |
| CLI matmul fixture peak RSS | KiB | 1,472.00 | 1,444.00 | 1,536.00 | 1,476.67 | 31.64 |

Median lexer throughput was **61.89 MiB/s**. Median parser throughput was **26.85 million non-EOF tokens/s**. The interpreter fixture amortizes to **5.65 µs per 15×15 product** including its share of program setup, array reads and result allocation. Per-product reference rows include one operation per sample. These timing boundaries differ.

## Method

- Linux x86_64, kernel 6.18.57, glibc 2.41, AMD EPYC 9V74 80-Core Processor. Process pinned to logical CPU 0. Host-reported memory 10,206,472 KiB. Governor and physical host load unavailable. Rust 1.100.0, default release profile, no custom RUSTFLAGS, scalar strict f64, one interpreter thread.
- Ten warmups then 30 recorded samples per case. No recorded sample removed. P5/P95 use inclusive linear percentiles. SD is sample SD. All individual observations are in the JSON. Timers use monotonic clocks. CLI timings include process launch, captured output and exit with warm filesystem caches. Cold disk and compilation time were excluded.
- The frontend fixture has 15,162 UTF-8 bytes, 2,822 non-EOF tokens and 256 Mongolian bindings with integer expressions. Lex timing includes creation of tokens. Parser timing starts with an owned copy of the same token stream prepared before the timer. Parsing includes token consumption and AST construction. AST drop occurs after the timer.
- The runtime fixture constructs two 15×15 row-major f64 tensors with deterministic index-based values 0–10. It evaluates 32 products of the same input shapes. Timing begins after parsing and static validation. A fresh run includes tensor initialization, variable copies, allocation, multiplication and teardown. It emits no text. A separate printed product was verified for exact numeric equality against Python and NumPy before reporting.
- References used CPython 3.12.14 and NumPy 2.3.5 with OpenBLAS 0.3.30, OMP_NUM_THREADS=1 and OPENBLAS_NUM_THREADS=1. Arrays were preconstructed. Both reference timers include result construction. BLAS timing includes Python dispatch. These inputs have small exact-integer f64 results. Heavy work, all operand sizes, SIMD scaling and numerical tolerances for other data need separate experiments.
- Resident memory uses Linux wait4 ru_maxrss for a fresh release CLI running the whole matrix file. A small C parent measures that one child to avoid attributing the Python harness heap. RSS includes loader, frontend, runtime and allocator. File size counts the installed-format ELF file's bytes; no separate strip step was used.

## Reproduction

From the repository root on Linux, install the pinned Rust toolchain, a C compiler, Python 3.11+ and NumPy 2.3.5 with a recorded BLAS. Use an available logical CPU on your machine.

```sh
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 taskset -c 0 python3 scripts/phase8_benchmark.py > baseline.json
```

The regular test suite skips the timing probe. The harness selects it explicitly in release mode. Fixture, package and host hashes allow results to be traced. A performance regression threshold requires repeated measurements on dedicated hardware.
