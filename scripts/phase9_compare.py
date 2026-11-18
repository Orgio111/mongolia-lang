"""Linux paired release comparison. Supply baseline source, baseline target, candidate target."""
import json
import os
from pathlib import Path
import platform
import re
import statistics
import subprocess
import sys
import tempfile
from time import perf_counter_ns

ROOT = Path(__file__).resolve().parents[1]


def capture(args, **kw):
    return subprocess.check_output(args, text=True, **kw)


def prepare(root, target):
    env = dict(os.environ, CARGO_TARGET_DIR=str(target))
    subprocess.run(["cargo", "build", "--release", "--locked", "--workspace"],
                   cwd=root, env=env, check=True)
    records = capture(["cargo", "test", "-p", "mn-core", "--locked", "--release", "--lib",
                       "--no-run", "--message-format=json"], cwd=root, env=env)
    probe, = [x["executable"] for line in records.splitlines() if
              (x := json.loads(line)).get("executable") and x.get("profile", {}).get("test")]
    return target / "release/mn", probe


def sample(binary, probe, rss):
    text = capture([probe, "stage_measurements", "--ignored", "--nocapture"])
    rows = [tuple(map(int, r)) for r in re.findall(r"TIMING (\d+) (\d+) (\d+) (\d+)", text)]
    assert [r[0] for r in rows] == list(range(40))
    data = {n:[r[i] for r in rows[10:]] for i,n in enumerate(
        ["", "lexer_ns", "parser_ns", "matmul32_ns"]) if i}
    cmd = [binary, "check", ROOT / "benchmarks/data/frontend.mn"]
    latencies, memory = [], []
    for i in range(40):
        start = perf_counter_ns()
        assert capture(cmd) == ""
        elapsed = perf_counter_ns() - start
        kib = int(capture([rss, *cmd]))
        if i >= 10:
            latencies.append(elapsed)
            memory.append(kib)
    data.update(check_ns=latencies, check_rss_kib=memory)
    return data


def main():
    baseline, bt, ct = [Path(s).resolve() for s in sys.argv[1:]]
    for name in ("frontend.mn", "matmul.mn"):
        assert (ROOT/"benchmarks/data"/name).read_bytes() == (baseline/"benchmarks/data"/name).read_bytes()
    builds = {"parent":prepare(baseline,bt), "patch":prepare(ROOT,ct)}
    batches = []
    with tempfile.TemporaryDirectory() as td:
        rss = Path(td) / "peak"
        subprocess.run(["cc","-O2",ROOT/"benchmarks/peak_rss.c","-o",rss],check=True)
        for order in [("parent","patch"),("patch","parent"),("parent","patch"),
                      ("patch","parent"),("parent","patch"),("patch","parent")]:
            batches.append({k:sample(*builds[k],rss) for k in order})
    summary = {}
    for key in batches[0]["parent"]:
        p, c = [[statistics.median(b[label][key]) for b in batches] for label in builds]
        ratios = [(y/x-1)*100 for x,y in zip(p,c)]
        summary[key] = dict(parent=statistics.median(p),patch=statistics.median(c),
                            paired_median_pct=statistics.median(ratios),paired_pct=ratios)
    print(json.dumps(dict(summary=summary,batches=batches,os=platform.platform(),
                          affinity=sorted(os.sched_getaffinity(0)),
                          rust=capture(["rustc","-vV"],cwd=ROOT),
                          sizes={k:v[0].stat().st_size for k,v in builds.items()}),indent=2))


if __name__ == "__main__":
    main()
