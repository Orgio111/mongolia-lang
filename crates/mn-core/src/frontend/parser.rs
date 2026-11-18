use super::lexer::reserved;
use super::{
    error, lex, Error, Expr, ExprKind, Function, Program, Span, Stmt, Token, TokenKind, Type,
};
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    depth: usize,
    nodes: usize,
}
impl Parser {
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn bump(&mut self) -> Token {
        let token = &mut self.tokens[self.pos];
        let t = Token {
            kind: std::mem::replace(&mut token.kind, TokenKind::Eof),
            span: token.span,
        };
        if t.kind != TokenKind::Eof {
            self.pos += 1;
        }
        t
    }
    fn fail(&self, message: &str) -> Error {
        error(self.current().span, "E_PARSE", message)
    }
    fn symbol(&mut self, c: char) -> bool {
        if self.current().kind == TokenKind::Symbol(c) {
            self.bump();
            true
        } else {
            false
        }
    }
    fn expect(&mut self, c: char) -> Result<(), Error> {
        if self.symbol(c) {
            Ok(())
        } else {
            Err(self.fail(&format!("expected '{c}'")))
        }
    }
    fn word(&mut self, word: &str) -> bool {
        if matches!(&self.current().kind, TokenKind::Word(s) if s == word) {
            self.bump();
            true
        } else {
            false
        }
    }
    fn ident(&mut self) -> Result<String, Error> {
        let t = self.bump();
        if let TokenKind::Word(s) = t.kind {
            if !reserved(&s) {
                return Ok(s);
            }
        }
        Err(error(t.span, "E_PARSE", "expected identifier"))
    }
    fn ty(&mut self) -> Result<Type, Error> {
        let token = self.bump();
        match token.kind {
            TokenKind::Word(s) => match s.as_str() {
                "i64" => Ok(Type::I64),
                "f64" => Ok(Type::F64),
                "bool" => Ok(Type::Bool),
                "string" => Ok(Type::String),
                "Tensor" => {
                    self.expect('<')?;
                    if !self.word("f64") {
                        return Err(self.fail("v0.1 tensors require f64 elements"));
                    }
                    self.expect(',')?;
                    self.expect('[')?;
                    let m = self.dimension()?;
                    self.expect(',')?;
                    let n = self.dimension()?;
                    self.expect(']')?;
                    self.expect('>')?;
                    Ok(Type::Tensor(m, n))
                }
                _ => Err(error(token.span, "E_TYPE", "unsupported type")),
            },
            TokenKind::Symbol('(') => {
                self.expect(')')?;
                Ok(Type::Unit)
            }
            _ => Err(error(token.span, "E_TYPE", "expected type")),
        }
    }
    fn dimension(&mut self) -> Result<usize, Error> {
        let t = self.bump();
        if let TokenKind::Number(s) = t.kind {
            if let Ok(n) = s.parse::<usize>() {
                if n > 0 && n <= 1_000_000 {
                    return Ok(n);
                }
            }
        }
        Err(error(
            t.span,
            "E_SHAPE",
            "dimension must be an integer in 1..=1000000",
        ))
    }
    fn function(&mut self) -> Result<Function, Error> {
        let start = self.current().span.start;
        if !self.word("fn") {
            return Err(self.fail("expected fn or функц"));
        }
        let name = self.ident()?;
        self.expect('(')?;
        let mut params = Vec::new();
        if !self.symbol(')') {
            loop {
                let p = self.ident()?;
                self.expect(':')?;
                let t = self.ty()?;
                params.push((p, t));
                if self.symbol(')') {
                    break;
                }
                self.expect(',')?;
            }
        }
        let result = if self.current().kind == TokenKind::Arrow {
            self.bump();
            self.ty()?
        } else {
            Type::Unit
        };
        self.expect('{')?;
        let mut body = Vec::new();
        while !self.symbol('}') {
            if self.current().kind == TokenKind::Eof {
                return Err(self.fail("missing closing brace"));
            }
            body.push(self.stmt()?);
        }
        let end = self.tokens[self.pos - 1].span.end;
        Ok(Function {
            name,
            params,
            result,
            body,
            span: Span { start, end },
        })
    }
    fn stmt(&mut self) -> Result<Stmt, Error> {
        self.nodes = 0;
        let stmt = if self.word("let") {
            let name = self.ident()?;
            let ty = if self.symbol(':') {
                Some(self.ty()?)
            } else {
                None
            };
            self.expect('=')?;
            Stmt::Let(name, ty, self.expr(0)?)
        } else if self.word("return") {
            Stmt::Return(if self.current().kind == TokenKind::Symbol(';') {
                None
            } else {
                Some(self.expr(0)?)
            })
        } else if self.word("print") {
            self.expect('(')?;
            let e = self.expr(0)?;
            self.expect(')')?;
            Stmt::Print(e)
        } else {
            Stmt::Expr(self.expr(0)?)
        };
        self.expect(';')?;
        Ok(stmt)
    }
    fn expr(&mut self, min: u8) -> Result<Expr, Error> {
        self.nodes += 1;
        if self.nodes > 256 {
            return Err(self.fail("expression exceeds 256 nodes"));
        }
        if self.depth >= 128 {
            return Err(self.fail("expression nesting exceeds 128"));
        }
        self.depth += 1;
        let result = self.expression(min);
        self.depth -= 1;
        result
    }
    fn expression(&mut self, min: u8) -> Result<Expr, Error> {
        let t = self.bump();
        let start = t.span.start;
        let kind = match t.kind {
            TokenKind::Number(s) => {
                if s.contains('.') {
                    let x = s
                        .parse::<f64>()
                        .map_err(|_| error(t.span, "E_NUMBER", "invalid float"))?;
                    if !x.is_finite() {
                        return Err(error(t.span, "E_NUMBER", "float literal out of range"));
                    }
                    ExprKind::Float(x)
                } else {
                    ExprKind::Int(
                        s.parse().map_err(|_| {
                            error(t.span, "E_NUMBER", "integer literal out of range")
                        })?,
                    )
                }
            }
            TokenKind::Str(s) => ExprKind::Str(s),
            TokenKind::Word(s) if s == "true" || s == "false" => ExprKind::Bool(s == "true"),
            TokenKind::Word(s) if !reserved(&s) => {
                if self.symbol('(') {
                    let mut args = Vec::new();
                    if !self.symbol(')') {
                        loop {
                            args.push(self.expr(0)?);
                            if self.symbol(')') {
                                break;
                            }
                            self.expect(',')?;
                        }
                    }
                    ExprKind::Call(s, args)
                } else {
                    ExprKind::Var(s)
                }
            }
            TokenKind::Symbol('-') => {
                if matches!(&self.current().kind, TokenKind::Number(s) if s.trim_start_matches('0') == "9223372036854775808")
                {
                    self.bump();
                    ExprKind::Int(i64::MIN)
                } else {
                    ExprKind::Neg(Box::new(self.expr(30)?))
                }
            }
            TokenKind::Symbol('(') => {
                let e = self.expr(0)?;
                self.expect(')')?;
                e.kind
            }
            TokenKind::Symbol('[') => {
                let mut rows = Vec::new();
                loop {
                    self.expect('[')?;
                    let mut row = Vec::new();
                    row.push(self.expr(0)?);
                    while self.symbol(',') {
                        row.push(self.expr(0)?);
                    }
                    self.expect(']')?;
                    rows.push(row);
                    if self.symbol(']') {
                        break;
                    }
                    self.expect(',')?;
                }
                let width = rows[0].len();
                if rows.iter().any(|r| r.len() != width) {
                    return Err(error(
                        Span {
                            start,
                            end: self.tokens[self.pos - 1].span.end,
                        },
                        "E_SHAPE",
                        "ragged tensor literal",
                    ));
                }
                ExprKind::Tensor(rows)
            }
            _ => return Err(error(t.span, "E_PARSE", "expected expression")),
        };
        let mut left = Expr {
            kind,
            span: Span {
                start,
                end: self.tokens[self.pos.saturating_sub(1)].span.end,
            },
        };
        loop {
            let (op, prec) = match self.current().kind {
                TokenKind::Symbol(c @ ('+' | '-')) => (c, 10),
                TokenKind::Symbol(c @ ('*' | '/' | '@')) => (c, 20),
                _ => break,
            };
            if prec < min {
                break;
            }
            self.bump();
            let right = self.expr(prec + 1)?;
            let span = Span {
                start: left.span.start,
                end: right.span.end,
            };
            left = Expr {
                kind: ExprKind::Binary(op, Box::new(left), Box::new(right)),
                span,
            };
        }
        Ok(left)
    }
}
pub fn parse(source: &str) -> Result<Program, Error> {
    parse_tokens(lex(source)?)
}

// Only the lexer constructs this stream, including its final EOF token.
fn parse_tokens(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut parser = Parser {
        tokens,
        pos: 0,
        depth: 0,
        nodes: 0,
    };
    let mut program = Vec::new();
    while parser.current().kind != TokenKind::Eof {
        program.push(parser.function()?);
    }
    Ok(program)
}

#[cfg(test)]
#[path = "bench_tests.rs"]
mod bench_tests;
