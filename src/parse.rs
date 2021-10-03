use crate::ast::Expr;
use crate::ast::TKind;
use crate::ast::Token;
use crate::lex::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Token<'a>,
    next: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            curr: Token::eof(),
            next: Token::eof(),
        }
    }

    pub fn ast(&mut self) -> Result<Expr, String> {
        std::mem::swap(&mut self.curr, &mut self.next);
        self.next = self.lexer.scan();
        self.advance()?;

        let expr = self.expression()?;
        match self.next.kind {
            TKind::EOF => Ok(expr),
            TKind::Err => Err(format!("unrecognized character '{}'", self.next.lexeme())),
            _ => Err(format!("currenly only one expression allowed")),
        }
    }

    fn advance(&mut self) -> Result<(), String> {
        std::mem::swap(&mut self.curr, &mut self.next);
        self.next = self.lexer.scan();
        match self.curr.kind {
            TKind::EOF => Err(format!("reached end-of-file")),
            TKind::Err => Err(format!("unrecognized character '{}'", self.curr.lexeme())),
            _ => Ok(()),
        }
    }

    fn consume(&mut self, tkind: TKind, message: &str) -> Result<(), String> {
        if self.next.kind == tkind {
            return self.advance();
        }
        Err(message.to_owned())
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.expr_precedence(Prec::Term)
    }

    fn expr_precedence(&mut self, prec: Prec) -> Result<Expr, String> {
        let prefix_fn = self.op_prefix(&self.curr.kind);
        let prefix_fn = match prefix_fn {
            None => return Err(format!("expected an expression")),
            Some(f) => f,
        };

        let mut expr = prefix_fn(self)?;
        while prec <= Prec::of(&self.next.kind) {
            self.advance()?;
            let infix_fn = self.op_infix(&self.curr.kind);
            let infix_fn = infix_fn.expect("infix");
            expr = infix_fn(self, expr)?;
        }

        Ok(expr)
    }

    fn expr_binary(&mut self, lhs: Expr) -> Result<Expr, String> {
        let operator = self.curr.kind;
        let prec = Prec::of(&operator);
        self.advance()?;

        let rhs = self.expr_precedence(prec.stronger())?;
        Ok(match operator {
            TKind::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
            TKind::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
            TKind::Star => Expr::Mul(Box::new(lhs), Box::new(rhs)),
            TKind::Slash => Expr::Div(Box::new(lhs), Box::new(rhs)),
            TKind::Percent => Expr::Mod(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        })
    }

    fn expr_unary(&mut self) -> Result<Expr, String> {
        let operator = self.curr.kind;
        self.advance()?;

        let expr = self.expr_precedence(Prec::Unary)?;
        Ok(match operator {
            TKind::Minus => Expr::Negate(Box::new(expr)),
            _ => unreachable!(),
        })
    }

    fn expr_power(&mut self, lhs: Expr) -> Result<Expr, String> {
        let operator = self.curr.kind;
        self.advance()?;

        let rhs = self.expr_precedence(Prec::Power)?;
        Ok(match operator {
            TKind::Caret => Expr::Power(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        })
    }

    fn expr_group(&mut self) -> Result<Expr, String> {
        self.advance()?;
        let expr = self.expression()?;
        self.consume(TKind::Rparen, "expected ')' after expression")?;
        Ok(expr)
    }

    fn expr_num(&mut self) -> Result<Expr, String> {
        match self.curr.lexeme().parse::<f64>() {
            Err(_) => Err(format!("invalid number format")),
            Ok(num) => Ok(Expr::Num(num)),
        }
    }

    fn expr_ident(&mut self) -> Result<Expr, String> {
        Ok(Expr::Ident)
    }

    fn op_prefix(&self, tkind: &TKind) -> Option<Box<PrefixParseFn<'a>>> {
        Some(Box::new(match tkind {
            TKind::Num => Self::expr_num,
            TKind::Ident => Self::expr_ident,
            TKind::Lparen => Self::expr_group,
            TKind::Minus => Self::expr_unary,
            _ => return None,
        }))
    }

    fn op_infix(&self, tkind: &TKind) -> Option<Box<InfixParseFn<'a>>> {
        Some(Box::new(match tkind {
            TKind::Caret => Self::expr_power,
            TKind::Plus => Self::expr_binary,
            TKind::Minus => Self::expr_binary,
            TKind::Star => Self::expr_binary,
            TKind::Slash => Self::expr_binary,
            TKind::Percent => Self::expr_binary,
            _ => return None,
        }))
    }
}

type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Result<Expr, String>;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Expr) -> Result<Expr, String>;

#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
enum Prec {
    None,
    Term,   // + -
    Factor, // * / %
    Unary,  // -
    Power,  // ^
    Primary,
}

impl Prec {
    fn of(tkind: &TKind) -> Self {
        match tkind {
            TKind::Caret => Self::Power,
            TKind::Plus => Self::Term,
            TKind::Minus => Self::Term,
            TKind::Star => Self::Factor,
            TKind::Slash => Self::Factor,
            TKind::Percent => Self::Factor,
            _ => Self::None,
        }
    }

    fn stronger(&self) -> Self {
        match self {
            Self::None => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Power,
            Self::Power => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}
