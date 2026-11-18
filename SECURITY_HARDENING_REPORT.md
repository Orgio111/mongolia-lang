# Foundation security hardening · mn 0.1.1

The Phase 8 audit recorded fixes for its concrete findings and identified residual risks without assigning critical/high severities. This patch addresses the actionable source-level gaps below. Assurance for public distribution and service isolation remains limited to the evidence listed here.

| Boundary | Change | Verification |
|---|---|---|
| Identifier identity | NFC precedes canonical keyword and identifier checks. Classify Latin and Cyrillic by the pinned Script table and retain the historical Cyrillic range. | Extended/precomposed/decomposed identifiers, alias conformance and original byte spans. See the regression suite. |
| Terminal diagnostics | Bound rendered context and detail independently. Preserve the absolute scalar column. Render control/bidi characters and Unicode line/paragraph separators inert in diagnostics. | Long-line, separator and partial UTF-8 subprocess tests. Raw program output is under program control. |
| Validation boundary | Reject nonfinite literal payloads even for caller-built ASTs. Shared source-size constant prevents CLI/core drift. Static tensor shapes and all functions are checked before execution. | Public-AST, shape-cap, unused-function and no-partial-output tests. |
| Runtime allocations | Recheck bytes and element ceiling at the final tensor allocator. Reserve call-frame/argument storage fallibly. Charge matrix work before reserving output. | Direct allocator boundary tests, 32-bit emulation, independent matrix oracle and actual memory-pressure probe. |
| Local filesystem | Exclusive creation, file synchronization, best-effort rollback of acquired paths and reporting of cleanup failure. | Injected I/O failures, retry and pre-existing paths. Trusted parent directory required. |
| Dependency and CI integrity | Added pinned unicode-script 0.5.8 to the two lockfiles. Checkout and Python setup actions use commit IDs. CI token has read-only contents permission. | Checksums matched for all 14 distinct locked registry packages across both workspaces. cargo-audit 0.22.1 scanned both locks against 1,533 advisories at revision 472a4c9a785030434f15d7da84964d60fbe789ab; neither scan reported a vulnerability. |

## Memory pressure experiment

A Linux child process computed (240×1) @ (1×240), then printed 42. With RLIMIT_AS of 3,700 and 3,900 KiB, it returned E_ALLOCATION, exit 1 and empty stdout. At 4,100, 4,600 and 8,192 KiB it succeeded. At 3,500 KiB a host allocator failure terminated the process with SIGABRT. This bounds the recovery claim to selected reservations. Thresholds depend on the binary, loader and address-space layout. Reproduce with `python3 scripts/probe_allocation.py target/release/mn` on Linux. [Raw observations](verification/phase9/allocation-probe.json).

## Fuzzing and residual exposure

The lexer, parser/checker and validated runtime each completed 50,000 ASan-instrumented runs with seed 20261118, maximum input 4,096 bytes and 10-second per-input timeout. The runtime checks repeatability of results from a fresh run of the same checked program and UTF-8 boundaries on runtime error spans. The first LeakSanitizer run failed at shutdown under the host tracing restriction. The completed runs used detect_leaks=0. CI requests leak-enabled runs. [Lexer](verification/phase9/fuzz-lexer.log) · [Parser](verification/phase9/fuzz-parser.log) · [Runtime](verification/phase9/fuzz-runtime.log) · [Leak attempt](verification/phase9/fuzz-leak-attempt.log).

Workspace crate roots forbid unsafe Rust. The compiler process, allocator, external dependencies and toolchain remain trusted. The Unicode policy covers selected confusables and does not implement full UTS #39. Untrusted service hosting needs external process isolation and total memory/time budgets. See [known limitations](KNOWN_LIMITATIONS.md), [dependency inventory](verification/phase9/dependencies.json), [core scan](verification/phase9/audit-core.log), [fuzz scan](verification/phase9/audit-fuzz.log), [Unicode UTS #39](https://www.unicode.org/reports/tr39/) and [RustSec](https://rustsec.org/).
