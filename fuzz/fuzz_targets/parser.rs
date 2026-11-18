#![no_main]

use libfuzzer_sys::fuzz_target;
use mn_core::{engine, frontend};

fuzz_target!(|source: &str| {
    let result = frontend::parse(source).and_then(|program| engine::check(&program));
    if let Err(e) = result {
        assert!(source.get(e.span.start..e.span.end).is_some());
    }
});
