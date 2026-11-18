"""Verify archive contents, install a checked copy, and test the packaged CLI."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import zipfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
from verify_install import verify


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path)
    parser.add_argument("--runner", help="Userspace emulator for the target")
    args = parser.parse_args()
    runner = [args.runner] if args.runner else []
    with tempfile.TemporaryDirectory() as work, zipfile.ZipFile(args.archive) as z:
        files = z.namelist()
        sums = z.read("SHA256SUMS").decode().splitlines()
        expected = dict(line.split("  ", 1)[::-1] for line in sums)
        if len(files) != len(set(files)) or len(expected) != len(sums) or set(expected) != set(files) - {"SHA256SUMS"}:
            raise SystemExit("Invalid package inventory")
        for name, digest in expected.items():
            if hashlib.sha256(z.read(name)).hexdigest() != digest:
                raise SystemExit(f"Invalid file checksum: {name}")
        info = json.loads(z.read("BUILD.json"))
        name = "mn.exe" if os.name == "nt" else "mn"
        entry = "bin/" + name
        if args.archive.name != f"mn-{info['version']}-{info['target']}.zip" or info["binary"] != entry or info["sha256"] != expected[entry]:
            raise SystemExit("Invalid build manifest")
        directory = Path(work)
        binary = directory/name
        binary.write_bytes(z.read(entry))
        dest = directory/"path with spaces"
        if os.name == "nt":
            command = ["pwsh", "-NoProfile", "-File", str(ROOT/"installers/install.ps1"),
                       "-Binary",str(binary),"-BinDirectory",str(dest),"-Sha256"]
        else:
            command = ["sh",str(ROOT/"installers/install.sh"),str(binary)]
        def install(digest):
            tail = [digest] if os.name == "nt" else [digest, str(dest)]
            return subprocess.run(command+tail,capture_output=True,text=True)
        if install("0"*64).returncode == 0 or (dest/name).exists():
            raise SystemExit("Installer accepted the wrong digest")
        good = install(expected[entry])
        if good.returncode:
            raise SystemExit(good.stderr)
        if install(expected[entry]).returncode == 0:
            raise SystemExit("Installer overwrote existing executable")
        verify(dest/name, directory/"hello", runner)
        for args2, code in ((["check",directory/"absent.mn"],2),(["build"],2),(["run"],2)):
            result = subprocess.run([*runner,dest/name,*args2],capture_output=True,timeout=30)
            if result.returncode != code or result.stdout or not result.stderr:
                raise SystemExit(f"Invalid CLI error result: {args2}")
        print(f"PASS {info['target']} archive checksums, installer, examples and exit statuses")


if __name__ == "__main__":
    main()
