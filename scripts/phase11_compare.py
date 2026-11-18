"""Compare already-built release libraries and CLI binaries on one Linux CPU."""
import hashlib
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import sys
import tempfile
from time import perf_counter_ns

root = Path(__file__).resolve().parents[1]
targets = [Path(x).resolve() / "release" for x in sys.argv[1:]]
if len(targets) != 2:
    raise SystemExit("Supply Phase 10 and Phase 11 Cargo target directories")
cpu = min(os.sched_getaffinity(0))
os.sched_setaffinity(0,{cpu})
fixture = root / "benchmarks/data/frontend.mn"
batches = []
with tempfile.TemporaryDirectory() as work:
    w = Path(work)
    peak = w / "peak"
    subprocess.run(["cc","-O2",root/"benchmarks/peak_rss.c","-o",peak],check=True)
    drivers = []
    for i,t in enumerate(targets):
        driver = w / f"probe{i}"
        directories=sorted({p.parent for p in t.rglob("*.rlib")})
        search=[arg for directory in directories for arg in ("-L",f"dependency={directory}")]
        subprocess.run(["rustc","-O","--edition","2021",root/"benchmarks/phase11_semantics.rs",
                        "--extern",f"mn_core={t/'libmn_core.rlib'}",*search,
                        "-o",driver],cwd=root,check=True)
        drivers.append(driver)
    for batch in range(6):
        pair = {}
        for i in ((0,1) if batch%2==0 else (1,0)):
            lines=subprocess.check_output([drivers[i],fixture],text=True).splitlines()[10:]
            semantic,total=zip(*(map(int,s.split()[1:]) for s in lines))
            cmd=[targets[i]/"mn","check",fixture]
            times=[]; rss=[]
            for j in range(20):
                start=perf_counter_ns()
                subprocess.run(cmd,check=True,stdout=subprocess.DEVNULL)
                elapsed=perf_counter_ns()-start
                memory=int(subprocess.check_output([peak,*cmd]))
                if j>=5:
                    times.append(elapsed);rss.append(memory)
            pair[str(i)] = dict(semantic_ns=list(semantic),parse_and_validate_ns=list(total),
                                cli_ns=times,rss_kib=rss)
        batches.append(pair)
summary={}
for key in batches[0]["0"]:
    vals=[[statistics.median(p[str(i)][key]) for p in batches] for i in (0,1)]
    summary[key] = [statistics.median(group) for group in vals]
print(json.dumps(dict(cpu=cpu,platform=platform.platform(),fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),
                      summary=summary,batches=batches),indent=2))
