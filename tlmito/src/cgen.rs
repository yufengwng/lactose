use tllang::ast::Ast;
use tllang::ast::Expr;
use tllang::ast::Item;
use tllang::ast::RelOp;

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
                Item::Expr(expr) => self.emit_expr(&mut chunk, &expr)?,
            }
        }
        Ok(chunk)
    }

    fn emit_expr(&self, chunk: &mut Chunk, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Int(lit) => {
                self.emit_const(chunk, Value::Int(*lit));
            }
            Expr::Real(lit) => {
                self.emit_const(chunk, Value::Real(*lit));
            }
            Expr::Bool(lit) => {
                chunk.write(if *lit { OpTrue } else { OpFalse });
            }
            Expr::Ident(lit) => {
                let name = Value::Str(lit.to_owned());
                let idx = chunk.add(name);
                chunk.write(OpGet);
                chunk.write_byte(idx as u8);
            }
            Expr::Power(base, exp) => {
                self.emit_expr(chunk, base)?;
                self.emit_expr(chunk, exp)?;
                chunk.write(OpPow);
            }
            Expr::Negate(inner) => {
                self.emit_expr(chunk, inner)?;
                chunk.write(OpNeg);
            }
            Expr::Add(lhs, rhs) => {
                self.emit_expr(chunk, lhs)?;
                self.emit_expr(chunk, rhs)?;
                chunk.write(OpAdd);
            }
            Expr::Sub(lhs, rhs) => {
                self.emit_expr(chunk, lhs)?;
                self.emit_expr(chunk, rhs)?;
                chunk.write(OpSub);
            }
            Expr::Mul(lhs, rhs) => {
                self.emit_expr(chunk, lhs)?;
                self.emit_expr(chunk, rhs)?;
                chunk.write(OpMul);
            }
            Expr::Div(lhs, rhs) => {
                self.emit_expr(chunk, lhs)?;
                self.emit_expr(chunk, rhs)?;
                chunk.write(OpDiv);
            }
            Expr::Rem(lhs, rhs) => {
                self.emit_expr(chunk, lhs)?;
                self.emit_expr(chunk, rhs)?;
                chunk.write(OpRem);
            }
            Expr::Relation(lhs, ops) => {
                // todo: handle chained comparisons
                let (op, rhs) = &ops[0];
                self.emit_expr(chunk, lhs)?;
                self.emit_expr(chunk, rhs)?;
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
            Expr::Call(callee, args) => {
                self.emit_call(chunk, callee, args)?;
            }
        }
        Ok(())
    }

    fn emit_call(&self, chunk: &mut Chunk, callee: &Expr, args: &[Expr]) -> Result<(), String> {
        match callee {
            Expr::Ident(_) => self.emit_expr(chunk, callee)?,
            _ => return Err(format!("can only call functions")),
        };
        for arg in args {
            self.emit_expr(chunk, arg)?;
        }
        chunk.write(OpCall);
        chunk.write_byte(args.len() as u8);
        Ok(())
    }

    fn emit_const(&self, chunk: &mut Chunk, value: Value) {
        let idx = chunk.add(value);
        chunk.write(OpConst);
        chunk.write_byte(idx as u8);
    }
}
