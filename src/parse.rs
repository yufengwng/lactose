use crate::ast::Expr;
use crate::ast::TKind;
use crate::ast::Token;
use crate::lex::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    stack: Vec<Expr>,
    curr: Token<'a>,
    next: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            stack: Vec::new(),
            curr: Token::eof(),
            next: Token::eof(),
        }
    }

    pub fn ast(mut self) -> Result<Vec<Expr>, String> {
        self.curr = self.next.clone();
        self.next = self.lexer.scan();
        self.advance()?;

        self.expression()?;
        while self.next.kind != TKind::EOF {
            if self.next.kind == TKind::Err {
                return Err(format!("unrecognized character '{}'", self.next.lexeme()));
            }
            self.consume(TKind::Semi, "expected ';' after expression")?;
            if self.next.kind != TKind::EOF {
                self.advance()?;
                self.expression()?;
            } else {
                break;
            }
        }

        Ok(self.stack)
    }

    fn advance(&mut self) -> Result<(), String> {
        self.curr = self.next.clone();
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

    fn expression(&mut self) -> Result<(), String> {
        self.expr_precedence(Prec::Term)
    }

    fn expr_precedence(&mut self, prec: Prec) -> Result<(), String> {
        let prefix_fn = self.op_prefix(&self.curr.kind);
        let prefix_fn = match prefix_fn {
            None => return Err(format!("expected an expression")),
            Some(f) => f,
        };

        prefix_fn(self)?;
        while prec <= Prec::of(&self.next.kind) {
            self.advance()?;
            let infix_fn = self.op_infix(&self.curr.kind);
            let infix_fn = infix_fn.expect("infix");
            infix_fn(self)?;
        }

        Ok(())
    }

    fn expr_binary(&mut self) -> Result<(), String> {
        let operator = self.curr.kind;
        let prec = Prec::of(&operator);

        self.advance()?;
        self.expr_precedence(prec.higher())?;

        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();

        let expr = match operator {
            TKind::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
            TKind::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
            TKind::Star => Expr::Mul(Box::new(lhs), Box::new(rhs)),
            TKind::Slash => Expr::Div(Box::new(lhs), Box::new(rhs)),
            TKind::Percent => Expr::Mod(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        };

        self.stack.push(expr);
        Ok(())
    }

    fn expr_unary(&mut self) -> Result<(), String> {
        let operator = self.curr.kind;

        self.advance()?;
        self.expr_precedence(Prec::Unary)?;

        let expr = self.stack.pop().unwrap();
        let expr = match operator {
            TKind::Minus => Expr::Negate(Box::new(expr)),
            _ => unreachable!(),
        };

        self.stack.push(expr);
        Ok(())
    }

    fn expr_power(&mut self) -> Result<(), String> {
        let operator = self.curr.kind;

        self.advance()?;
        self.expr_precedence(Prec::Power)?;

        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();

        let expr = match operator {
            TKind::Caret => Expr::Power(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        };

        self.stack.push(expr);
        Ok(())
    }

    fn expr_group(&mut self) -> Result<(), String> {
        self.advance()?;
        self.expression()?;
        self.consume(TKind::Rparen, "expected ')' after expression")?;
        Ok(())
    }

    fn expr_num(&mut self) -> Result<(), String> {
        match self.curr.lexeme().parse::<f64>() {
            Err(_) => return Err(format!("invalid number format")),
            Ok(num) => {
                self.stack.push(Expr::Num(num));
                Ok(())
            }
        }
    }

    fn expr_ident(&mut self) -> Result<(), String> {
        self.stack.push(Expr::Ident);
        Ok(())
    }

    fn op_prefix(&self, tkind: &TKind) -> Option<Box<ParseFn<'a>>> {
        Some(Box::new(match tkind {
            TKind::Num => Self::expr_num,
            TKind::Ident => Self::expr_ident,
            TKind::Lparen => Self::expr_group,
            TKind::Minus => Self::expr_unary,
            _ => return None,
        }))
    }

    fn op_infix(&self, tkind: &TKind) -> Option<Box<ParseFn<'a>>> {
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

type ParseFn<'a> = fn(&mut Parser<'a>) -> Result<(), String>;

#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
enum Prec {
    None = 0,
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

    fn higher(&self) -> Self {
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
