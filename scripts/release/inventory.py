"""Verify locked package archives and reviewed license expressions. Generate SPDX and notices."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tomllib

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "release"


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def main():
    policy = json.loads((OUT / "reviewed-licenses.json").read_text())
    inventory, packages, links, notices = {}, [], [], {}
    for workspace in (ROOT, ROOT / "fuzz"):
        lock = tomllib.loads((workspace / "Cargo.lock").read_text())
        hashes = {(p["name"], p["version"]): p["checksum"] for p in lock["package"] if "source" in p}
        meta = json.loads(subprocess.check_output(
            ["cargo", "metadata", "--locked", "--offline", "--format-version", "1"], cwd=workspace))
        ids = {p["id"]: f'SPDXRef-{p["name"]}-{p["version"]}' for p in meta["packages"]}
        for p in meta["packages"]:
            name, version = p["name"], p["version"]
            entry = dict(SPDXID=ids[p["id"]], name=name, versionInfo=version,
                         downloadLocation="NOASSERTION", filesAnalyzed=False,
                         licenseConcluded="NOASSERTION", copyrightText="NOASSERTION",
                         licenseDeclared=(p["license"] or "NOASSERTION").replace("MIT/Apache-2.0", "MIT OR Apache-2.0"))
            if p["source"]:
                key = f"{name}@{version}"
                if policy.get(key) != p["license"]:
                    raise SystemExit(f"License review required: {key} {p['license']}")
                registry = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")) / "registry/cache"
                filename = f"{name}-{version}.crate"
                archives = list(registry.glob(f"*/{filename}"))
                checksum = hashes[name, version]
                if not archives or any(hashlib.sha256(a.read_bytes()).hexdigest() != checksum for a in archives):
                    raise SystemExit(f"Missing or mismatched cached archive: {filename}")
                inventory.setdefault(key, dict(license=p["license"], sha256=checksum, locks=[]))["locks"].append(
                    (workspace / "Cargo.lock").relative_to(ROOT).as_posix())
                entry.update(downloadLocation=f"https://static.crates.io/crates/{name}/{filename}",
                             checksums=[dict(algorithm="SHA256", checksumValue=checksum)])
                if workspace == ROOT:
                    with tarfile.open(archives[0]) as archive:
                        for member in archive:
                            parts = Path(member.name).parts
                            if len(parts) == 2 and member.isfile() and parts[1].upper().startswith(("LICENSE","COPYRIGHT","NOTICE")):
                                notices[f"{name}-{version}/{parts[1]}"] = archive.extractfile(member).read()
            if workspace == ROOT:
                packages.append(entry)
        if workspace == ROOT:
            for node in meta["resolve"]["nodes"]:
                for dep in node["dependencies"]:
                    links.append(dict(spdxElementId=ids[node["id"]], relationshipType="DEPENDS_ON",
                                      relatedSpdxElement=ids[dep]))
    if set(inventory) != set(policy):
        raise SystemExit("License policy and the two resolved workspaces differ")
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    lock_hash = hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest()
    sbom = dict(spdxVersion="SPDX-2.3", dataLicense="CC0-1.0", SPDXID="SPDXRef-DOCUMENT",
                name=f"mn-{version}-cargo", documentNamespace=f"urn:mn:spdx:{lock_hash}",
                creationInfo=dict(created="2026-11-18T00:00:00Z", creators=["Tool: Mongolia release inventory 1"]),
                documentDescribes=[f"SPDXRef-mn-cli-{version}"], packages=sorted(packages,key=lambda x:x["name"]),
                relationships=sorted(links,key=lambda x:(x["spdxElementId"],x["relatedSpdxElement"])))
    write_json(OUT / "SBOM.spdx.json", sbom)
    write_json(OUT / "dependency-inventory.json", dict(sorted(inventory.items())))
    dest = OUT / "third-party"
    for name,data in notices.items():
        path = dest/name
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_bytes(data)
    if {p.relative_to(dest).as_posix() for p in dest.rglob("*") if p.is_file()} != set(notices):
        raise SystemExit("Obsolete notice files in release/third-party")
    print(f"{len(inventory)} registry hashes and licenses, {len(packages)} production SBOM packages, {len(notices)} notices")


if __name__ == "__main__":
    main()
