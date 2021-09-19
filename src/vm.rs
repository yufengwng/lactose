use crate::lex::Lexer;
use crate::parse::Expr;
use crate::parse::Parser;

pub struct Aqvm;

impl Aqvm {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self, source: &str) {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse();
        match self.eval(expr) {
            Ok(n) => println!("{}", n),
            Err(e) => eprintln!("[E] {}", e),
        }
    }

    fn eval(&self, expr: Expr) -> Result<f64, String> {
        match expr {
            Expr::Err(msg) => Err(msg),
            Expr::Num(n) => Ok(n),
            Expr::Group(inner) => self.eval(*inner),
            Expr::Power(base, nth) => {
                let base = self.eval(*base)?;
                let nth = self.eval(*nth)?;
                Ok(base.powf(nth))
            }
            Expr::Negate(num) => {
                let num = self.eval(*num)?;
                Ok(-num)
            }
            Expr::Mul(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                Ok(lhs * rhs)
            }
            Expr::Div(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                Ok(lhs / rhs)
            }
            Expr::Mod(lhs, rhs) => {
                let lhs = self.eval(*lhs)?;
                let rhs = self.eval(*rhs)?;
                Ok(lhs % rhs)
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
        }
    }
}
