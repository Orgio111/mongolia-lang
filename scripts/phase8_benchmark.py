"""Linux release baseline. Run with OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1."""
import ast
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import statistics
import subprocess
import tempfile
from time import perf_counter_ns

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
FIX = ROOT / "benchmarks/data"
WARMUP = 10
COUNT = 30


def run(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def samples(fn):
    ns = []
    for i in range(WARMUP + COUNT):
        start = perf_counter_ns()
        fn()
        elapsed = perf_counter_ns() - start
        if i >= WARMUP:
            ns.append(elapsed)
    return ns


def summary(values):
    q = statistics.quantiles(values, n=20, method="inclusive")
    return dict(median=statistics.median(values), p5=q[0], p95=q[-1],
                mean=statistics.mean(values), sd=statistics.stdev(values))


def main():
    if any(os.environ.get(key) != "1" for key in ("OPENBLAS_NUM_THREADS", "OMP_NUM_THREADS")):
        raise SystemExit("Set OPENBLAS_NUM_THREADS and OMP_NUM_THREADS to 1")
    os.chdir(ROOT)
    run("cargo", "build", "--release", "--workspace", "--locked")
    binary = Path(json.loads(run("cargo", "metadata", "--locked", "--no-deps",
                                 "--format-version", "1"))["target_directory"]) / "release/mn"
    probe = run("cargo", "test", "--locked", "--release", "-p", "mn-core", "--lib",
                "stage_measurements", "--", "--ignored", "--nocapture")
    nbytes, ntokens = map(int, re.search(r"INPUT (\d+) (\d+)", probe).groups())
    rows = [list(map(int, x)) for x in re.findall(r"TIMING (\d+) (\d+) (\d+) (\d+)", probe)]
    assert [x[0] for x in rows] == list(range(WARMUP + COUNT))
    raw = {key: [row[i] for row in rows[WARMUP:]] for i, key in enumerate(
        ["", "lex_ns", "parse_tokens_ns", "checked_32_matmuls_ns"]) if i}
    raw["startup_version_ns"] = samples(lambda: run(binary, "--version"))
    raw["cli_check_ns"] = samples(lambda: run(binary, "check", FIX / "frontend.mn"))
    text = (FIX / "matmul.mn").read_text()
    a, b = [ast.literal_eval(x) for x in re.findall(r"let [ab] = (.*);", text)]
    assert text.count("a @ b;") == 32

    def reference():
        columns = list(zip(*b))
        return [[sum(x*y for x, y in zip(row, col)) for col in columns] for row in a]

    x, y = np.array(a), np.array(b)
    expected = reference()
    assert np.array_equal(x @ y, expected)
    with tempfile.TemporaryDirectory() as temp:
        path = Path(temp) / "oracle.mn"
        path.write_text(text.replace("a @ b;", "print(a @ b);", 1))
        assert json.loads(run(binary, "run", path)) == expected
        raw["python_one_matmul_ns"] = samples(reference)
        raw["numpy_one_matmul_ns"] = samples(lambda: x @ y)
        rss = Path(temp) / "peak"
        run("cc", "-O2", "-Wall", "-Wextra", "-Werror", "benchmarks/peak_rss.c", "-o", rss)
        cmd = [rss, binary, "run", FIX / "matmul.mn"]
        raw["peak_rss_kib"] = [int(run(*cmd)) for _ in range(WARMUP + COUNT)][WARMUP:]
    cpu = Path("/proc/cpuinfo").read_text()
    def read(path):
        p = Path(path)
        return p.read_text().strip() if p.is_file() else None
    data = dict(warmup=WARMUP, samples=COUNT, frontend_bytes=nbytes, frontend_tokens=ntokens,
                executable_bytes=binary.stat().st_size,
                executable_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                fixtures={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in FIX.glob("*.mn")},
                cargo_lock=hashlib.sha256(Path("Cargo.lock").read_bytes()).hexdigest(),
                rust=run("rustc", "-vV"), cargo=run("cargo", "--version"),
                host=platform.platform(), python=platform.python_version(), numpy=np.__version__,
                blas=np.show_config(mode="dicts")["Build Dependencies"]["blas"],
                cpu=next((v.split(":",1)[1].strip() for v in cpu.splitlines() if v.startswith("model name")), "unreported"),
                affinity=sorted(os.sched_getaffinity(0)), meminfo=Path("/proc/meminfo").read_text().splitlines()[0],
                governor=read("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"),
                rustflags=os.environ.get("RUSTFLAGS", ""), raw=raw,
                summary={k:summary(v) for k,v in raw.items()})
    print(json.dumps(data, indent=2))


if __name__ == "__main__":
    main()
