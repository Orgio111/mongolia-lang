//! One bilingual frontend. Spans index the original UTF-8 bytes.
pub const MAX_SOURCE_BYTES: usize = 1_048_576;

mod ast;
mod lexer;
mod parser;
pub use ast::*;
pub use lexer::{lex, Token, TokenKind};
pub use parser::parse;

fn error(span: Span, code: &'static str, detail: impl Into<String>) -> Error {
    Error {
        span,
        code,
        detail: detail.into(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bilingual() {
        assert_eq!(
            lex("функц хувьсагч буцаах хэвлэх үнэн")
                .unwrap()
                .iter()
                .map(|t| t.kind.clone())
                .collect::<Vec<_>>(),
            lex("fn let return print true")
                .unwrap()
                .iter()
                .map(|t| t.kind.clone())
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn unicode_spans() {
        let s = "функц жин() {}";
        for t in lex(s).unwrap() {
            assert!(s.get(t.span.start..t.span.end).is_some());
        }
    }
    #[test]
    fn confusable_collision() {
        assert_eq!(lex("A А").unwrap_err().code, "E_CONFUSABLE");
    }
    #[test]
    fn nfc() {
        assert_eq!(
            lex("cafe\u{301}").unwrap()[0].kind,
            TokenKind::Word("café".into())
        );
    }
    #[test]
    fn mixed_identifier_rejected() {
        assert_eq!(lex("aА").unwrap_err().code, "E_SCRIPT");
    }
    #[test]
    fn malformed() {
        for s in [
            "fn main( {",
            "fn main() { let x = [[1.0],[2.0,3.0]]; }",
            "/*",
            "fn main() { print(1) }",
        ] {
            assert!(parse(s).is_err());
        }
    }
    #[test]
    fn precedence() {
        let p = parse("fn main() { print(1 + 2 * 3); }").unwrap();
        if let Stmt::Print(Expr {
            kind: ExprKind::Binary('+', _, r),
            ..
        }) = &p[0].body[0]
        {
            assert!(matches!(r.kind, ExprKind::Binary('*', _, _)));
        } else {
            panic!("unexpected AST");
        }
    }
    #[test]
    fn type_and_call() {
        assert!(parse("fn add(a: i64,b: i64)->i64 { return a+b; } функц main() { хувьсагч жин: Тензор<бодит64,[2,2]> = [[1.0,2.0],[3.0,4.0]]; хэвлэх(add(1,2)); }").is_ok());
    }
    #[test]
    fn depth_limit() {
        let s = format!(
            "fn main() {{ print({}1{}); }}",
            "(".repeat(200),
            ")".repeat(200)
        );
        assert!(parse(&s).is_err());
    }
}
