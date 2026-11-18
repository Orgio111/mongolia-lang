#!/usr/bin/env python3
"""Build and test the workspace, then check each example through the CLI."""
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    for command in (
        ["cargo", "build", "--workspace", "--locked"],
        ["cargo", "test", "--workspace", "--locked"],
        ["cargo", "fmt", "--all", "--", "--check"],
        ["cargo", "clippy", "--locked", "--workspace", "--all-targets", "--", "-D", "warnings"],
    ):
        print("$", *command, flush=True)
        subprocess.run(command, cwd=ROOT, check=True)

    cases = json.loads((ROOT / "verification/examples.json").read_text(encoding="utf-8"))
    sources = sorted(path.relative_to(ROOT).as_posix() for path in (ROOT / "examples").rglob("*.mn"))
    if sorted(case["file"] for case in cases) != sources:
        raise SystemExit("Update verification/examples.json to cover every .mn example exactly once")

    for case in cases:
        source = case["file"]
        language = "mn" if source.endswith("_mn.mn") else "en"
        command = ["cargo", "run", "--locked", "--quiet", "-p", "mn-cli", "--",
                   "run", source, "--lang", language]
        result = subprocess.run(command, cwd=ROOT, capture_output=True, encoding="utf-8")
        actual = (result.returncode, result.stdout, result.stderr)
        expected = (case["exit_code"], case["stdout"], case["stderr"])
        if actual != expected:
            raise SystemExit(f"{source}\nexpected {expected!r}\nobserved {actual!r}")
        print(f"PASS {source}")


if __name__ == "__main__":
    main()
