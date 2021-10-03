use crate::ast::Expr;
use crate::lex::Lexer;
use crate::parse::Parser;

pub struct Aqvm {
    underscore: f64,
}

impl Aqvm {
    pub fn new() -> Self {
        Self { underscore: 0.0 }
    }

    pub fn run(&mut self, source: &str) {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);

        let expr = match parser.ast() {
            Ok(e) => e,
            Err(msg) => {
                eprintln!("[E] {}", msg);
                return;
            }
        };

        let value = match self.eval(expr) {
            Ok(v) => v,
            Err(msg) => {
                eprintln!("[E] {}", msg);
                return;
            }
        };

        self.underscore = value;
        println!("{}", value);
    }

    fn eval(&self, expr: Expr) -> Result<f64, String> {
        match expr {
            Expr::Num(num) => Ok(num),
            Expr::Ident => Ok(self.underscore),
            Expr::Power(base, power) => {
                let base = self.eval(*base)?;
                let power = self.eval(*power)?;
                Ok(base.powf(power))
            }
            Expr::Negate(num) => {
                let num = self.eval(*num)?;
                Ok(-num)
            }
            Expr::Add(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                Ok(lhs + rhs)
            }
            Expr::Sub(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                Ok(lhs - rhs)
            }
            Expr::Mul(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                Ok(lhs * rhs)
            }
            Expr::Div(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                if rhs == 0.0 {
                    return Err(format!("divide-by-zero"));
                }
                Ok(lhs / rhs)
            }
            Expr::Mod(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                if rhs == 0.0 {
                    return Err(format!("divide-by-zero"));
                }
                Ok(lhs % rhs)
            }
        }
    }
}
