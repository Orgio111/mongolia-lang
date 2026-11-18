//! Deterministic bounded property and mutation tests, without external test dependencies.
use mn_core::diagnostics::render_detail;
use mn_core::engine::check;
use mn_core::frontend::{lex, parse, Error, Span};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.0
    }
    fn index(&mut self, len: usize) -> usize {
        (self.next() % len as u64) as usize
    }
}
fn assert_span(source: &str, span: Span) {
    assert!(span.start <= span.end);
    assert!(source.get(span.start..span.end).is_some());
}
fn exercise(source: &str) {
    match lex(source) {
        Ok(tokens) => {
            let mut previous = 0;
            for token in tokens {
                assert_span(source, token.span);
                assert!(token.span.start >= previous);
                previous = token.span.end;
            }
        }
        Err(error) => assert_span(source, error.span),
    }
    match parse(source) {
        Ok(program) => {
            for function in &program {
                assert_span(source, function.span);
            }
            if let Err(error) = check(&program) {
                assert_span(source, error.span);
            }
        }
        Err(error) => assert_span(source, error.span),
    }
}
#[test]
fn arbitrary_unicode_does_not_panic_and_spans_are_boundaries() {
    let mut rng = Rng(0x4d4f4e474f4c4941);
    for _ in 0..2048 {
        let length = rng.index(128);
        let mut source = String::new();
        for _ in 0..length {
            let value = (rng.next() % 0x110000) as u32;
            if let Some(ch) = char::from_u32(value) {
                source.push(ch);
            }
        }
        exercise(&source);
    }
}
#[test]
fn mutate_valid_programs_without_panics_or_invalid_spans() {
    let seeds = [
        "fn main() { print(1 + 2 * 3); }",
        "функц main() { хувьсагч жин = [[1.0,2.0],[3.0,4.0]]; хэвлэх(жин @ жин); }",
        "fn add(a:i64,b:i64)->i64{return a+b;} fn main(){print(add(12,30));}",
        "/* outer /* inner */ end */ fn main(){let café=\"hello\\n\";print(café);}",
    ];
    let alphabet: Vec<char> = "fnmainlet(){}[]+-*/@;,:=0123456789.\"\\ абвАБВ\n\t\u{301}\u{200d}💻"
        .chars()
        .collect();
    let mut rng = Rng(20261117);
    for seed in seeds {
        exercise(seed);
        for _ in 0..512 {
            let mut chars: Vec<char> = seed.chars().collect();
            let edits = 1 + rng.index(8);
            for _ in 0..edits {
                let at = rng.index(chars.len() + 1);
                match rng.index(3) {
                    0 if at < chars.len() => {
                        chars.remove(at);
                    }
                    1 if at < chars.len() => {
                        chars[at] = alphabet[rng.index(alphabet.len())];
                    }
                    _ => chars.insert(at, alphabet[rng.index(alphabet.len())]),
                }
            }
            exercise(&chars.into_iter().collect::<String>());
        }
    }
}
#[test]
fn bilingual_diagnostic_catalog_matches_golden_files() {
    let errors = [
        Error {
            span: Span::default(),
            code: "E_SHAPE",
            detail: "[2, 3] @ [2, 2]: inner dimensions 3 and 2 are incompatible".into(),
        },
        Error {
            span: Span::default(),
            code: "E_NAME",
            detail: "undefined identifier жин".into(),
        },
        Error {
            span: Span::default(),
            code: "E_TYPE",
            detail: "expected I64, found Bool".into(),
        },
        Error {
            span: Span::default(),
            code: "E_ARGUMENTS",
            detail: "add expects 2 arguments, found 1".into(),
        },
        Error {
            span: Span::default(),
            code: "E_DIV_ZERO",
            detail: "integer division by zero".into(),
        },
    ];
    for (lang, expected) in [
        ("en", include_str!("golden/diagnostics.en.txt")),
        ("mn", include_str!("golden/diagnostics.mn.txt")),
    ] {
        let actual: String = errors
            .iter()
            .map(|e| format!("{}: {}\n", e.code, render_detail(e, lang)))
            .collect();
        assert_eq!(actual, expected, "catalog language {lang}");
    }
}
