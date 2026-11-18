use mn_core::{engine, frontend};

#[test]
fn script_checks_use_normalized_unicode_scripts() {
    for name in ["éБ", "еé", "aꙀ", "e\u{301}Б"] {
        let source = format!("fn main() {{ let {name} = 1; }}");
        let err = frontend::lex(&source).unwrap_err();
        assert_eq!(err.code, "E_SCRIPT");
        assert_eq!(&source[err.span.start..err.span.end], name);
    }
    let p = frontend::parse("функц main(){let café=2; хувьсагч жин=3; хэвлэх(cafe\u{301}+жин);}")
        .unwrap();
    assert_eq!(engine::run(&p).unwrap(), "5\n");
}

#[test]
fn padded_minimum_has_the_same_integer_value() {
    let p = frontend::parse("fn main(){print(-0009223372036854775808);}").unwrap();
    assert_eq!(engine::run(&p).unwrap(), "-9223372036854775808\n");
    for s in ["0009223372036854775808", "-0009223372036854775809"] {
        assert_eq!(
            frontend::parse(&format!("fn main(){{print({s});}}"))
                .unwrap_err()
                .code,
            "E_NUMBER"
        );
    }
}

#[test]
fn return_mismatch_labels_the_returned_expression() {
    let s = "функц f()->i64{буцаах (үнэн);} fn main(){}";
    let err = engine::check(&frontend::parse(s).unwrap()).unwrap_err();
    assert_eq!(err.code, "E_TYPE");
    assert_eq!(&s[err.span.start..err.span.end], "(үнэн)");
}

#[test]
fn externally_supplied_nonfinite_literals_are_rejected() {
    for value in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
        let mut p = frontend::parse("fn main(){print(0.0);}").unwrap();
        let frontend::Stmt::Print(e) = &mut p[0].body[0] else {
            unreachable!()
        };
        e.kind = frontend::ExprKind::Float(value);
        assert_eq!(engine::run(&p).unwrap_err().code, "E_NUMBER");
    }
    let p = frontend::parse("fn main(){print(1.0/0.0);}").unwrap();
    assert_eq!(engine::run(&p).unwrap(), "inf\n");
}

#[test]
fn rectangular_matmul_preserves_contraction_order() {
    let p=frontend::parse("fn main(){let a=[[10000000000000000.0,-10000000000000000.0,1.0]];let b=[[1.0,2.0],[1.0,2.0],[1.0,3.0]];print(a@b);}").unwrap();
    assert_eq!(engine::run(&p).unwrap(), "[[1.0, 3.0]]\n");
}

#[test]
fn mongolian_limits_and_confusable_messages_identify_the_problem() {
    let err = frontend::lex("fn main(){let A=1;let А=2;}").unwrap_err();
    assert_eq!(err.code, "E_CONFUSABLE");
    let mn = mn_core::diagnostics::render_detail(&err, "mn");
    assert!(mn.contains("A, А") && !mn.contains("confusable"));
    let p = frontend::parse("fn main(){main();}").unwrap();
    let err = engine::run(&p).unwrap_err();
    assert_eq!(err.code, "E_LIMIT");
    assert!(!mn_core::diagnostics::render_detail(&err, "mn").contains("depth"));
}
