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
}
