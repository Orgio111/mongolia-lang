# Mongolia v0.1.1 release limits

The language and resource limits below also apply to compiler 0.1.2. [Platform support](PLATFORM_SUPPORT_MATRIX.md) and [release readiness](RELEASE_READINESS.md) supersede the Phase 9 distribution status.

| Boundary | Observed behavior and consequence |
|---|---|
| Language surface | The executable core supports immutable bindings, typed functions, scalar values and positive, static rank-2 f64 tensors on the host. Dynamic shapes, branches, loops, HIR/VM, AD, native lowering, GPUs, FFI, package resolution and the LSP remain proposals. SPEC.md defines what programs can run. |
| Compatibility | The patch can reject an identifier that combines extended Latin and Cyrillic. Use a single script per name. File-wide confusable collisions can flag independent names in different scopes. Other scripts, Script_Extensions and the full UTS #39 repertoire need a separately reviewed policy. |
| Locations and languages | Columns count original Unicode scalars. Terminal cell width, grapheme clusters, multiple labels and LSP coordinates require additional work. Some declaration spans cover a whole function. The Mongolian catalog still admits English fallback. Native-speaker acceptance is pending. |
| Resource accounting | Source, token, expression, tensor element, evaluation and output budgets are separate. A statement can exhaust its 256 expression entries before a tensor reaches the element cap. Multiple live values and compiler data structures have no shared heap quota. Parsing and OS allocation can terminate the process under severe pressure. Direct AST callers supply trusted in-memory structures and their own host limits. |
| Filesystem | Scaffold rollback handles ordinary write/create failures inside a trusted directory. Power loss or another process replacing an entry during rollback can leave an incomplete project or change which object occupies that path. There is no two-file crash transaction. |
| Platforms and dependencies | Host CLI, reproducibility and profiles were tested on Linux x86_64. Two additional Linux musl core suites ran under QEMU. Hosted Windows/macOS, real 32-bit/ARM devices and cross-host identical builds have no Phase 9 execution record. Fresh installs need a Rust toolchain, linker and registry crates or a prepared local cache. |
| Security evidence | Fuzz limits were 50,000 iterations per target and 4,096 bytes per input. LeakSanitizer could not complete locally. Advisory results refer to the recorded database revision. Process isolation and limits for running untrusted programs remain the host's responsibility. |
| Performance | One host, one CPU affinity and warm fixtures. Lexer/CLI timings vary between batches. RSS and executable size increased. Other workloads, devices, backends and long-running memory behavior require independent measurement. |
| Distribution | Source archive with checksums. Maintainer identity, signature, disclosure channel verification, name/namespace review and native vocabulary approval remain public release gates. |

See [regression evidence](REGRESSION_TEST_REPORT.md), [security evidence](SECURITY_HARDENING_REPORT.md), [measurements](PERFORMANCE_COMPARISON.md) and the [future capability matrix](EXPANSION_REQUIREMENTS_MATRIX.md).
