use mn_core::{
    engine, frontend,
    shape::{require_equal, Dimension::*, Equality::*},
};

fn dump(src: &str) -> String {
    let p = frontend::parse(src).unwrap();
    engine::validate(&p).unwrap().hir_dump()
}
#[test]
fn canonical_function_ids_slots_and_nfc() {
    let english = "fn main(){let café=add(10);print(cafe\u{301});}fn add(v:i64)->i64{return v+2;}";
    let mn="функц main(){хувьсагч утга=нэм(10);хэвлэх(утга);}функц нэм(тоо:бүхэл64)->бүхэл64{буцаах тоо+2;}";
    let ir = dump(english);
    assert_eq!(ir, dump(mn));
    assert!(ir.contains("Call(1)"));
    assert!(ir.contains("Read(0)"));
    assert_eq!(ir, dump(english));
}
#[test]
fn frame_initialization_and_separate_namespaces() {
    assert_eq!(
        engine::run(
            &frontend::parse("fn f()->i64{return 9;}fn main(){let f=f();print(f);}").unwrap()
        )
        .unwrap(),
        "9\n"
    );
    for (src, code) in [
        ("fn main(){let x=x;}", "E_NAME"),
        ("fn f(a:i64){}fn main(){print(a);}", "E_NAME"),
        (
            "fn bad()->i64{return 1;print(0);}fn main(){}",
            "E_UNREACHABLE",
        ),
    ] {
        assert_eq!(
            engine::check(&frontend::parse(src).unwrap())
                .unwrap_err()
                .code,
            code
        );
    }
}
#[test]
fn generated_bilingual_casts_and_tensor_types() {
    for k in -16..17 {
        let a = format!("fn main(){{let t=[[f64({k})]];print(t@[[2.0]]);}}");
        let b = format!("функц main(){{let ж=[[бодит64({k})]];хэвлэх(ж@[[2.0]]);}}");
        assert_eq!(dump(&a), dump(&b));
        assert_eq!(
            engine::run(&frontend::parse(&b).unwrap()).unwrap(),
            format!("[[{:.1}]]\n", 2.0 * f64::from(k))
        );
    }
}
#[test]
fn explicit_cast_boundaries_and_errors() {
    for (expr, out) in [
        ("i64(-3.9)", "-3"),
        ("бодит64(9007199254740993)", "9007199254740992.0"),
        ("i64(-9223372036854775808.0)", "-9223372036854775808"),
        ("f64(-0.0)", "-0.0"),
    ] {
        let p = frontend::parse(&format!("fn main(){{print({expr});}}")).unwrap();
        assert_eq!(engine::run(&p).unwrap(), format!("{out}\n"));
    }
    for expr in ["i64(1.0/0.0)", "i64(0.0/0.0)", "i64(9223372036854775808.0)"] {
        let p = frontend::parse(&format!("fn main(){{print(42);print({expr});}}")).unwrap();
        engine::check(&p).unwrap();
        assert_eq!(engine::run(&p).unwrap_err().code, "E_CAST_RANGE");
    }
    let s = "функц main(){хэвлэх(бүхэл64(үнэн));}";
    let e = engine::check(&frontend::parse(s).unwrap()).unwrap_err();
    assert_eq!(e.code, "E_CAST");
    assert_eq!(&s[e.span.start..e.span.end], "бүхэл64(үнэн)");
    assert!(mn_core::diagnostics::render_detail(&e, "mn").contains("бүхэл64"));
    for e in ["i64()", "f64(1,2)", "бүхэл64("] {
        assert!(frontend::parse(&format!("fn main(){{print({e});}}")).is_err())
    }
}
#[test]
fn symbolic_and_dynamic_axes_require_identity() {
    let a = Symbol { binder: 1, axis: 0 };
    let b = Symbol { binder: 2, axis: 0 };
    let c = Runtime { value: 1, axis: 0 };
    assert_eq!(require_equal(a, a), Proven);
    assert_eq!(require_equal(a, b), Guard(a, b));
    assert_eq!(require_equal(a, c), Guard(a, c));
    assert_eq!(require_equal(Known(2), Known(3)), Contradiction);
    assert_eq!(require_equal(Known(0), Known(0)), Contradiction);
}
