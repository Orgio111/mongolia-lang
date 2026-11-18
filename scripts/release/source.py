"""Create a deterministic source zip from tracked paths or a verified source export."""
import argparse
import hashlib
from pathlib import Path
import subprocess
import zipfile

ROOT = Path(__file__).resolve().parents[2]


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("output", type=Path)
    args = p.parse_args()
    git_root = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=ROOT,
                              capture_output=True, text=True)
    if git_root.returncode == 0 and Path(git_root.stdout.strip()).resolve() == ROOT:
        listing = subprocess.check_output(["git","ls-files","-z"],cwd=ROOT).decode().split("\0")
        names = sorted(n for n in listing if n and n != "SHA256SUMS")
    else:
        recorded = dict(line.split("  ",1)[::-1] for line in (ROOT/"SHA256SUMS").read_text().splitlines())
        names = sorted(recorded)
        for name in names:
            if hashlib.sha256((ROOT/name).read_bytes()).hexdigest() != recorded[name]:
                raise SystemExit(f"Export differs from SHA256SUMS: {name}")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    inventory = []
    with zipfile.ZipFile(args.output,"w") as archive:
        def put(name,data):
            member = zipfile.ZipInfo("mongolia-lang/"+name,(2026,11,18,0,0,0))
            member.create_system = 3
            member.external_attr = 0o100644 << 16
            archive.writestr(member,data,compress_type=zipfile.ZIP_DEFLATED,compresslevel=9)
        for name in names:
            path = ROOT/name
            if path.is_symlink() or not path.is_file():
                raise SystemExit(f"Expected regular file: {name}")
            data = path.read_bytes()
            inventory.append(f"{hashlib.sha256(data).hexdigest()}  {name}\n")
            put(name,data)
        put("SHA256SUMS","".join(inventory).encode())
    print(hashlib.sha256(args.output.read_bytes()).hexdigest(),args.output.name)


if __name__ == "__main__":
    main()
