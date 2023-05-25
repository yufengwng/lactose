use std::collections::HashMap;
use std::rc::Rc;

use tflang::parse::Parser;

use crate::cgen::CodeGen;
use crate::code::Chunk;
use crate::code::OpCode;
use crate::code::OpCode::*;
use crate::value::FnNative;
use crate::value::Value;

pub enum MitoRes {
    Ok(Value),
    CompileErr(String),
    RuntimeErr(String),
}

pub struct MitoEnv {
    vals: HashMap<String, Value>,
}

impl MitoEnv {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
        }
    }

    pub fn with_builtins() -> Self {
        let mut env = Self::new();
        let native = FnNative::new("println", 1, native_println);
        env.set("println", Value::Native(Rc::new(native)));
        env
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.vals.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        return self.vals.get(name).map(|val| val.clone());
    }
}

pub struct MitoVM {
    stack: Vec<Value>,
    ip: usize,
}

impl MitoVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn run(&mut self, env: &mut MitoEnv, source: &str) -> MitoRes {
        let ast = match Parser::new(source).ast() {
            Ok(ls) => ls,
            Err(msg) => return MitoRes::CompileErr(msg),
        };
        let chunk = match CodeGen::new().compile(&ast) {
            Ok(ch) => ch,
            Err(msg) => return MitoRes::CompileErr(msg),
        };
        self.execute(env, &chunk)
    }

    fn execute(&mut self, env: &mut MitoEnv, chunk: &Chunk) -> MitoRes {
        self.ip = 0;
        while self.ip < chunk.len() {
            let byte = chunk.code(self.ip);
            self.ip += 1;
            let opcode = byte.try_into().unwrap();
            self.dispatch(env, chunk, opcode);
        }
        let res = if !self.stack.is_empty() {
            self.stack.pop().unwrap()
        } else {
            Value::Unit
        };
        MitoRes::Ok(res)
    }

    fn dispatch(&mut self, env: &mut MitoEnv, chunk: &Chunk, opcode: OpCode) {
        match opcode {
            OpNop => return,
            OpUnit => self.stack.push(Value::Unit),
            OpTrue => self.stack.push(Value::Bool(true)),
            OpFalse => self.stack.push(Value::Bool(false)),
            OpConst => {
                let idx = chunk.code(self.ip) as usize;
                self.ip += 1;
                let val = chunk.value(idx);
                self.stack.push(val);
            }
            OpAdd => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs + rhs;
                self.stack.push(Value::Real(val));
            }
            OpSub => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs - rhs;
                self.stack.push(Value::Real(val));
            }
            OpMul => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs * rhs;
                self.stack.push(Value::Real(val));
            }
            OpDiv => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs / rhs;
                self.stack.push(Value::Real(val));
            }
            OpRem => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs % rhs;
                self.stack.push(Value::Real(val));
            }
            OpPow => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs.powf(rhs);
                self.stack.push(Value::Real(val));
            }
            OpNeg => {
                let val = self.pop_as_float();
                self.stack.push(Value::Real(-val));
            }
            OpLt => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs < rhs;
                self.stack.push(Value::Bool(val));
            }
            OpGt => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs > rhs;
                self.stack.push(Value::Bool(val));
            }
            OpLtEq => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs <= rhs;
                self.stack.push(Value::Bool(val));
            }
            OpGtEq => {
                let rhs = self.pop_as_float();
                let lhs = self.pop_as_float();
                let val = lhs >= rhs;
                self.stack.push(Value::Bool(val));
            }
            OpEqual => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                let is_eq = lhs.is_eq(&rhs);
                self.stack.push(Value::Bool(is_eq));
            }
            OpNotEq => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                let is_eq = lhs.is_eq(&rhs);
                self.stack.push(Value::Bool(!is_eq));
            }
            OpLoop => todo!(),
            OpJump => todo!(),
            OpBranch => todo!(),
            OpGet => {
                let idx = chunk.code(self.ip) as usize;
                self.ip += 1;
                let name = chunk.value(idx).as_str();
                let val = env.get(&name).unwrap();
                self.stack.push(val);
            }
            OpCall => {
                let count = chunk.code(self.ip) as usize;
                self.ip += 1;
                let idx = self.stack.len() - count - 1;
                let callee = self.stack[idx].clone();
                self.dispatch_call(callee, count);
            }
            OpPop => {
                self.stack.pop();
            }
        }
    }

    fn dispatch_call(&mut self, callee: Value, count: usize) {
        match callee {
            Value::Native(_) => self.call_native(callee.as_native(), count),
            _ => panic!(),
        }
    }

    fn call_native(&mut self, native: Rc<FnNative>, count: usize) {
        if native.arity != count {
            eprintln!("expected {} arguments but got {}", native.arity, count);
            return;
        }
        let idx = self.stack.len() - count;
        let args = self.stack.split_off(idx);
        let result = native.invoke(args);
        self.stack.pop();
        self.stack.push(result);
    }

    fn pop_as_float(&mut self) -> f64 {
        let val = self.stack.pop().unwrap();
        if val.is_int() {
            val.as_int() as f64
        } else {
            val.as_real()
        }
    }
}

fn native_println(args: Vec<Value>) -> Value {
    println!("{}", args[0]);
    Value::Unit
}
