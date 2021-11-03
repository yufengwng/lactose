use crate::ast::{RelOp, Expr};
use crate::parse::Parser;
use crate::value::Value;

pub struct Aqvm {
    underscore: Value,
}

impl Aqvm {
    pub fn new() -> Self {
        Self {
            underscore: Value::Num(0.0),
        }
    }

    pub fn run(&mut self, source: &str) {
        let parser = Parser::new(source);

        let list = match parser.ast() {
            Ok(ls) => ls,
            Err(msg) => {
                eprintln!("[E] {}", msg);
                return;
            }
        };

        for expr in list {
            self.underscore = match self.eval(expr) {
                Ok(value) => value,
                Err(msg) => {
                    eprintln!("[E] {}", msg);
                    return;
                }
            };
        }

        println!("{}", self.underscore);
    }

    fn eval(&self, expr: Expr) -> Result<Value, String> {
        macro_rules! check_nums {
            ($val:ident, $msg:literal) => {{
                if !$val.is_num() {
                    return Err(format!($msg));
                }
                $val.as_num()
            }};
            ($lhs:ident, $rhs:ident, $msg:literal) => {{
                if !$lhs.is_num() || !$rhs.is_num() {
                    return Err(format!($msg));
                }
                ($lhs.as_num(), $rhs.as_num())
            }};
        }
        Ok(match expr {
            Expr::Num(n) => Value::Num(n),
            Expr::Bool(b) => Value::Bool(b),
            Expr::Ident => self.underscore.clone(),
            Expr::Power(base, power) => {
                let base = self.eval(*base)?;
                let power = self.eval(*power)?;
                let (base, power) = check_nums!(base, power, "base and power must be numeric");
                Value::Num(base.powf(power))
            }
            Expr::Negate(num) => {
                let num = self.eval(*num)?;
                let num = check_nums!(num, "negation operand must be numeric");
                Value::Num(-num)
            }
            Expr::Add(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                let (lhs, rhs) = check_nums!(lhs, rhs, "addition operands must be numeric");
                Value::Num(lhs + rhs)
            }
            Expr::Sub(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                let (lhs, rhs) = check_nums!(lhs, rhs, "subtraction operands must be numeric");
                Value::Num(lhs - rhs)
            }
            Expr::Mul(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                let (lhs, rhs) = check_nums!(lhs, rhs, "multiply operands must be numeric");
                Value::Num(lhs * rhs)
            }
            Expr::Div(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                let (lhs, rhs) = check_nums!(lhs, rhs, "division operands must be numeric");
                if rhs == 0.0 {
                    return Err(format!("divide-by-zero"));
                }
                Value::Num(lhs / rhs)
            }
            Expr::Mod(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                let (lhs, rhs) = check_nums!(lhs, rhs, "modulo operands must be numeric");
                if rhs == 0.0 {
                    return Err(format!("divide-by-zero"));
                }
                Value::Num(lhs % rhs)
            }
            Expr::Relation(init, relations) => {
                let mut curr = self.eval(*init)?;
                let mut satisfy = true;
                for (rel_op, rel_expr) in relations {
                    let next = self.eval(rel_expr)?;
                    let rhs = next.clone();
                    match rel_op {
                        RelOp::Lt | RelOp::Gt | RelOp::LtEq | RelOp::GtEq => {
                            let (lhs, rhs) =
                                check_nums!(curr, rhs, "inequality operands must be numeric");
                            satisfy = rel_op.apply(lhs, rhs);
                            if !satisfy {
                                break;
                            }
                        }
                        RelOp::EqEq | RelOp::NotEq => {
                            let is_eq = match (curr.is_num(), rhs.is_num()) {
                                (true, true) => curr.as_num() == rhs.as_num(),
                                (false, false) => curr.as_bool() == rhs.as_bool(),
                                _ => false,
                            };
                            satisfy = if rel_op == RelOp::EqEq {
                                is_eq
                            } else {
                                !is_eq
                            };
                            if !satisfy {
                                break;
                            }
                        }
                    }
                    curr = next;
                }
                Value::Bool(satisfy)
            }
        })
    }
}
