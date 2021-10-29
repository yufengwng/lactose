use crate::ast::CompOp;
use crate::ast::Expr;
use crate::ast::TKind;
use crate::ast::Token;
use crate::lex::Lexer;

pub struct Parser<'a> {
    stack: Vec<Expr>,
    lexer: Lexer<'a>,
    curr: Token<'a>,
    next: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            stack: Vec::new(),
            lexer: Lexer::new(src),
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
            self.consume_next(TKind::Semi, "expected ';' after expression")?;
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

    fn consume_next(&mut self, tkind: TKind, message: &str) -> Result<(), String> {
        if self.next.kind == tkind {
            return self.advance();
        }
        Err(message.to_owned())
    }

    fn expression(&mut self) -> Result<(), String> {
        self.expr_precedence(Prec::Compare)
    }

    fn expr_precedence(&mut self, prec: Prec) -> Result<(), String> {
        self.dispatch_prefix_op()?;
        while prec <= Prec::of(&self.next.kind) {
            self.advance()?;
            self.dispatch_infix_op()?;
        }
        Ok(())
    }

    fn expr_compare(&mut self) -> Result<(), String> {
        let init = self.stack.pop().unwrap();

        let mut compares = Vec::new();
        let mut operator = self.curr.kind;
        let mut prec = Prec::of(&operator);

        loop {
            self.advance()?;
            self.expr_precedence(prec.higher())?;

            let comp_op = CompOp::from(operator);
            let comp_expr = self.stack.pop().unwrap();
            compares.push((comp_op, comp_expr));

            operator = self.next.kind;
            prec = Prec::of(&operator);
            if prec == Prec::Compare {
                self.advance()?;
            } else {
                break;
            }
        }

        let expr = Expr::Compare(Box::new(init), compares);
        self.stack.push(expr);
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
        self.consume_next(TKind::Rparen, "expected ')' after expression")?;
        Ok(())
    }

    fn expr_literal(&mut self) -> Result<(), String> {
        let expr = match self.curr.kind {
            TKind::True => Expr::Bool(true),
            TKind::False => Expr::Bool(false),
            _ => unreachable!(),
        };
        self.stack.push(expr);
        Ok(())
    }

    fn expr_num(&mut self) -> Result<(), String> {
        let expr = match self.curr.lexeme().parse::<f64>() {
            Err(_) => return Err(format!("invalid number format")),
            Ok(num) => Expr::Num(num),
        };
        self.stack.push(expr);
        Ok(())
    }

    fn expr_ident(&mut self) -> Result<(), String> {
        self.stack.push(Expr::Ident);
        Ok(())
    }

    fn dispatch_prefix_op(&mut self) -> Result<(), String> {
        return match self.curr.kind {
            TKind::Num => self.expr_num(),
            TKind::True => self.expr_literal(),
            TKind::False => self.expr_literal(),
            TKind::Ident => self.expr_ident(),
            TKind::Lparen => self.expr_group(),
            TKind::Minus => self.expr_unary(),
            _ => Err(format!("expected an expression")),
        };
    }

    fn dispatch_infix_op(&mut self) -> Result<(), String> {
        return match self.curr.kind {
            TKind::Caret => self.expr_power(),
            TKind::Plus => self.expr_binary(),
            TKind::Minus => self.expr_binary(),
            TKind::Star => self.expr_binary(),
            TKind::Slash => self.expr_binary(),
            TKind::Percent => self.expr_binary(),
            TKind::Lt => self.expr_compare(),
            TKind::Gt => self.expr_compare(),
            TKind::LtEq => self.expr_compare(),
            TKind::GtEq => self.expr_compare(),
            TKind::EqEq => self.expr_compare(),
            TKind::NotEq => self.expr_compare(),
            _ => panic!(),
        };
    }
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
enum Prec {
    None = 0,
    Compare, // < > <= >= == !=
    Term,    // + -
    Factor,  // * / %
    Unary,   // -
    Power,   // ^
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
            TKind::Lt => Self::Compare,
            TKind::Gt => Self::Compare,
            TKind::LtEq => Self::Compare,
            TKind::GtEq => Self::Compare,
            TKind::EqEq => Self::Compare,
            TKind::NotEq => Self::Compare,
            _ => Self::None,
        }
    }

    fn higher(&self) -> Self {
        match self {
            Self::None => Self::Compare,
            Self::Compare => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Power,
            Self::Power => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}
