# Contributor and AI assistant instructions

Compiler version follows Cargo.toml. Check VERSIONING.md, SPEC.md and release/ before changing package, language or distribution claims.

This repository implements a narrow v0.1 foundation. Read SPEC.md, STATUS.md and BUILD_REPORT.md.
Use the current source as the implementation evidence. Future design documents are proposals.
Do not fabricate native code generation, AD, borrowing, package installation or GPU execution.
Do not silently change language semantics. Maintain one canonical token and semantic path for
English and Mongolian. Preserve byte spans and NFC identifier handling. Validate before running.
Use checked integer/allocation arithmetic, ordinary Result errors and safe Rust. Do not introduce
panic-based user-input handling, todo! or unimplemented! in supported paths. Test every claimed
feature and its negative cases. Record actual build/test/fmt/clippy outcomes. Keep dependency
lockfiles and implementation docs current. Split crates only for working components.
