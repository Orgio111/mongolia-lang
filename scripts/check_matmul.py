#!/usr/bin/env python3
"""Compare debug and release CLI results to exact small dyadic matrix products."""
import argparse
from fractions import Fraction
import json
from pathlib import Path
import random
import subprocess
import tempfile


def matrix(rng, rows, cols):
    return [[Fraction(rng.randint(-32, 32), 8) for _ in range(cols)] for _ in range(rows)]


def literal(values):
    return json.dumps([[float(cell) for cell in row] for row in values])


def product(left, right):
    return [[sum(x * y for x, y in zip(row, col)) for col in zip(*right)] for row in left]


def compare(binaries, seed):
    rng = random.Random(seed)
    completed = 0
    with tempfile.TemporaryDirectory() as directory:
        path = Path(directory) / "matrix.mn"
        for case in range(96):
            m, k, n = (rng.randint(1, 6) for _ in range(3))
            valid = case % 3 != 2
            j = k if valid else k + 1
            left, right = matrix(rng, m, k), matrix(rng, j, n)
            expected = product(left, right) if valid else None
            if case % 2:
                source = f"функц main(){{хувьсагч а={literal(left)};хэвлэх(а @ {literal(right)});}}"
            else:
                source = f"fn main(){{let a={literal(left)};print(a @ {literal(right)});}}"
            path.write_text(source, encoding="utf-8")
            observations = []
            for binary in binaries:
                run = subprocess.run([str(binary), "run", str(path)], capture_output=True,
                                     encoding="utf-8", timeout=10)
                observations.append((run.returncode, run.stdout, run.stderr))
                if valid:
                    if run.returncode or run.stderr or json.loads(run.stdout, parse_float=Fraction) != expected:
                        raise RuntimeError(f"matrix result mismatch, {binary}, case {case}: {source}\n{run}")
                elif (run.returncode != 1 or run.stdout or "E_SHAPE" not in run.stderr
                      or f"[{m}, {k}] @ [{j}, {n}]" not in run.stderr):
                    raise RuntimeError(f"shape diagnostic mismatch, {binary}, case {case}: {source}\n{run}")
                completed += 1
            if observations[0] != observations[1]:
                raise RuntimeError(f"profile difference in case {case}: {source}\n{observations}")
    print(f"seed {seed}: 64 compatible and 32 incompatible pairs, {completed} CLI runs passed")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binaries", nargs=2, type=Path, metavar="BINARY")
    parser.add_argument("--seed", type=int, default=20261118)
    args = parser.parse_args()
    compare([path.resolve() for path in args.binaries], args.seed)
