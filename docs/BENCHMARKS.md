# Benchmark protocol

No performance claims are established by this repository. Correctness tests are not benchmarks.

Record CPU/GPU model, memory, OS/kernel/driver, power mode, compiler and dependency versions,
commit, release flags, target features, thread count, dtype, shapes, layout, seed, math mode and
numerical tolerance. Save raw samples and scripts. Separate parse/check/compile, process startup,
allocation, transfer, synchronization and steady execution. Use warmup and enough repeated runs
to report median, dispersion and confidence intervals; retain outliers and explain exclusions.
GPU timers must synchronize the measured stream and separate host enqueue from completed work.

Start with lexer bytes/s, parser latency, CLI startup, validation latency, scalar calls, matmul and
peak RSS. Use deterministic shape sweeps including tiny, non-square and awkward dimensions.
Compare matmul to both a simple independent reference and optimized BLAS with equal precision,
threading and allocation treatment. Later add VM throughput, broadcasting, memory bandwidth,
kernel launch latency, training step throughput, inference latency, peak RAM/VRAM and transfer
costs. Report compilation amortization separately. Test output validity before retaining timings.
Automated regression alerts need noise characterization and a dedicated machine; CI wall-clock
noise must not become a language-speed claim.
