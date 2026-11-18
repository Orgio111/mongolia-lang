"""Expose the workspace version to Actions and reject a mismatched release tag."""
import os
from pathlib import Path
import tomllib

root = Path(__file__).resolve().parents[2]
version = tomllib.loads((root / "Cargo.toml").read_text())["workspace"]["package"]["version"]
ref = os.environ.get("GITHUB_REF", "")
if ref.startswith("refs/tags/") and ref != f"refs/tags/v{version}":
    raise SystemExit(f"Expected release tag v{version}")
if "GITHUB_OUTPUT" in os.environ:
    with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as output:
        print(f"version={version}", file=output)
print(version)
