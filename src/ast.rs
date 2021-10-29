#[derive(Copy, Clone, PartialEq)]
pub enum TKind {
    EOF,
    Err,
    Num,
    True,
    False,
    Ident,
    Semi,
    Lparen,
    Rparen,
    Caret,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Lt,
    Gt,
    LtEq,
    GtEq,
    EqEq,
    NotEq,
}

#[derive(Clone)]
pub struct Token<'a> {
    pub kind: TKind,
    span: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn eof() -> Self {
        Self::new(TKind::EOF, &[])
    }

    pub fn new(kind: TKind, span: &'a [u8]) -> Self {
        Self { kind, span }
    }

    pub fn lexeme(&self) -> &'a str {
        std::str::from_utf8(self.span).unwrap()
    }
}

#[derive(PartialEq)]
pub enum CompOp {
    Lt,
    Gt,
    LtEq,
    GtEq,
    EqEq,
    NotEq,
}

impl CompOp {
    pub fn from(tkind: TKind) -> Self {
        match tkind {
            TKind::Lt => CompOp::Lt,
            TKind::Gt => CompOp::Gt,
            TKind::LtEq => CompOp::LtEq,
            TKind::GtEq => CompOp::GtEq,
            TKind::EqEq => CompOp::EqEq,
            TKind::NotEq => CompOp::NotEq,
            _ => panic!(),
        }
    }

    pub fn apply(&self, lhs: f64, rhs: f64) -> bool {
        match self {
            CompOp::Lt => lhs < rhs,
            CompOp::Gt => lhs > rhs,
            CompOp::LtEq => lhs <= rhs,
            CompOp::GtEq => lhs >= rhs,
            CompOp::EqEq => lhs == rhs,
            CompOp::NotEq => lhs != rhs,
        }
    }
}

pub enum Expr {
    Ident,
    Num(f64),
    Bool(bool),
    Power(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Compare(Box<Expr>, Vec<(CompOp, Expr)>),
}
