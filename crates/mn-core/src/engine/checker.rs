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
    let mut ids = HashMap::new();
    for (i, function) in program.iter().enumerate() {
        ids.insert(function.name.as_str(), i);
        if signatures
            .insert(function.name.as_str(), function)
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
    let main = signatures
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
                    let ty = infer(expr, &env, &signatures, &ids, &mut ops, 0)?;
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
                        infer(expr, &env, &signatures, &ids, &mut ops, 0)?
                    } else {
                        Type::Unit
                    };
                    let origin = expr.as_ref().map_or(&function.span, |e| &e.span);
                    require_type(&ty, &function.result, origin)?;
                    returned = true;
                    End::Return(expr.is_some())
                }
                Stmt::Print(expr) | Stmt::Expr(expr) => {
                    infer(expr, &env, &signatures, &ids, &mut ops, 0)?;
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
    Ok((
        signatures,
        Module {
            entry: ids["main"],
            functions,
        },
    ))
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
    ids: &HashMap<&str, usize>,
    ops: &mut Vec<Node<'a>>,
    depth: usize,
) -> Result<Type, Error> {
    if depth > MAX_DEPTH {
        return Err(error(&expr.span, "E_LIMIT", "expression depth exceeded"));
    }
    let mut nested = |e| infer(e, env, signatures, ids, ops, depth + 1);
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
                    "numeric operand required for conversion",
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
            let f = signatures
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
            (Op::Call(ids[name.as_str()]), f.result.clone())
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
