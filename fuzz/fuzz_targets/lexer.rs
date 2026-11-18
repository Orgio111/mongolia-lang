#![no_main]

use libfuzzer_sys::fuzz_target;
use mn_core::frontend::lex;

fuzz_target!(|bytes: &[u8]| {
    let source = String::from_utf8_lossy(bytes);
    match lex(&source) {
        Ok(tokens) => {
            let mut end = 0;
            for token in tokens {
                assert!(end <= token.span.start);
                assert!(source.get(token.span.start..token.span.end).is_some());
                end = token.span.end;
            }
            assert_eq!(end, source.len());
        }
        Err(e) => assert!(source.get(e.span.start..e.span.end).is_some()),
    }
});
