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
pub enum RelOp {
    Lt,
    Gt,
    LtEq,
    GtEq,
    EqEq,
    NotEq,
}

impl RelOp {
    pub fn from(tkind: TKind) -> Self {
        match tkind {
            TKind::Lt => RelOp::Lt,
            TKind::Gt => RelOp::Gt,
            TKind::LtEq => RelOp::LtEq,
            TKind::GtEq => RelOp::GtEq,
            TKind::EqEq => RelOp::EqEq,
            TKind::NotEq => RelOp::NotEq,
            _ => panic!(),
        }
    }

    pub fn apply(&self, lhs: f64, rhs: f64) -> bool {
        match self {
            RelOp::Lt => lhs < rhs,
            RelOp::Gt => lhs > rhs,
            RelOp::LtEq => lhs <= rhs,
            RelOp::GtEq => lhs >= rhs,
            RelOp::EqEq => lhs == rhs,
            RelOp::NotEq => lhs != rhs,
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
    Relation(Box<Expr>, Vec<(RelOp, Expr)>),
}
