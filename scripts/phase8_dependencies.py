"""Inventory the locked crates against a local, pinned RustSec advisory-db checkout (Python 3.11+)."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tomllib

root = Path(__file__).resolve().parents[1]
db = Path(sys.argv[1]).resolve()
packages = {}
for lock in ["Cargo.lock", "fuzz/Cargo.lock"]:
    for pkg in tomllib.loads((root / lock).read_text())["package"]:
        if "source" not in pkg:
            continue
        key = (pkg["name"], pkg["version"])
        item = packages.setdefault(key, dict(name=key[0], version=key[1], locks=[],
                                              checksum=pkg["checksum"], records=[]))
        assert item["checksum"] == pkg["checksum"]
        item["locks"].append(lock)
for item in packages.values():
    for file in sorted((db / "crates" / item["name"]).glob("*.md")):
        metadata = tomllib.loads(file.read_text().split("```toml", 1)[1].split("```", 1)[0])
        assert metadata["advisory"]["package"] == item["name"]
        item["records"].append({"id":metadata["advisory"]["id"], "versions":metadata.get("versions", {}),
                                "withdrawn":metadata["advisory"].get("withdrawn")})
    cache = Path.home() / ".cargo/registry/cache"
    archives = list(cache.glob(f'*/{item["name"]}-{item["version"]}.crate'))
    item["cached_checksum_verified"] = bool(archives) and all(
        hashlib.sha256(p.read_bytes()).hexdigest() == item["checksum"] for p in archives)
print(json.dumps({"database":"https://github.com/RustSec/advisory-db",
                  "revision":subprocess.check_output(["git", "-C", db, "rev-parse", "HEAD"],text=True).strip(),
                  "packages":sorted(packages.values(),key=lambda p:p["name"])},indent=2,default=str))
