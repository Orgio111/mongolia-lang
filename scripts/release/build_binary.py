"""Build twice offline and package one native/cross target with its build inputs and notices."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[2]
TARGETS = ("x86_64-unknown-linux-musl", "aarch64-unknown-linux-musl",
           "x86_64-pc-windows-msvc", "x86_64-apple-darwin", "aarch64-apple-darwin")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def build(folder, target):
    folder.mkdir()
    for name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml"):
        shutil.copy2(ROOT / name, folder / name)
    shutil.copytree(ROOT / "crates", folder / "crates")
    home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")).resolve()
    env = dict(os.environ, CARGO_TARGET_DIR=str(folder / "target"), SOURCE_DATE_EPOCH="0",
               CARGO_INCREMENTAL="0", CARGO_NET_OFFLINE="true")
    flags = [f"--remap-path-prefix={folder}=/mn", f"--remap-path-prefix={home}=/cargo"]
    if target.endswith("msvc"):
        flags += ["-Clink-arg=/Brepro"]
    env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(flags)
    if target.endswith("linux-musl"):
        env["CARGO_TARGET_" + target.upper().replace("-", "_") + "_LINKER"] = "rust-lld"
    cmd = ["cargo", "build", "--release", "--locked", "--offline", "--workspace", "--target", target]
    subprocess.run(cmd, cwd=folder, env=env, check=True)
    filename = "mn.exe" if "windows" in target else "mn"
    return (folder / "target" / target / "release" / filename).read_bytes(), filename, cmd


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("target", choices=TARGETS)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as td:
        data, name, cmd = build(Path(td) / "one", args.target)
        again, _, _ = build(Path(td) / "independent-two", args.target)
    if data != again:
        raise SystemExit("Independent binaries differ; no package generated")
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    sysroot = Path(subprocess.check_output(["rustc","--print","sysroot"],cwd=ROOT,text=True).strip())
    docs = sysroot / "share/doc/rust"
    files = {f"bin/{name}":data}
    for n in ("LICENSE","LICENSE-MIT","LICENSE-APACHE","release/SBOM.spdx.json","INSTALLATION_GUIDE.md"):
        files[Path(n).name] = (ROOT/n).read_bytes()
    for p in (ROOT/"release/third-party").rglob("*"):
        if p.is_file():
            files["notices/"+p.relative_to(ROOT/"release/third-party").as_posix()] = p.read_bytes()
    files["notices/rust/COPYRIGHT-library.html"] = (docs/"COPYRIGHT-library.html").read_bytes()
    for p in (docs/"licenses").glob("*.txt"):
        files["notices/rust/licenses/"+p.name] = p.read_bytes()
    if args.target.endswith("musl"):
        files["notices/musl/COPYRIGHT"] = (ROOT/"release/MUSL-COPYRIGHT").read_bytes()
    inputs = [ROOT/n for n in ("Cargo.toml","Cargo.lock","rust-toolchain.toml")]
    inputs += sorted(p for p in (ROOT/"crates").rglob("*") if p.is_file())
    files["BUILD.json"] = (json.dumps(dict(version=version,target=args.target,binary=f"bin/{name}",
        bytes=len(data),sha256=sha(data), independent_builds=2,command=cmd,
        rustc=subprocess.check_output(["rustc","-vV"],cwd=ROOT,text=True).strip(),
        inputs={p.relative_to(ROOT).as_posix():sha(p.read_bytes()) for p in inputs}),indent=2)+"\n").encode()
    files["SHA256SUMS"] = "".join(f"{sha(b)}  {n}\n" for n,b in sorted(files.items())).encode()
    output = args.output / f"mn-{version}-{args.target}.zip"
    output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(output, "w") as archive:
        for n,b in sorted(files.items()):
            info = zipfile.ZipInfo(n, (2026,11,18,0,0,0))
            info.create_system = 3
            info.external_attr = (0o100755 if n.startswith("bin/") else 0o100644) << 16
            archive.writestr(info,b,compress_type=zipfile.ZIP_DEFLATED,compresslevel=9)
    print(f"{sha(output.read_bytes())}  {output.name}")


if __name__ == "__main__":
    main()
