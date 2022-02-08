use crate::ast::Ast;
use crate::ast::Expr;
use crate::ast::Item;
use crate::ast::RelOp;
use crate::lex::Lexer;
use crate::token::TKind;
use crate::token::TKind::*;
use crate::token::Token;

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

    pub fn ast(mut self) -> Result<Ast, String> {
        self.curr = self.next.clone();
        self.next = self.lexer.scan();
        self.advance()?;

        self.expression()?;
        while self.next.kind != TkEof {
            if self.next.kind == TkErr {
                return Err(format!("unrecognized character '{}'", self.next.lexeme()));
            }
            self.consume_next(TkSemi, "expected ';' after expression")?;
            if self.next.kind != TkEof {
                self.advance()?;
                self.expression()?;
            } else {
                break;
            }
        }

        let mut root = Ast::new();
        for expr in self.stack {
            root.nodes.push(Item::Expr(expr));
        }

        Ok(root)
    }

    fn advance(&mut self) -> Result<(), String> {
        self.curr = self.next.clone();
        self.next = self.lexer.scan();
        match self.curr.kind {
            TkEof => Err(format!("reached end-of-file")),
            TkErr => Err(format!("unrecognized character '{}'", self.curr.lexeme())),
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
        self.expr_precedence(Prec::Relation)
    }

    fn expr_precedence(&mut self, prec: Prec) -> Result<(), String> {
        self.dispatch_prefix_op()?;
        while prec <= Prec::of(&self.next.kind) {
            self.advance()?;
            self.dispatch_infix_op()?;
        }
        Ok(())
    }

    fn expr_relation(&mut self) -> Result<(), String> {
        let init = self.stack.pop().unwrap();

        let mut relations = Vec::new();
        let mut operator = self.curr.kind;
        let mut prec = Prec::of(&operator);

        loop {
            self.advance()?;
            self.expr_precedence(prec.higher())?;

            let rel_op = RelOp::from(operator);
            let rel_expr = self.stack.pop().unwrap();
            relations.push((rel_op, rel_expr));

            operator = self.next.kind;
            prec = Prec::of(&operator);
            if prec == Prec::Relation {
                self.advance()?;
            } else {
                break;
            }
        }

        let expr = Expr::Relation(Box::new(init), relations);
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
            TkPlus => Expr::Add(Box::new(lhs), Box::new(rhs)),
            TkMinus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
            TkStar => Expr::Mul(Box::new(lhs), Box::new(rhs)),
            TkSlash => Expr::Div(Box::new(lhs), Box::new(rhs)),
            TkPercent => Expr::Mod(Box::new(lhs), Box::new(rhs)),
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
            TkMinus => Expr::Negate(Box::new(expr)),
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
            TkCaret => Expr::Power(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        };

        self.stack.push(expr);
        Ok(())
    }

    fn expr_group(&mut self) -> Result<(), String> {
        self.advance()?;
        self.expression()?;
        self.consume_next(TkRparen, "expected ')' after expression")?;
        Ok(())
    }

    fn expr_literal(&mut self) -> Result<(), String> {
        let expr = match self.curr.kind {
            TkTrue => Expr::Bool(true),
            TkFalse => Expr::Bool(false),
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
            TkReal => self.expr_num(),
            TkTrue => self.expr_literal(),
            TkFalse => self.expr_literal(),
            TkIdent => self.expr_ident(),
            TkLparen => self.expr_group(),
            TkMinus => self.expr_unary(),
            _ => Err(format!("expected an expression")),
        };
    }

    fn dispatch_infix_op(&mut self) -> Result<(), String> {
        return match self.curr.kind {
            TkCaret => self.expr_power(),
            TkPlus => self.expr_binary(),
            TkMinus => self.expr_binary(),
            TkStar => self.expr_binary(),
            TkSlash => self.expr_binary(),
            TkPercent => self.expr_binary(),
            TkLt => self.expr_relation(),
            TkGt => self.expr_relation(),
            TkLtEq => self.expr_relation(),
            TkGtEq => self.expr_relation(),
            TkEqEq => self.expr_relation(),
            TkNotEq => self.expr_relation(),
            _ => panic!(),
        };
    }
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
enum Prec {
    None = 0,
    Relation, // < > <= >= == !=
    Term,     // + -
    Factor,   // * / %
    Unary,    // -
    Power,    // ^
    Primary,
}

impl Prec {
    fn of(tkind: &TKind) -> Self {
        match tkind {
            TkCaret => Self::Power,
            TkPlus => Self::Term,
            TkMinus => Self::Term,
            TkStar => Self::Factor,
            TkSlash => Self::Factor,
            TkPercent => Self::Factor,
            TkLt => Self::Relation,
            TkGt => Self::Relation,
            TkLtEq => Self::Relation,
            TkGtEq => Self::Relation,
            TkEqEq => Self::Relation,
            TkNotEq => Self::Relation,
            _ => Self::None,
        }
    }

    fn higher(&self) -> Self {
        match self {
            Self::None => Self::Relation,
            Self::Relation => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Power,
            Self::Power => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}
