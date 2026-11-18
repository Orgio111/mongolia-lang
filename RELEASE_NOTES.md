# Mongolia 0.1.2 release notes

This patch packages the bilingual v0.1 language for local installation and verification. The language and interpreter have the same semantics as 0.1.1. Existing .mn programs need no migration. The new release process compares two clean builds before packaging a binary, ships its inputs, checksums, SPDX inventory and license notices, and exercises the extracted binary with the installer and examples.

The kit contains source and two Linux musl executable packages. On x86-64 the CLI ran natively; AArch64 execution used QEMU. The Windows x64 and both macOS packages have build and native CI recipes. Refer to PLATFORM_SUPPORT_MATRIX.md before selecting a download. Install with Cargo, a verified binary copy or the local installer script. RELEASE_READINESS.md records the conditions for a public release.
