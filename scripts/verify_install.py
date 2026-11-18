#!/usr/bin/env python3
"""Install the packaged CLI into a temporary directory and exercise its public commands."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def verify(binary, project, runner=()):
    def invoke(args, *, code=0, stdout=None, stderr=""):
        result = subprocess.run([*runner, binary, *args], cwd=ROOT, capture_output=True,
                                encoding="utf-8", timeout=30)
        if (result.returncode != code or (stdout is not None and result.stdout != stdout)
                or result.stderr != stderr):
            raise SystemExit(f"FAIL {args!r}\n{result!r}")
        print(f"PASS {args[0]} {' '.join(map(str, args[1:]))} (exit {code})")
        return result.stdout

    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    invoke(["--version"], stdout=f"mn {version}\n")
    if "mn run FILE" not in invoke(["--help"]):
        raise SystemExit("help output is missing the run command")
    invoke(["init", project])
    if not (project / "mn.toml").is_file():
        raise SystemExit("init did not create mn.toml")
    invoke(["check", project / "main.mn"], stdout="")
    invoke(["run", project / "main.mn"], stdout="Hello, Mongolia!\n")

    cases = json.loads((ROOT / "verification/examples.json").read_text(encoding="utf-8"))
    files = sorted(p.relative_to(ROOT).as_posix() for p in (ROOT / "examples").rglob("*.mn"))
    if sorted(c["file"] for c in cases) != files:
        raise SystemExit("example inventory differs from the expected outputs")
    for case in cases:
        path = case["file"]
        args = [path, "--lang", "mn" if path.endswith("_mn.mn") else "en"]
        invoke(["check", *args], code=case["exit_code"], stdout="", stderr=case["stderr"])
        invoke(["run", *args], code=case["exit_code"], stdout=case["stdout"], stderr=case["stderr"])


if __name__ == "__main__":
    with tempfile.TemporaryDirectory(prefix="mn-install-") as tmp:
        location = Path(tmp)
        install = location / "install"
        environment = dict(os.environ, CARGO_TARGET_DIR=str(location / "target"))
        subprocess.run(["cargo", "install", "--locked", "--path", "crates/mn-cli",
                        "--root", install], cwd=ROOT, env=environment, check=True)
        verify(install / "bin" / ("mn.exe" if os.name == "nt" else "mn"), location / "hello")
