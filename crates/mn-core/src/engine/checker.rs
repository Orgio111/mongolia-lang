use super::hir::{Body, End, Module, Node, Op, Statement};
use super::{area, error, tensor, CheckedProgram, Signatures, MAX_DEPTH};
use crate::frontend::{Error, Expr, ExprKind, Program, Span, Stmt, Type};
use std::collections::HashMap;
/// Check a program without retaining its validated view.
pub fn check(program: &Program) -> Result<(), Error> {
    validate(program).map(|_| ())
}

/// Resolve signatures and validate every function exactly once for this view.
pub fn validate(program: &Program) -> Result<CheckedProgram<'_>, Error> {
    let (signatures, hir) = lower(program)?;
    hir.verify()?;
    Ok(CheckedProgram { signatures, hir })
}
fn lower(program: &Program) -> Result<(Signatures<'_>, Module<'_>), Error> {
    let mut signatures = HashMap::new();
    for (i, function) in program.iter().enumerate() {
        if signatures
            .insert(function.name.as_str(), (i, function))
            .is_some()
        {
            return Err(error(
                &function.span,
                "E_DUPLICATE",
                format!("duplicate function {}", function.name),
            ));
        }
    }
    let empty = Span { start: 0, end: 0 };
    let &(entry, main) = signatures
        .get("main")
        .ok_or_else(|| error(&empty, "E_MAIN", "missing main function"))?;
    if !main.params.is_empty() || main.result != Type::Unit {
        return Err(error(
            &main.span,
            "E_MAIN",
            "main must have no parameters and return unit",
        ));
    }
    let mut functions = Vec::with_capacity(program.len());
    for function in program {
        let mut statements = Vec::with_capacity(function.body.len());
        let mut env = HashMap::new();
        for (name, ty) in &function.params {
            validate_type(ty, &function.span)?;
            let slot = env.len();
            if env.insert(name.as_str(), (slot, ty.clone())).is_some() {
                return Err(error(
                    &function.span,
                    "E_DUPLICATE",
                    format!("duplicate parameter {name}"),
                ));
            }
        }
        validate_type(&function.result, &function.span)?;
        let mut returned = false;
        for statement in &function.body {
            if returned {
                return Err(error(
                    &function.span,
                    "E_UNREACHABLE",
                    "statement after return",
                ));
            }
            let mut ops = Vec::new();
            let end = match statement {
                Stmt::Let(name, annotation, expr) => {
                    let ty = infer(expr, &env, &signatures, &mut ops, 0)?;
                    if let Some(expected) = annotation {
                        validate_type(expected, &expr.span)?;
                        require_type(&ty, expected, &expr.span)?;
                    }
                    let slot = env.len();
                    if env.insert(name.as_str(), (slot, ty)).is_some() {
                        return Err(error(
                            &expr.span,
                            "E_DUPLICATE",
                            format!("duplicate binding {name}"),
                        ));
                    }
                    End::Bind(slot, annotation.clone())
                }
                Stmt::Return(expr) => {
                    let ty = if let Some(expr) = expr {
                        infer(expr, &env, &signatures, &mut ops, 0)?
                    } else {
                        Type::Unit
                    };
                    let origin = expr.as_ref().map_or(&function.span, |e| &e.span);
                    require_type(&ty, &function.result, origin)?;
                    returned = true;
                    End::Return(expr.is_some())
                }
                Stmt::Print(expr) | Stmt::Expr(expr) => {
                    infer(expr, &env, &signatures, &mut ops, 0)?;
                    if matches!(statement, Stmt::Print(_)) {
                        End::Print
                    } else {
                        End::Discard
                    }
                }
            };
            statements.push(Statement { ops, end });
        }
        if !returned && function.result != Type::Unit {
            return Err(error(
                &function.span,
                "E_RETURN",
                "non-unit function must return a value",
            ));
        }
        functions.push(Body {
            params: function.params.iter().map(|(_, t)| t.clone()).collect(),
            result: function.result.clone(),
            span: function.span,
            statements,
        });
    }
    Ok((signatures, Module { entry, functions }))
}
pub(super) fn validate_type(ty: &Type, span: &Span) -> Result<(), Error> {
    if let Type::Tensor(r, c) = ty {
        area(*r, *c, span)?;
    }
    Ok(())
}
fn require_type(actual: &Type, expected: &Type, span: &Span) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(error(
            span,
            "E_TYPE",
            format!("expected {expected:?}, found {actual:?}"),
        ))
    }
}
pub(super) fn binary_type(op: char, a: &Type, b: &Type, span: &Span) -> Result<Type, Error> {
    if op == '@' {
        return match (a, b) {
            (Type::Tensor(m, k), Type::Tensor(j, n)) => {
                tensor::product_area((*m, *k), (*j, *n), span)?;
                Ok(Type::Tensor(*m, *n))
            }
            _ => Err(error(span, "E_TYPE", "@ requires two rank-2 tensors")),
        };
    }
    if !matches!(op, '+' | '-' | '*' | '/') {
        return Err(error(span, "E_OPERATOR", "unsupported binary operator"));
    }
    require_type(b, a, span)?;
    if matches!(a, Type::I64 | Type::F64 | Type::Tensor(_, _)) {
        Ok(a.clone())
    } else {
        Err(error(
            span,
            "E_TYPE",
            "arithmetic requires numeric operands",
        ))
    }
}

