use super::{area, error, tensor, CheckedProgram, Signatures, MAX_DEPTH};
use crate::frontend::{Error, Expr, ExprKind, Program, Span, Stmt, Type};
use std::collections::HashMap;
/// Check a program without retaining its validated view.
pub fn check(program: &Program) -> Result<(), Error> {
    validate(program).map(|_| ())
}

/// Resolve signatures and validate every function exactly once for this view.
pub fn validate(program: &Program) -> Result<CheckedProgram<'_>, Error> {
    let mut signatures = HashMap::new();
    for function in program {
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
    for function in program {
        let mut env = HashMap::new();
        for (name, ty) in &function.params {
            validate_type(ty, &function.span)?;
            if env.insert(name.clone(), ty.clone()).is_some() {
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
            match statement {
                Stmt::Let(name, annotation, expr) => {
                    let ty = infer(expr, &env, &signatures, 0)?;
                    if let Some(expected) = annotation {
                        validate_type(expected, &expr.span)?;
                        require_type(&ty, expected, &expr.span)?;
                    }
                    if env.insert(name.clone(), ty).is_some() {
                        return Err(error(
                            &expr.span,
                            "E_DUPLICATE",
                            format!("duplicate binding {name}"),
                        ));
                    }
                }
                Stmt::Return(expr) => {
                    let ty = if let Some(expr) = expr {
                        infer(expr, &env, &signatures, 0)?
                    } else {
                        Type::Unit
                    };
                    let origin = expr.as_ref().map_or(&function.span, |e| &e.span);
                    require_type(&ty, &function.result, origin)?;
                    returned = true;
                }
                Stmt::Print(expr) | Stmt::Expr(expr) => {
                    infer(expr, &env, &signatures, 0)?;
                }
            }
        }
        if !returned && function.result != Type::Unit {
            return Err(error(
                &function.span,
                "E_RETURN",
                "non-unit function must return a value",
            ));
        }
    }
    Ok(CheckedProgram { signatures })
}
fn validate_type(ty: &Type, span: &Span) -> Result<(), Error> {
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
fn infer(
    expr: &Expr,
    env: &HashMap<String, Type>,
    signatures: &Signatures<'_>,
    depth: usize,
) -> Result<Type, Error> {
    if depth > MAX_DEPTH {
        return Err(error(&expr.span, "E_LIMIT", "expression depth exceeded"));
    }
    let nested = |e: &Expr| infer(e, env, signatures, depth + 1);
    match &expr.kind {
        ExprKind::Int(_) => Ok(Type::I64),
        ExprKind::Float(value) if !value.is_finite() => {
            Err(error(&expr.span, "E_NUMBER", "float literal out of range"))
        }
        ExprKind::Float(_) => Ok(Type::F64),
        ExprKind::Bool(_) => Ok(Type::Bool),
        ExprKind::Str(_) => Ok(Type::String),
        ExprKind::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| error(&expr.span, "E_NAME", format!("undefined identifier {name}"))),
        ExprKind::Neg(value) => {
            let ty = nested(value)?;
            if ty == Type::I64 || ty == Type::F64 {
                Ok(ty)
            } else {
                Err(error(
                    &expr.span,
                    "E_TYPE",
                    "negation requires a scalar number",
                ))
            }
        }
        ExprKind::Call(name, args) => {
            let function = signatures
                .get(name.as_str())
                .ok_or_else(|| error(&expr.span, "E_NAME", format!("undefined function {name}")))?;
            if args.len() != function.params.len() {
                return Err(error(
                    &expr.span,
                    "E_ARGUMENTS",
                    format!(
                        "{} expects {} arguments, found {}",
                        name,
                        function.params.len(),
                        args.len()
                    ),
                ));
            }
            for (arg, (_, expected)) in args.iter().zip(&function.params) {
                require_type(&nested(arg)?, expected, &arg.span)?;
            }
            Ok(function.result.clone())
        }
        ExprKind::Tensor(rows) => {
            let cols = rows.first().map_or(0, Vec::len);
            if rows.is_empty() || cols == 0 {
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
                for cell in row {
                    require_type(&nested(cell)?, &Type::F64, &cell.span)?;
                }
            }
            Ok(Type::Tensor(rows.len(), cols))
        }
        ExprKind::Binary(op, left, right) => {
            let a = nested(left)?;
            let b = nested(right)?;
            if *op == '@' {
                return match (&a, &b) {
                    (Type::Tensor(m, k), Type::Tensor(j, n)) => {
                        tensor::product_area((*m, *k), (*j, *n), &expr.span)?;
                        Ok(Type::Tensor(*m, *n))
                    }
                    _ => Err(error(&expr.span, "E_TYPE", "@ requires two rank-2 tensors")),
                };
            }
            if !matches!(op, '+' | '-' | '*' | '/') {
                return Err(error(
                    &expr.span,
                    "E_OPERATOR",
                    "unsupported binary operator",
                ));
            }
            require_type(&b, &a, &expr.span)?;
            if matches!(a, Type::I64 | Type::F64 | Type::Tensor(_, _)) {
                Ok(a)
            } else {
                Err(error(
                    &expr.span,
                    "E_TYPE",
                    "arithmetic requires numeric operands",
                ))
            }
        }
    }
}
