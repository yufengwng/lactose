use ltlang::ast::Ast;
use ltlang::ast::Expr;
use ltlang::ast::Item;
use ltlang::ast::RelOp;

use crate::code::Chunk;
use crate::code::OpCode::*;
use crate::value::Value;

pub struct CodeGen {}

impl CodeGen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, ast: &Ast) -> Result<Chunk, String> {
        let mut chunk = Chunk::new();
        for item in &ast.nodes {
            match item {
                // Item::Fn(_) => unimplemented!(),
                // Item::Let(_) => unimplemented!(),
                Item::Expr(expr) => self.emit_expr(&mut chunk, &expr),
            }
        }
        Ok(chunk)
    }

    fn emit_expr(&self, chunk: &mut Chunk, expr: &Expr) {
        match expr {
            Expr::Ident => {
                let name = Value::Str("_".to_owned());
                let idx = chunk.add(name);
                chunk.write(OpGet);
                chunk.write_byte(idx as u8);
            }
            Expr::Int(lit) => {
                self.emit_const(chunk, Value::Int(*lit));
            }
            Expr::Real(lit) => {
                self.emit_const(chunk, Value::Real(*lit));
            }
            Expr::Bool(lit) => {
                chunk.write(if *lit { OpTrue } else { OpFalse });
            }
            Expr::Power(base, exp) => {
                self.emit_expr(chunk, base);
                self.emit_expr(chunk, exp);
                chunk.write(OpPow);
            }
            Expr::Negate(inner) => {
                self.emit_expr(chunk, inner);
                chunk.write(OpNeg);
            }
            Expr::Add(lhs, rhs) => {
                self.emit_expr(chunk, lhs);
                self.emit_expr(chunk, rhs);
                chunk.write(OpAdd);
            }
            Expr::Sub(lhs, rhs) => {
                self.emit_expr(chunk, lhs);
                self.emit_expr(chunk, rhs);
                chunk.write(OpSub);
            }
            Expr::Mul(lhs, rhs) => {
                self.emit_expr(chunk, lhs);
                self.emit_expr(chunk, rhs);
                chunk.write(OpMul);
            }
            Expr::Div(lhs, rhs) => {
                self.emit_expr(chunk, lhs);
                self.emit_expr(chunk, rhs);
                chunk.write(OpDiv);
            }
            Expr::Rem(lhs, rhs) => {
                self.emit_expr(chunk, lhs);
                self.emit_expr(chunk, rhs);
                chunk.write(OpRem);
            }
            Expr::Relation(lhs, ops) => {
                // todo: handle chained comparisons
                let (op, rhs) = &ops[0];
                self.emit_expr(chunk, lhs);
                self.emit_expr(chunk, rhs);
                let op = match op {
                    RelOp::Lt => OpLt,
                    RelOp::Gt => OpGt,
                    RelOp::Le => OpLtEq,
                    RelOp::Ge => OpGtEq,
                    RelOp::Eq => OpEqual,
                    RelOp::Ne => OpNotEq,
                };
                chunk.write(op);
            }
        }
    }

    fn emit_const(&self, chunk: &mut Chunk, value: Value) {
        let idx = chunk.add(value);
        chunk.write(OpConst);
        chunk.write_byte(idx as u8);
    }
}