fn infer<'a>(
    expr: &'a Expr,
    env: &HashMap<&str, (usize, Type)>,
    signatures: &Signatures<'_>,
    ops: &mut Vec<Node<'a>>,
    depth: usize,
) -> Result<Type, Error> {
    if depth > MAX_DEPTH {
        return Err(error(&expr.span, "E_LIMIT", "expression depth exceeded"));
    }
    let mut nested = |e| infer(e, env, signatures, ops, depth + 1);
    let (op, ty) = match &expr.kind {
        ExprKind::Int(v) => (Op::Int(*v), Type::I64),
        ExprKind::Float(v) if v.is_finite() => (Op::Float(v.to_bits()), Type::F64),
        ExprKind::Float(_) => {
            return Err(error(&expr.span, "E_NUMBER", "float literal out of range"))
        }
        ExprKind::Bool(v) => (Op::Bool(*v), Type::Bool),
        ExprKind::Str(v) => (Op::Str(v), Type::String),
        ExprKind::Var(name) => {
            let (slot, ty) = env.get(name.as_str()).ok_or_else(|| {
                error(&expr.span, "E_NAME", format!("undefined identifier {name}"))
            })?;
            (Op::Read(*slot), ty.clone())
        }
        ExprKind::Cast(to, x) => {
            let ty = nested(x)?;
            if !matches!(to, Type::I64 | Type::F64) || !matches!(ty, Type::I64 | Type::F64) {
                return Err(error(
                    &expr.span,
                    "E_CAST",
                    format!("cannot convert {ty:?} to {to:?}; use i64 or f64"),
                ));
            }
            (Op::Cast(to.clone()), to.clone())
        }
        ExprKind::Neg(x) => {
            let ty = nested(x)?;
            if !matches!(ty, Type::I64 | Type::F64) {
                return Err(error(
                    &expr.span,
                    "E_TYPE",
                    "negation requires a scalar number",
                ));
            }
            (Op::Neg, ty)
        }
        ExprKind::Binary(op, a, b) => {
            let a = nested(a)?;
            let b = nested(b)?;
            (Op::Binary(*op), binary_type(*op, &a, &b, &expr.span)?)
        }
        ExprKind::Call(name, args) => {
            let &(id, f) = signatures
                .get(name.as_str())
                .ok_or_else(|| error(&expr.span, "E_NAME", format!("undefined function {name}")))?;
            if args.len() != f.params.len() {
                return Err(error(
                    &expr.span,
                    "E_ARGUMENTS",
                    format!(
                        "{} expects {} arguments, found {}",
                        name,
                        f.params.len(),
                        args.len()
                    ),
                ));
            }
            for (a, (_, expected)) in args.iter().zip(&f.params) {
                require_type(&nested(a)?, expected, &a.span)?;
            }
            (Op::Call(id), f.result.clone())
        }
        ExprKind::Tensor(rows) => {
            let cols = rows.first().map_or(0, Vec::len);
            if cols == 0 {
                return Err(error(
                    &expr.span,
                    "E_SHAPE",
                    "tensor literals require nonempty rows",
                ));
            }
            area(rows.len(), cols, &expr.span)?;
            for row in rows {
                if row.len() != cols {
                    return Err(error(&expr.span, "E_SHAPE", "ragged tensor literal"));
                }
                for e in row {
                    require_type(&nested(e)?, &Type::F64, &e.span)?
                }
            }
            (Op::Tensor(rows.len(), cols), Type::Tensor(rows.len(), cols))
        }
    };
    ops.push(Node {
        op,
        ty: ty.clone(),
        span: expr.span,
    });
    Ok(ty)
}

