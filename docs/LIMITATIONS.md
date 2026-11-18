# Known limitations and release gates

This deliverable implements a bounded reference interpreter, not all requested future language features.

- f32/i32 and other scalar widths are reserved/proposed, not supported types. No implicit casts.
- No control flow, mutable binding, modules, closures, arrays, structs, generics, dynamic shapes,
  broadcasting, differentiation, native compilation, GPU, FFI, package resolution or language server.
- Rank-2 f64 tensors only; dimensions must be positive. Tensor reads copy their buffers.
- No recovery after the first diagnostic; source diagnostics use scalar columns, not terminal display
  width. Tabs, combining characters and wide glyphs can make caret alignment imperfect.
- Mongolian catalog and vocabulary need native-speaker review. Some technical details remain English.
- Confusable checks are a conservative Latin/Cyrillic subset, not full UTS #39 conformance;
  mixed-script detection does not cover every Unicode script or extended Latin character.
- Limits intentionally favor predictable failures: 1 MiB source; 100,000 tokens; bounded expressions,
  calls, runtime work and output. This is not a security sandbox. Host process isolation is still needed.
- Fallible reserve is used for runtime tensor buffers; standard compiler allocations and buffer clones
  elsewhere can still encounter process-level OOM. No complete recovery from host memory exhaustion.
- No typed IR yet; validated AST is the reference execution representation. No native/VM/GPU
  backend equivalence or gradient checks can run until those backends exist.
- Cross-platform CI is supplied; only the actual local verification platform in BUILD_REPORT is tested here.
- Project name, CLI collisions, trademarks and registry namespace availability remain unverified;
  these are public release gates, not claims of ownership.
- License texts are supplied; maintainers must establish contribution provenance and project ownership
  before public distribution. Governance roles are proposals, not appointed people.

See STATUS for phase evidence and ROADMAP for measurable next milestones.
