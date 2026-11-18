use mn_core::{engine, frontend};

fn checked(source: &str) -> Result<frontend::Program, frontend::Error> {
    let program = frontend::parse(source)?;
    engine::check(&program)?;
    Ok(program)
}

#[test]
fn negation_and_subtraction_have_distinct_domains() {
    let source = "функц main(){ хувьсагч жин=[[2.0]]; хэвлэх(-жин); }";
    let error = checked(source).unwrap_err();
    assert_eq!(error.code, "E_TYPE");
    assert_eq!(&source[error.span.start..error.span.end], "-жин");
    let ok = checked("fn main(){ print(-2); print(-2.0); print([[2.0]] - [[3.0]]); }").unwrap();
    assert_eq!(engine::run(&ok).unwrap(), "-2\n-2.0\n[[-1.0]]\n");
}

#[test]
fn minimum_integer_and_left_associativity() {
    let program =
        checked("fn main(){ print(-9223372036854775808); print(24/4/2); print(-7/2); }").unwrap();
    assert_eq!(
        engine::run(&program).unwrap(),
        "-9223372036854775808\n3\n-3\n"
    );
    assert_eq!(
        checked("fn main(){print(-(9223372036854775808));}")
            .unwrap_err()
            .code,
        "E_NUMBER"
    );
    let overflow = checked("fn main(){print(-9223372036854775808 / -1);}").unwrap();
    assert_eq!(engine::run(&overflow).unwrap_err().code, "E_OVERFLOW");
}

#[test]
fn lexical_rules_and_normalized_scope() {
    let source = r#"функц main(){ /* 1 /* 2 */ 3 */ let café="\t\"\\"; хэвлэх(café); }"#;
    assert_eq!(engine::run(&checked(source).unwrap()).unwrap(), "\t\"\\\n");
    for (source, code) in [
        ("fn main(){let café=1; let café=2;}", "E_DUPLICATE"),
        ("fn f(A:i64){} fn g(А:i64){} fn main(){}", "E_CONFUSABLE"),
        ("fn main(){let aА=1;}", "E_SCRIPT"),
        ("fn main(){let хэрэв=1;}", "E_PARSE"),
    ] {
        assert_eq!(checked(source).unwrap_err().code, code);
    }
}

#[test]
fn forward_calls_and_argument_effect_order() {
    let source="fn main(){let add=0; print(add(tap(1),tap(2)));} fn tap(x:i64)->i64{print(x);return x;} fn add(a:i64,b:i64)->i64{return a+b;}";
    assert_eq!(engine::run(&checked(source).unwrap()).unwrap(), "1\n2\n3\n");
    assert_eq!(
        checked("fn main(){} fn f(){return;print(0);}")
            .unwrap_err()
            .code,
        "E_UNREACHABLE"
    );
}

#[test]
fn tensor_dimensions_and_result_capacity() {
    for (source, code) in [
        ("fn f(a:Tensor<f64,[0,2]> ){} fn main(){}", "E_SHAPE"),
        ("fn f(a:Tensor<f64,[1001,1000]> ){} fn main(){}", "E_LIMIT"),
        (
            "fn f(a:Tensor<f64,[1001,1]>, b:Tensor<f64,[1,1000]>){print(a@b);} fn main(){}",
            "E_LIMIT",
        ),
        ("fn main(){print([[1.0,2.0]] + [[1.0]]);}", "E_TYPE"),
        ("fn main(){print([[1]]);}", "E_TYPE"),
    ] {
        assert_eq!(checked(source).unwrap_err().code, code);
    }
    checked("fn f(a:Tensor<f64,[1000,1000]> ){} fn main(){}").unwrap();
}

#[test]
fn exact_source_and_token_caps() {
    let at_cap = " ".repeat(1_048_576);
    frontend::lex(&at_cap).unwrap();
    assert_eq!(frontend::lex(&(at_cap + " ")).unwrap_err().code, "E_LIMIT");
    let tokens = "+".repeat(100_000);
    frontend::lex(&tokens).unwrap();
    assert_eq!(frontend::lex(&(tokens + "+")).unwrap_err().code, "E_LIMIT");
}

#[test]
fn exact_work_and_output_caps() {
    for extra in [0, 1] {
        let source = format!("fn main(){{let s=\"{}\";s;}}", "x".repeat(999_995 + extra));
        let result = engine::run(&checked(&source).unwrap());
        if extra == 0 {
            assert_eq!(result.unwrap(), "");
        } else {
            assert_eq!(result.unwrap_err().code, "E_LIMIT");
        }
        let source = format!("fn main(){{print(\"{}\");}}", "x".repeat(999_999 + extra));
        let result = engine::run(&checked(&source).unwrap());
        if extra == 0 {
            assert_eq!(result.unwrap().len(), 1_000_000);
        } else {
            assert_eq!(result.unwrap_err().code, "E_LIMIT");
        }
    }
}

#[test]
fn large_tensor_annotation_fails_in_semantic_validation() {
    let program = frontend::parse("fn f(a:Tensor<f64,[1000000,1000000]>){} fn main(){}").unwrap();
    assert_eq!(engine::check(&program).unwrap_err().code, "E_LIMIT");
}

#[test]
fn all_proposed_keyword_aliases_share_canonical_tokens() {
    let english = "fn let mut const if else loop while for in return break continue struct enum trait impl import pub true false print Tensor grad f32 f64 i32 i64 bool string";
    let mongolian = "функц хувьсагч өөрчлөгдөх тогтмол хэрэв үгүйбол давтах зуур бүр дотор буцаах зогсох үргэлжлэх бүтэц тоочилт шинж хэрэгжүүлэх импорт нийтийн үнэн худал хэвлэх Тензор уламжлал бодит32 бодит64 бүхэл32 бүхэл64 логик мөр";
    let kinds = |input| {
        frontend::lex(input)
            .unwrap()
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>()
    };
    assert_eq!(kinds(english), kinds(mongolian));
}

#[test]
fn strings_returned_from_a_call_keep_their_values() {
    let p = checked("fn literal()->string{return \"Монгол\";} fn echo(s:мөр)->мөр{буцаах s;} fn main(){let a=literal(); print(echo(a)); print(a);}").unwrap();
    assert_eq!(engine::run(&p).unwrap(), "Монгол\nМонгол\n");
}
