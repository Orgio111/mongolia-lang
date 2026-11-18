//! Release-only timing probe, explicitly selected by scripts/phase8_benchmark.py.
use super::{lex, parse, parse_tokens};
use crate::engine;
use std::{hint::black_box, time::Instant};

#[test]
#[ignore = "measurement, run via scripts/phase8_benchmark.py"]
fn stage_measurements() {
    let source = include_str!("../../../../benchmarks/data/frontend.mn");
    let tokens = lex(source).unwrap();
    let program = parse(include_str!("../../../../benchmarks/data/matmul.mn")).unwrap();
    let checked = engine::validate(&program).unwrap();
    println!("INPUT {} {}", source.len(), tokens.len() - 1);
    for index in 0..40 {
        let start = Instant::now();
        let result = black_box(lex(black_box(source)).unwrap());
        let lex_ns = start.elapsed().as_nanos();
        drop(result);

        // The parser consumes its input. Restore one identical stream outside the timed region.
        let input = tokens.clone();
        let start = Instant::now();
        let ast = black_box(parse_tokens(input).unwrap());
        let parse_ns = start.elapsed().as_nanos();
        drop(ast);

        let start = Instant::now();
        black_box(checked.run().unwrap());
        let run_ns = start.elapsed().as_nanos();
        println!("TIMING {index} {lex_ns} {parse_ns} {run_ns}");
    }
}
