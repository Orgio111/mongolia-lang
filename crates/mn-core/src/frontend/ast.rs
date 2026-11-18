//! Source locations, checked language types, and owned syntax nodes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub span: Span,
    pub code: &'static str,
    pub detail: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    I64,
    F64,
    Bool,
    String,
    Unit,
    Tensor(usize, usize),
}
pub type Program = Vec<Function>;
#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub result: Type,
    pub body: Vec<Stmt>,
    pub span: Span,
}
#[derive(Clone, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
#[derive(Clone, Debug)]
pub enum ExprKind {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Var(String),
    Call(String, Vec<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Tensor(Vec<Vec<Expr>>),
}
#[derive(Clone, Debug)]
pub enum Stmt {
    Let(String, Option<Type>, Expr),
    Return(Option<Expr>),
    Print(Expr),
    Expr(Expr),
}
