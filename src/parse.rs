use crate::lex::Lexer;
use crate::lex::TKind;

pub enum Expr {
    Err(String),
    Num(f64),
    Group(Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Expr {
        let token = match self.lexer.next() {
            None => return Expr::Err("reached end-of-file".to_owned()),
            Some(t) => t,
        };

        match token.kind {
            TKind::Err => Expr::Err(
                format!("unrecognized character: {}", token.lexeme())),
            TKind::Lparen => {
                let expr = self.parse();
                if !self.consume(TKind::Rparen) {
                    Expr::Err("expected closing parenthesis".to_owned())
                } else {
                    Expr::Group(Box::new(expr))
                }
            }
            TKind::Minus => {
                let expr = self.parse();
                Expr::Negate(Box::new(expr))
            }
            TKind::Num => {
                let lhs = token.lexeme().parse().unwrap();
                let lhs = Box::new(Expr::Num(lhs));

                let op = match self.lexer.next() {
                    None => return Expr::Err("expected operator".to_owned()),
                    Some(t) => t,
                };
                let dummy = Box::new(Expr::Num(0.0));
                let partial = match op.kind {
                    TKind::Caret => Expr::Power(lhs, dummy),
                    TKind::Plus => Expr::Add(lhs, dummy),
                    TKind::Minus => Expr::Sub(lhs, dummy),
                    TKind::Star => Expr::Mul(lhs, dummy),
                    TKind::Slash => Expr::Div(lhs, dummy),
                    TKind::Percent => Expr::Mod(lhs, dummy),
                    _ => return Expr::Err(
                        format!("unrecognized operator: {}", op.lexeme())),
                };

                let rhs = match self.lexer.next() {
                    None => return Expr::Err("expected operand".to_owned()),
                    Some(t) => t.lexeme().parse().unwrap(),
                };
                let rhs = Box::new(Expr::Num(rhs));

                match partial {
                    Expr::Err(_) => partial,
                    Expr::Power(lhs, _) => Expr::Power(lhs, rhs),
                    Expr::Mul(lhs, _) => Expr::Mul(lhs, rhs),
                    Expr::Div(lhs, _) => Expr::Div(lhs, rhs),
                    Expr::Mod(lhs, _) => Expr::Mod(lhs, rhs),
                    Expr::Add(lhs, _) => Expr::Add(lhs, rhs),
                    Expr::Sub(lhs, _) => Expr::Sub(lhs, rhs),
                    _ => unreachable!(),
                }
            }
            _ => Expr::Err(
                format!("unrecognized starting token: {}", token.lexeme())),
        }
    }

    fn consume(&mut self, kind: TKind) -> bool {
        let token = match self.lexer.next() {
            None => return false,
            Some(t) => t,
        };
        token.kind == kind
    }
}
