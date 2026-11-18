use super::{
    area, error, tensor, tensor::allocate, validate, CheckedProgram, Signatures, MAX_DEPTH,
    MAX_OUTPUT, MAX_STEPS,
};
use crate::frontend::{Error, Expr, ExprKind, Program, Span, Stmt};
use std::{borrow::Cow, collections::HashMap};
#[derive(Clone, Debug)]
enum Value<'ast> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Cow<'ast, str>),
    Unit,
    Tensor {
        rows: usize,
        cols: usize,
        data: Vec<f64>,
    },
}
/// Append only the bytes that fit the documented output budget. The machine and
/// its partial output are dropped if a print or any later instruction fails.
fn append_output(out: &mut String, text: &str, span: &Span) -> Result<(), Error> {
    if text.len() > MAX_OUTPUT - out.len() {
        return Err(error(span, "E_LIMIT", "output budget exceeded"));
    }
    out.try_reserve(text.len())
        .map_err(|_| error(span, "E_ALLOCATION", "output allocation failed"))?;
    out.push_str(text);
    Ok(())
}

impl Value<'_> {
    fn print_line(&self, out: &mut String, span: &Span) -> Result<(), Error> {
        let mut append = |s: &str| append_output(out, s, span);
        match self {
            Self::Int(v) => append(&v.to_string())?,
            Self::Float(v) => append(&format!("{v:?}"))?,
            Self::Bool(v) => append(if *v { "true" } else { "false" })?,
            Self::Str(v) => append(v)?,
            Self::Unit => append("()")?,
            Self::Tensor { cols, data, .. } => {
                append("[")?;
                for (index, row) in data.chunks_exact(*cols).enumerate() {
                    if index != 0 {
                        append(", ")?;
                    }
                    append("[")?;
                    for (index, value) in row.iter().enumerate() {
                        if index != 0 {
                            append(", ")?;
                        }
                        append(&format!("{value:?}"))?;
                    }
                    append("]")?;
                }
                append("]")?;
            }
        }
        append("\n")
    }
}

struct Machine<'table, 'ast> {
    signatures: &'table Signatures<'ast>,
    remaining: usize,
    output: String,
}

/// Convenience entry point for callers holding an unchecked AST.
pub fn run(program: &Program) -> Result<String, Error> {
    validate(program)?.run()
}

impl CheckedProgram<'_> {
    /// Execute with fresh call frames, work budget and buffered output on each invocation.
    pub fn run(&self) -> Result<String, Error> {
        let mut machine = Machine {
            signatures: &self.signatures,
            remaining: MAX_STEPS,
            output: String::new(),
        };
        machine.call("main", Vec::new(), 0, &Span { start: 0, end: 0 })?;
        Ok(machine.output)
    }
}

