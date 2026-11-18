"""Compare locked release binaries built offline in independent absolute source/target trees."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def build(directory):
    directory.mkdir()
    for name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml"):
        shutil.copy2(ROOT / name, directory / name)
    shutil.copytree(ROOT / "crates", directory / "crates")
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")).resolve()
    flags = [f"--remap-path-prefix={directory}=/mn", f"--remap-path-prefix={cargo_home}=/cargo"]
    env = dict(os.environ, CARGO_NET_OFFLINE="true", CARGO_INCREMENTAL="0",
               CARGO_TARGET_DIR=str(directory / "target"),
               CARGO_ENCODED_RUSTFLAGS="\x1f".join(flags), SOURCE_DATE_EPOCH="0")
    subprocess.run(["cargo", "build", "--workspace", "--locked", "--release", "--offline"],
                   cwd=directory, env=env, check=True)
    binary = directory / "target/release" / ("mn.exe" if os.name == "nt" else "mn")
    return {"bytes": binary.stat().st_size,
            "sha256": hashlib.sha256(binary.read_bytes()).hexdigest()}


if __name__ == "__main__":
    with tempfile.TemporaryDirectory() as tmp:
        a = build(Path(tmp) / "tree-one")
        b = build(Path(tmp) / "longer-source-path-two")
    print(json.dumps({"rust": subprocess.check_output(["rustc", "-vV"], cwd=ROOT, text=True),
                      "first": a, "second": b, "equal": a == b}, indent=2))
    if a != b:
        raise SystemExit("Release binaries differ")
