use crate::token::TKind;
use crate::token::TKind::*;

pub enum Item {
    // Mod,
    // Use,
    // Def,
    // Impl,
    // Alias,
    // Trait,
    //Fn(FnDef),
    // Const,
    //Let(LetBind),
    Expr(Expr),
}

pub struct Ast {
    pub nodes: Vec<Item>,
}

impl Ast {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

// struct FnDef {
//     name: String,
//     params: Vec<FnParam>,
//     body: Vec<Item>,
//     ret: Option<TyHint>,
// }
//
// struct FnParam {
//     name: String,
//     ty: Option<TyHint>,
// }
//
// struct LetBind {
//     name: String,
//     init: Expr,
//     ty: Option<TyHint>,
// }

#[derive(PartialEq)]
pub enum RelOp {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}

impl RelOp {
    pub fn from(tkind: TKind) -> Self {
        match tkind {
            TkLt => RelOp::Lt,
            TkGt => RelOp::Gt,
            TkLtEq => RelOp::Le,
            TkGtEq => RelOp::Ge,
            TkEqEq => RelOp::Eq,
            TkNotEq => RelOp::Ne,
            _ => panic!(),
        }
    }
}

pub enum Expr {
    Int(i32),
    Real(f64),
    Bool(bool),
    Ident(String),
    Call(Box<Expr>, Vec<Expr>),
    Power(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Relation(Box<Expr>, Vec<(RelOp, Expr)>),
}