impl<'ast> Machine<'_, 'ast> {
    fn tick(&mut self, amount: usize, span: &Span) -> Result<(), Error> {
        self.remaining = self
            .remaining
            .checked_sub(amount)
            .ok_or_else(|| error(span, "E_LIMIT", "execution step budget exceeded"))?;
        Ok(())
    }
    fn call(
        &mut self,
        name: &str,
        args: Vec<Value<'ast>>,
        depth: usize,
        span: &Span,
    ) -> Result<Value<'ast>, Error> {
        if depth >= MAX_DEPTH {
            return Err(error(span, "E_LIMIT", "call depth exceeded"));
        }
        self.tick(1, span)?;
        let function = *self
            .signatures
            .get(name)
            .ok_or_else(|| error(span, "E_NAME", "function unavailable"))?;
        let mut env = HashMap::new();
        env.try_reserve(args.len())
            .map_err(|_| error(span, "E_ALLOCATION", "frame allocation failed"))?;
        for ((name, _), value) in function.params.iter().zip(args) {
            env.insert(name.as_str(), value);
        }
        for statement in &function.body {
            self.tick(1, &function.span)?;
            match statement {
                Stmt::Let(name, _, expr) => {
                    let value = self.eval(expr, &env, depth + 1)?;
                    env.try_reserve(1).map_err(|_| {
                        error(&expr.span, "E_ALLOCATION", "frame allocation failed")
                    })?;
                    env.insert(name.as_str(), value);
                }
                Stmt::Return(expr) => {
                    return if let Some(expr) = expr {
                        self.eval(expr, &env, depth + 1)
                    } else {
                        Ok(Value::Unit)
                    }
                }
                Stmt::Expr(expr) => {
                    self.eval(expr, &env, depth + 1)?;
                }
                Stmt::Print(expr) => {
                    let value = self.eval(expr, &env, depth + 1)?;
                    value.print_line(&mut self.output, &expr.span)?;
                }
            }
        }
        Ok(Value::Unit)
    }
    fn eval(
        &mut self,
        expr: &'ast Expr,
        env: &HashMap<&'ast str, Value<'ast>>,
        depth: usize,
    ) -> Result<Value<'ast>, Error> {
        if depth > MAX_DEPTH {
            return Err(error(&expr.span, "E_LIMIT", "evaluation depth exceeded"));
        }
        self.tick(1, &expr.span)?;
        match &expr.kind {
            ExprKind::Int(v) => Ok(Value::Int(*v)),
            ExprKind::Float(v) => Ok(Value::Float(*v)),
            ExprKind::Bool(v) => Ok(Value::Bool(*v)),
            ExprKind::Str(v) => Ok(Value::Str(Cow::Borrowed(v.as_str()))),
            ExprKind::Var(name) => {
                let value = env
                    .get(name.as_str())
                    .ok_or_else(|| error(&expr.span, "E_NAME", "binding unavailable"))?;
                match value {
                    Value::Tensor { rows, cols, data } => {
                        self.tick(data.len(), &expr.span)?;
                        let mut copy = allocate(data.len(), &expr.span)?;
                        copy.copy_from_slice(data);
                        Ok(Value::Tensor {
                            rows: *rows,
                            cols: *cols,
                            data: copy,
                        })
                    }
                    Value::Str(text) => {
                        self.tick(text.len(), &expr.span)?;
                        let mut copy = String::new();
                        copy.try_reserve(text.len()).map_err(|_| {
                            error(&expr.span, "E_ALLOCATION", "string allocation failed")
                        })?;
                        copy.push_str(text);
                        Ok(Value::Str(Cow::Owned(copy)))
                    }
                    other => Ok(other.clone()),
                }
            }
            ExprKind::Cast(to, x) => {
                use crate::frontend::Type;
                match (to, self.eval(x, env, depth + 1)?) {
                    (Type::I64, v @ Value::Int(_)) | (Type::F64, v @ Value::Float(_)) => Ok(v),
                    (Type::F64, Value::Int(v)) => Ok(Value::Float(v as f64)),
                    (Type::I64, Value::Float(v))
                        if (-9223372036854775808.0..9223372036854775808.0).contains(&v) =>
                    {
                        Ok(Value::Int(v as i64))
                    }
                    (Type::I64, Value::Float(_)) => Err(error(
                        &expr.span,
                        "E_CAST_RANGE",
                        "value outside i64 conversion range",
                    )),
                    _ => Err(error(
                        &expr.span,
                        "E_CAST",
                        "numeric operand required for conversion",
                    )),
                }
            }
            ExprKind::Neg(value) => match self.eval(value, env, depth + 1)? {
                Value::Int(v) => v
                    .checked_neg()
                    .map(Value::Int)
                    .ok_or_else(|| error(&expr.span, "E_OVERFLOW", "integer negation overflow")),
                Value::Float(v) => Ok(Value::Float(-v)),
                _ => Err(error(&expr.span, "E_TYPE", "invalid negation")),
            },
            ExprKind::Call(name, args) => {
                let mut values = Vec::new();
                values
                    .try_reserve_exact(args.len())
                    .map_err(|_| error(&expr.span, "E_ALLOCATION", "argument allocation failed"))?;
                for arg in args {
                    values.push(self.eval(arg, env, depth + 1)?);
                }
                self.call(name, values, depth + 1, &expr.span)
            }
            ExprKind::Tensor(rows) => {
                let cols = rows.first().map_or(0, Vec::len);
                let size = area(rows.len(), cols, &expr.span)?;
                let mut data = allocate(size, &expr.span)?;
                for (r, row) in rows.iter().enumerate() {
                    for (c, cell) in row.iter().enumerate() {
                        if let Value::Float(value) = self.eval(cell, env, depth + 1)? {
                            data[r * cols + c] = value;
                        } else {
                            return Err(error(&cell.span, "E_TYPE", "tensor cell is not f64"));
                        }
                    }
                }
                Ok(Value::Tensor {
                    rows: rows.len(),
                    cols,
                    data,
                })
            }
            ExprKind::Binary(op, left, right) => {
                let a = self.eval(left, env, depth + 1)?;
                let b = self.eval(right, env, depth + 1)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        if *op == '/' && y == 0 {
                            return Err(error(
                                &expr.span,
                                "E_DIV_ZERO",
                                "integer division by zero",
                            ));
                        }
                        let value = match op {
                            '+' => x.checked_add(y),
                            '-' => x.checked_sub(y),
                            '*' => x.checked_mul(y),
                            '/' => x.checked_div(y),
                            _ => None,
                        };
                        value.map(Value::Int).ok_or_else(|| {
                            error(&expr.span, "E_OVERFLOW", "integer arithmetic overflow")
                        })
                    }
                    (Value::Float(x), Value::Float(y)) => Ok(Value::Float(float_op(*op, x, y))),
                    (
                        Value::Tensor {
                            rows: m,
                            cols: k,
                            data: a,
                        },
                        Value::Tensor {
                            rows: j,
                            cols: n,
                            data: b,
                        },
                    ) => {
                        if *op == '@' {
                            let size = tensor::product_area((m, k), (j, n), &expr.span)?;
                            let cost = size.checked_mul(k).ok_or_else(|| {
                                error(&expr.span, "E_LIMIT", "matrix work overflow")
                            })?;
                            self.tick(cost, &expr.span)?;
                            let mut data = allocate(size, &expr.span)?;
                            // Update contiguous rows. Each cell still accumulates
                            // in increasing contraction-index order, starting at +0.
                            for (left_row, output) in
                                a.chunks_exact(k).zip(data.chunks_exact_mut(n))
                            {
                                for (&left, right_row) in left_row.iter().zip(b.chunks_exact(n)) {
                                    for (cell, &right) in output.iter_mut().zip(right_row) {
                                        *cell += left * right;
                                    }
                                }
                            }
                            Ok(Value::Tensor {
                                rows: m,
                                cols: n,
                                data,
                            })
                        } else {
                            if m != j || k != n {
                                return Err(error(
                                    &expr.span,
                                    "E_SHAPE",
                                    "elementwise shapes differ",
                                ));
                            }
                            self.tick(a.len(), &expr.span)?;
                            let mut data = allocate(a.len(), &expr.span)?;
                            for ((out, x), y) in data.iter_mut().zip(a).zip(b) {
                                *out = float_op(*op, x, y);
                            }
                            Ok(Value::Tensor {
                                rows: m,
                                cols: k,
                                data,
                            })
                        }
                    }
                    _ => Err(error(&expr.span, "E_TYPE", "invalid arithmetic operands")),
                }
            }
        }
    }
}
fn float_op(op: char, x: f64, y: f64) -> f64 {
    match op {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        _ => f64::NAN,
    }
}
