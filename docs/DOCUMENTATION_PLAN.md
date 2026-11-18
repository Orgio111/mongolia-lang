# Documentation plan

Current entry points: README.md for build/run, SPEC.md and grammar.ebnf for syntax,
ARCHITECTURE.md for implementation boundaries, FUTURE_DESIGN.md for proposals and ROADMAP.md for
gates. BUILD_REPORT.md is the authoritative local verification record.

Add paired English/Mongolian tutorials covering literals, functions, immutability, tensors and
shape diagnostics; run every supported example in CI. Native speakers must review terminology.
Each reference page needs status, minimum version, type/effect rules, errors and runnable tests.
Compiler-internals pages track IR verifiers and pass invariants. AD/GPU/FFI/package guides remain
proposal documents until backend fixtures run. Generate API docs from actual signatures when the
standard library exists. A documentation build should reject broken references and future examples
accidentally included in the executable corpus. Machine-readable grammar assists tools but does not
replace semantic rules; AI tools may not automatically discover llms.txt.
