"""Fail when a sustained paired regression crosses the release review budget."""
import json
from pathlib import Path
import sys

data = json.loads(Path(sys.argv[1]).read_text())["summary"]
failures = []
for key in ("lexer_ns", "parser_ns", "matmul32_ns"):
    pairs = data[key]["paired_pct"]
    if len(pairs) != 6:
        raise SystemExit("Six paired batches required")
    sustained = sum(v > 20 for v in pairs) >= 5
    print(key, pairs, "REVIEW" if sustained else "within threshold")
    if sustained:
        failures.append(key)
if failures:
    raise SystemExit("Review sustained >20% regressions in " + ", ".join(failures))
