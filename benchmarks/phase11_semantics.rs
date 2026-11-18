// Compile this same driver against each release library. See PERFORMANCE_COMPARISON.md.
use mn_core::{engine, frontend};
use std::{env, fs, hint::black_box, time::Instant};
fn main() {
    let source = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let parsed = frontend::parse(&source).unwrap();
    for _ in 0..60 {
        let start = Instant::now();
        let checked = engine::validate(black_box(&parsed)).unwrap();
        let sem = start.elapsed().as_nanos();
        drop(black_box(checked));
        let start = Instant::now();
        let ast = frontend::parse(black_box(&source)).unwrap();
        let checked = engine::validate(&ast).unwrap();
        println!("SAMPLE {sem} {}", start.elapsed().as_nanos());
        drop(black_box(checked));
    }
}
