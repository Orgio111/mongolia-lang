#![no_main]
use libfuzzer_sys::fuzz_target;
use mn_core::{engine, frontend};

fuzz_target!(|source: &str| {
    if let Ok(program) = frontend::parse(source) {
        if let Ok(checked) = engine::validate(&program) {
            let result = checked.run();
            if let Err(e) = &result {
                assert!(source.get(e.span.start..e.span.end).is_some());
            }
            assert_eq!(result, checked.run());
        }
    }
});
