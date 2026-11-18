//! Typed, resolved expression sequences. No backend instruction encoding.
use super::{area, checker, error};
use crate::frontend::{Error, Span, Type};
use std::fmt::Write;

pub(super) struct Module<'a> {
    pub entry: usize,
    pub functions: Vec<Body<'a>>,
}
pub(super) struct Body<'a> {
    pub params: Vec<Type>,
    pub result: Type,
    pub span: Span,
    pub statements: Vec<Statement<'a>>,
}
pub(super) struct Statement<'a> {
    pub end: End,
    pub ops: Vec<Node<'a>>,
}
#[derive(Debug)]
pub(super) enum End {
    Bind(usize, Option<Type>),
    Print,
    Return(bool),
    Discard,
}
pub(super) struct Node<'a> {
    pub op: Op<'a>,
    pub ty: Type,
    pub span: Span,
}
#[derive(Debug)]
pub(super) enum Op<'a> {
    Int(i64),
    Float(u64),
    Bool(bool),
    Str(&'a str),
    Read(usize),
    Call(usize),
    Cast(Type),
    Neg,
    Binary(char),
    Tensor(usize, usize),
}

fn invalid(span: Span) -> Error {
    error(&span, "E_HIR", "invalid typed HIR")
}

impl Module<'_> {
    pub fn verify(&self) -> Result<(), Error> {
        let main = self
            .functions
            .get(self.entry)
            .ok_or_else(|| invalid(Span::default()))?;
        if !main.params.is_empty() || main.result != Type::Unit {
            return Err(invalid(main.span));
        }
        for f in &self.functions {
            let bad = || invalid(f.span);
            if f.span.start > f.span.end {
                return Err(bad());
            }
            for ty in f.params.iter().chain([&f.result]) {
                checker::validate_type(ty, &f.span).map_err(|_| bad())?;
            }
            let mut slots = f.params.clone();
            let mut stack = Vec::new();
            let mut returned = false;
            for stmt in &f.statements {
                if returned {
                    return Err(bad());
                }
                for node in &stmt.ops {
                    if node.span.start < f.span.start
                        || node.span.end > f.span.end
                        || node.span.start > node.span.end
                    {
                        return Err(invalid(node.span));
                    }
                    node.verify(self, &slots, &mut stack)?;
                }
                match (&stmt.end, stack.as_slice()) {
                    (End::Bind(id, ann), [ty])
                        if *id == slots.len() && ann.as_ref().is_none_or(|t| t == ty) =>
                    {
                        slots.push(ty.clone())
                    }
                    (End::Print | End::Discard, [_]) => {}
                    (End::Return(true), [ty]) if ty == &f.result => returned = true,
                    (End::Return(false), []) if f.result == Type::Unit => returned = true,
                    _ => return Err(bad()),
                }
                stack.clear();
            }
            if !returned && f.result != Type::Unit {
                return Err(bad());
            }
        }
        Ok(())
    }

    pub fn dump(&self) -> String {
        let mut out = format!("module entry f{}\n", self.entry);
        for (idx, f) in self.functions.iter().enumerate() {
            writeln!(out, "fn f{idx} {:?} -> {:?}", f.params, f.result).expect("String writer");
            for stmt in &f.statements {
                for node in &stmt.ops {
                    write!(out, "  ").expect("String writer");
                    match &node.op {
                        Op::Int(v) => write!(out, "int {v}"),
                        Op::Float(v) => write!(out, "f64bits {v:016x}"),
                        Op::Bool(v) => write!(out, "bool {v}"),
                        Op::Str(v) => write!(out, "str {v:?}"),
                        op => write!(out, "{op:?}"),
                    }
                    .expect("String writer");
                    writeln!(out, " : {:?}", node.ty).expect("String writer");
                }
                match stmt.end {
                    End::Bind(slot, _) => writeln!(out, "  bind {slot}"),
                    ref end => writeln!(out, "  {end:?}"),
                }
                .expect("String writer");
            }
        }
        out
    }
}

impl Node<'_> {
    fn verify(
        &self,
        module: &Module<'_>,
        slots: &[Type],
        stack: &mut Vec<Type>,
    ) -> Result<(), Error> {
        let bad = || invalid(self.span);
        let actual = match &self.op {
            Op::Int(_) => Type::I64,
            Op::Float(bits) if f64::from_bits(*bits).is_finite() => Type::F64,
            Op::Bool(_) => Type::Bool,
            Op::Str(_) => Type::String,
            Op::Read(slot) => slots.get(*slot).ok_or_else(bad)?.clone(),
            Op::Neg => {
                let ty = stack.pop().ok_or_else(bad)?;
                if !matches!(ty, Type::I64 | Type::F64) {
                    return Err(bad());
                }
                ty
            }
            Op::Cast(to) => {
                if !matches!(stack.pop(), Some(Type::I64 | Type::F64))
                    || !matches!(to, Type::I64 | Type::F64)
                {
                    return Err(bad());
                }
                to.clone()
            }
            Op::Binary(op) => {
                let right = stack.pop().ok_or_else(bad)?;
                let left = stack.pop().ok_or_else(bad)?;
                checker::binary_type(*op, &left, &right, &self.span).map_err(|_| bad())?
            }
            Op::Tensor(r, c) => {
                let count = area(*r, *c, &self.span).map_err(|_| bad())?;
                let start = stack.len().checked_sub(count).ok_or_else(bad)?;
                if stack[start..].iter().any(|t| *t != Type::F64) {
                    return Err(bad());
                }
                stack.truncate(start);
                Type::Tensor(*r, *c)
            }
            Op::Call(id) => {
                let callee = module.functions.get(*id).ok_or_else(bad)?;
                let start = stack
                    .len()
                    .checked_sub(callee.params.len())
                    .ok_or_else(bad)?;
                if stack[start..] != callee.params {
                    return Err(bad());
                }
                stack.truncate(start);
                callee.result.clone()
            }
            _ => return Err(bad()),
        };
        if actual != self.ty {
            return Err(bad());
        }
        stack.push(actual);
        Ok(())
    }
}
