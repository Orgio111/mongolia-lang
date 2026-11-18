"""Observe the Linux CLI under a per-child virtual address space limit."""
import argparse
import json
import platform
from pathlib import Path
import resource
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve()
    column = "[" + ",".join(["[1.0]"] * 240) + "]"
    row = "[[" + ",".join(["1.0"] * 240) + "]]"
    data = f"fn main() {{ let a={column}; let b={row}; let c=a@b; print(42); }}"
    observations = []
    with tempfile.TemporaryDirectory() as folder:
        source = Path(folder) / "allocation.mn"
        source.write_text(data, encoding="utf-8")
        for kib in (3500, 3700, 3900, 4100, 4600, 8192):
            def limit():
                resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
                resource.setrlimit(resource.RLIMIT_AS, (kib * 1024,) * 2)
            run = subprocess.run([binary, "run", source], cwd=folder, preexec_fn=limit,
                                 timeout=10, capture_output=True, encoding="utf-8")
            observations.append(dict(limit_kib=kib, exit=run.returncode, stdout=run.stdout,
                                     first_error=run.stderr.splitlines()[:1]))
    print(json.dumps(dict(platform=platform.platform(), shape=[240, 1, 240],
                          observations=observations), indent=2))


if __name__ == "__main__":
    main()
