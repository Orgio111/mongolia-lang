# Candidate artifact manifest · 0.1.2

The outer kit contains mongolia-lang-0.1.2-source.zip, the two executable ZIPs below, release notes, the platform/readiness reports, the installation guide, this manifest and SHA256SUMS. The kit checksum index provides the digest of every member except itself, including the final source archive. The download has its own out-of-band SHA-256. The candidate was built locally from the verified Phase 9 source; no hosted commit, tag or maintainer signature is attached.

| ZIP | ZIP SHA-256 | Executable bytes | Executable SHA-256 |
|---|---|---:|---|
| mn-0.1.2-aarch64-unknown-linux-musl.zip | 68e56ac2054a13b98723144f6feda569db89890edee22864fa8f726a474aebe7 | 989,400 | fe1339511135c4d5685c445350a9575ebc5e8a5100d27f40eb6d8c1966f55b16 |
| mn-0.1.2-x86_64-unknown-linux-musl.zip | 7ad8d9470305bde6603352de38963e5a0c094ec9ed4293b7b43da04f050cf003 | 970,152 | a52f5a8cef5d4522009b3c4b7909cb8792788fd44a760703e0711ef42cce7bb9 |

Each binary ZIP contains bin/mn, BUILD.json, the three project license files, third-party notices, a production Cargo SPDX SBOM, the installation guide and an inner SHA256SUMS. Both packages use the default release profile with offline locked dependencies, path remapping, rust-lld for musl and two independent builds. Verified execution was native on x86-64 and emulated on AArch64. The source archive contains the full two-crate workspace, fuzz workspace, lockfiles, scripts, workflows, examples, specifications, historical evidence and Phase 10 logs. No compiler build cache or vendor directory is required in the source tree.