#[cfg(test)]
mod hir_tests {
    use super::*;
    use crate::frontend::parse;

    #[test]
    fn verifier_rejects_invalid_hir() {
        let p=parse("fn main(){let x=[[2.0]];print(f(x));} fn f(a:Tensor<f64,[1,1]>)->Tensor<f64,[1,1]>{return a@a;}").unwrap();
        for fault in 0..11 {
            let (_, mut m) = lower(&p).unwrap();
            let expr = &mut m.functions[0].statements[1].ops;
            match fault {
                0 => expr[1].op = Op::Call(9),
                1 => expr[0].op = Op::Read(2),
                2 => expr.clear(),
                3 => expr[1].ty = Type::Bool,
                4 => expr[0].span.end = usize::MAX,
                5 => m.entry = 1,
                6 => m.functions[1].statements.clear(),
                7 => m.functions[0].statements.swap(0, 1),
                8 => m.functions[0].statements[0].ops[1].op = Op::Tensor(usize::MAX, 2),
                9 => m.functions[0].statements[0].end = End::Bind(0, Some(Type::I64)),
                _ => m.functions[1].statements.push(Statement {
                    ops: vec![],
                    end: End::Return(false),
                }),
            }
            assert_eq!(m.verify().unwrap_err().code, "E_HIR");
        }
    }
    #[test]
    fn original_spans_survive_nfc_and_lowering() {
        let source = "функц main(){let café=2;хэвлэх(бодит64(cafe\u{301}));}";
        let ast = parse(source).unwrap();
        let (_, module) = lower(&ast).unwrap();
        let ops = &module.functions[0].statements[1].ops;
        let original: Vec<_> = ops
            .iter()
            .map(|node| &source[node.span.start..node.span.end])
            .collect();
        assert_eq!(original, ["cafe\u{301}", "бодит64(cafe\u{301})"]);
        assert_eq!(module.dump(), "module entry f0\nfn f0 [] -> Unit\n  int 2 : I64\n  bind 0\n  Read(0) : I64\n  Cast(F64) : F64\n  Print\n");
    }
    #[test]
    #[ignore = "release timing probe"]
    fn phase11_stage_times() {
        use std::{hint::black_box, time::Instant};
        let p = parse(include_str!("../../../../benchmarks/data/frontend.mn")).unwrap();
        for _ in 0..60 {
            let t = Instant::now();
            let (_, m) = lower(black_box(&p)).unwrap();
            let lower_ns = t.elapsed().as_nanos();
            let t = Instant::now();
            black_box(&m).verify().unwrap();
            println!("STAGE {lower_ns} {}", t.elapsed().as_nanos());
        }
    }
}
