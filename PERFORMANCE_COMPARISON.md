# Phase 11 compiler cost on the Phase 10 fixture

Both versions used Rust 1.100.0 and the same 15,162-byte frontend fixture. The pair ran on one Linux CPU with warm file caches, alternating AB/BA across six batches. Figures are medians of batch medians. In-process times include construction and exclude destruction of the final result. CLI time includes startup and file loading. Separate wait4 measurements record maximum resident set size of the check process. These are compiler measurements; the program itself did not run.

| Observation | Phase 10 | Phase 11 | Difference |
|---|---:|---:|---:|
| Validate a parsed AST | 40.654 µs | 57.098 µs | +40.5% |
| Lex, parse and validate | 258.534 µs | 269.942 µs | +4.4% |
| Complete mn check process | 1.746 ms | 1.756 ms | +0.6% |
| Peak process RSS | 1,636.000 KiB | 1,680.000 KiB | +2.7% |

In the separate release-only stage probe, median resolution + inference + HIR construction was 42.90 µs; independent verifier traversal was 7.98 µs. Type inference and HIR construction share a traversal, so their timings cannot be independently attributed. The Phase 10 validation measurement includes its name resolution and type checker. Phase 10 has no HIR generation or verifier stage.

The retained typed operations and verification pass add measurable semantic cost. The checker shares one function-ID/signature table between lowering and the AST oracle and emits operations in postorder without a second unresolved tree. Future optimization work should profile operation storage and allocations before changing this boundary. No cross-hardware or runtime speed claim follows.

Reproduce by building each workspace with cargo build --release --locked --workspace in separate target directories. From the candidate root run python3 scripts/phase11_compare.py BASE_TARGET CANDIDATE_TARGET. The script compiles the same Rust driver against each library, discards ten of 60 in-process observations and five of 20 CLI/RSS observations per batch. Run the ignored release test phase11_stage_times for the candidate stage breakdown (ten warmups, fifty samples). See verification/phase11/performance.json and cmd-10.log. Other host load and CPU frequency were uncontrolled.
