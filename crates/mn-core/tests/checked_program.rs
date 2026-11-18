use mn_core::{engine, frontend};

#[test]
fn checked_handle_reuses_code_and_resets_execution_state() {
    let program =
        frontend::parse("fn main(){print(double(21));} fn double(x:i64)->i64{return x+x;}")
            .unwrap();
    let validated = engine::validate(&program).unwrap();
    assert_eq!(validated.run().unwrap(), "42\n");
    assert_eq!(validated.run().unwrap(), "42\n");
    assert_eq!(engine::run(&program).unwrap(), "42\n");
}

#[test]
fn checker_rejects_invalid_annotation_in_an_externally_assembled_ast() {
    let mut program = frontend::parse("fn main(){} fn f(x:i64){}").unwrap();
    program[1].params[0].1 = frontend::Type::Tensor(0, 2);
    assert_eq!(engine::check(&program).unwrap_err().code, "E_SHAPE");
    assert_eq!(engine::run(&program).unwrap_err().code, "E_SHAPE");
}

#[test]
fn tensor_print_budget_includes_delimiters_and_final_newline() {
    // "[[1.0, 2.0]]\n" is 13 bytes. The first print adds its own newline.
    for extra in [0, 1] {
        let text = "x".repeat(1_000_000 - 14 + extra);
        let source = format!("fn main(){{print(\"{text}\");print([[1.0,2.0]]);}}");
        let result = engine::run(&frontend::parse(&source).unwrap());
        if extra == 0 {
            let output = result.unwrap();
            assert_eq!(output.len(), 1_000_000);
            assert!(output.ends_with("\n[[1.0, 2.0]]\n"));
        } else {
            assert_eq!(result.unwrap_err().code, "E_LIMIT");
        }
    }
}
