//! Static validation and the bounded, safe-Rust reference interpreter.
use crate::frontend::{Error, Function, Span};
use std::collections::HashMap;

mod checker;
mod hir;
mod runtime;
mod tensor;
pub use checker::{check, validate};
pub use runtime::run;
use tensor::area;

const MAX_ELEMENTS: usize = 1_000_000;
const MAX_STEPS: usize = 1_000_000;
const MAX_DEPTH: usize = 128;
const MAX_OUTPUT: usize = 1_000_000;

fn error(span: &Span, code: &'static str, detail: impl Into<String>) -> Error {
    Error {
        span: Span {
            start: span.start,
            end: span.end,
        },
        code,
        detail: detail.into(),
    }
}
type Signatures<'a> = HashMap<&'a str, &'a Function>;

/// An immutable view of all functions after whole-program static validation.
/// The borrow keeps the checked syntax and signatures valid for the lifetime of the view.
///
/// ```compile_fail
/// use mn_core::{engine, frontend};
/// let mut program = frontend::parse("fn main(){}").unwrap();
/// let checked = engine::validate(&program).unwrap();
/// program.clear();
/// checked.run().unwrap();
/// ```
pub struct CheckedProgram<'a> {
    signatures: Signatures<'a>,
    hir: hir::Module<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::parse;
    fn execute(source: &str) -> Result<String, Error> {
        run(&parse(source)?)
    }
    #[test]
    fn matmul() {
        assert_eq!(execute("fn main() { let A = [[1.0,2.0],[3.0,4.0]]; let B = [[5.0,6.0],[7.0,8.0]]; print(A @ B); }").unwrap(),"[[19.0, 22.0], [43.0, 50.0]]\n");
    }
    #[test]
    fn bilingual() {
        assert_eq!(execute("функц main() { хувьсагч А = [[1.0,2.0],[3.0,4.0]]; хувьсагч Б = [[5.0,6.0],[7.0,8.0]]; хэвлэх(А @ Б); }").unwrap(),"[[19.0, 22.0], [43.0, 50.0]]\n");
    }
    #[test]
    fn mixed() {
        assert_eq!(
            execute("функц main() { let a = 10; хувьсагч b = 20; print(a+b); }").unwrap(),
            "30\n"
        );
    }
    #[test]
    fn functions() {
        assert_eq!(
            execute(
                "fn add(a:i64,b:i64)->i64 { return a+b; } функц main() { хэвлэх(add(12,30)); }"
            )
            .unwrap(),
            "42\n"
        );
    }
    #[test]
    fn invalid_shapes() {
        let e=execute("fn main(){ let A=[[1.0,2.0,3.0],[4.0,5.0,6.0]]; let B=[[1.0,2.0],[3.0,4.0]]; print(A@B); }").unwrap_err();
        assert_eq!(e.code, "E_SHAPE");
        assert!(e.detail.contains("3 and 2"));
    }
    #[test]
    fn errors() {
        for source in [
            "fn main(){print(x);}",
            "fn f(a:i64){} fn main(){f();}",
            "fn main(){let x=1; let x=2;}",
            "fn main(){let x=[[1.0],[2.0,3.0]];}",
            "fn f()->i64{} fn main(){}",
            "fn main(){let x:Tensor<f64,[3,2]>=[[1.0,2.0]];}",
        ] {
            assert!(execute(source).is_err(), "{source}");
        }
    }
    #[test]
    fn arithmetic() {
        assert_eq!(
            execute("fn main(){print(2+3*4); print((2+3)*4); print(7/2);}").unwrap(),
            "14\n20\n3\n"
        );
    }
    #[test]
    fn checked_integers() {
        assert_eq!(
            execute("fn main(){print(1/0);}").unwrap_err().code,
            "E_DIV_ZERO"
        );
        assert_eq!(
            execute("fn main(){print(9223372036854775807+1);}")
                .unwrap_err()
                .code,
            "E_OVERFLOW"
        );
    }
    #[test]
    fn validate_unused_functions() {
        assert!(execute("fn bad(){print(missing);} fn main(){print(1);}").is_err());
    }
    #[test]
    fn recursion_is_bounded() {
        assert_eq!(
            execute("fn f()->i64{return f();} fn main(){print(f());}")
                .unwrap_err()
                .code,
            "E_LIMIT"
        );
    }
    #[test]
    fn differential_small_matrices() {
        for a in 0..10 {
            for b in 0..10 {
                let source = format!("fn main(){{print([[{a}.0,2.0]] @ [[{b}.0],[3.0]]);}}");
                assert_eq!(
                    execute(&source).unwrap(),
                    format!("[[{:.1}]]\n", f64::from(a * b + 6))
                );
            }
        }
    }
    #[test]
    fn integer_arithmetic_matches_checked_reference() {
        let mut state = 0x12345678_u64;
        for _ in 0..256 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = (state % 2001) as i64 - 1000;
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let b = (state % 2001) as i64 - 1000;
            for (op, expected) in [
                ('+', a.checked_add(b)),
                ('-', a.checked_sub(b)),
                ('*', a.checked_mul(b)),
                ('/', a.checked_div(b)),
            ] {
                let actual = execute(&format!("fn main(){{print(({a}){op}({b}));}}"));
                match expected {
                    Some(value) => assert_eq!(actual.unwrap(), format!("{value}\n")),
                    None => assert!(actual.is_err()),
                }
            }
        }
    }
    #[test]
    fn tensor_identity_and_shape_properties() {
        for n in 1..=4 {
            let rows: Vec<_> = (0..n)
                .map(|r| {
                    let cells: Vec<_> = (0..n).map(|c| format!("{}.0", r * n + c)).collect();
                    format!("[{}]", cells.join(","))
                })
                .collect();
            let identity: Vec<_> = (0..n)
                .map(|r| {
                    let cells: Vec<_> =
                        (0..n).map(|c| if r == c { "1.0" } else { "0.0" }).collect();
                    format!("[{}]", cells.join(","))
                })
                .collect();
            let source = format!(
                "fn main(){{let a=[{}]; let i=[{}]; print(a); print(a@i); print(i@a);}}",
                rows.join(","),
                identity.join(",")
            );
            let output = execute(&source).unwrap();
            let lines: Vec<_> = output.lines().collect();
            assert_eq!(lines.len(), 3);
            assert_eq!(lines[0], lines[1]);
            assert_eq!(lines[0], lines[2]);
        }
    }
    #[test]
    fn float_division_is_ieee() {
        assert_eq!(
            execute("fn main(){print(1.0/0.0); print(0.0/0.0);}").unwrap(),
            "inf\nNaN\n"
        );
    }
    #[test]
    fn allocation_size_overflow_is_rejected() {
        assert!(area(usize::MAX, 2, &Span { start: 0, end: 0 }).is_err());
    }
}

impl CheckedProgram<'_> {
    /// Backend-independent text for semantic conformance tests.
    pub fn hir_dump(&self) -> String {
        self.hir.dump()
    }
}
