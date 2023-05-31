use std::fmt;
use std::rc::Rc;

use crate::bytecode::Chunk;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i32),
    Real(f64),
    Str(String),
    Func(Rc<Function>),
    Native(Rc<FnNative>),
}

impl Value {
    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(..))
    }

    pub fn is_real(&self) -> bool {
        matches!(self, Self::Real(..))
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(..))
    }

    pub fn is_func(&self) -> bool {
        matches!(self, Self::Func(..))
    }

    pub fn is_native(&self) -> bool {
        matches!(self, Self::Native(..))
    }

    pub fn as_bool(self) -> bool {
        match self {
            Self::Bool(b) => b,
            _ => panic!(),
        }
    }

    pub fn as_int(self) -> i32 {
        match self {
            Self::Int(n) => n,
            _ => panic!(),
        }
    }

    pub fn as_real(self) -> f64 {
        match self {
            Self::Real(n) => n,
            _ => panic!(),
        }
    }

    pub fn as_str(self) -> String {
        match self {
            Self::Str(s) => s,
            _ => panic!(),
        }
    }

    pub fn as_func(self) -> Rc<Function> {
        match self {
            Self::Func(func) => func,
            _ => panic!(),
        }
    }

    pub fn as_native(self) -> Rc<FnNative> {
        match self {
            Self::Native(native) => native,
            _ => panic!(),
        }
    }

    pub fn is_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(b1), Self::Bool(b2)) => *b1 == *b2,
            (Self::Int(i1), Self::Int(i2)) => i1 == i2,
            (Self::Int(i), Self::Real(f)) => (*i as f64) == *f,
            (Self::Real(f), Self::Int(i)) => *f == (*i as f64),
            (Self::Real(f1), Self::Real(f2)) => *f1 == *f2,
            (Self::Str(s1), Self::Str(s2)) => *s1 == *s2,
            (Self::Func(fn1), Self::Func(fn2)) => Rc::ptr_eq(fn1, fn2),
            (Self::Native(n1), Self::Native(n2)) => Rc::ptr_eq(n1, n2),
            (Self::Unit, Self::Unit) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "(unit)"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Int(n) => write!(f, "{}", n),
            Self::Real(n) => write!(f, "{}", n),
            Self::Str(s) => write!(f, "{}", s),
            Self::Func(func) => fmt::Display::fmt(func, f),
            Self::Native(native) => fmt::Display::fmt(native, f),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
}

impl Function {
    pub fn new(name: &str, arity: usize, chunk: Chunk) -> Self {
        Self {
            name: name.to_owned(),
            arity,
            chunk,
        }
    }

    pub fn with_chunk(chunk: Chunk) -> Self {
        Self {
            name: String::new(),
            arity: 0,
            chunk,
        }
    }

    pub fn empty() -> Self {
        Self {
            name: String::new(),
            arity: 0,
            chunk: Chunk::new(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(fn|{})", self.name)
    }
}

pub type NativeFnPtr = fn(Vec<Value>) -> Value;

#[derive(Debug)]
pub struct FnNative {
    pub name: String,
    pub arity: usize,
    pub function: NativeFnPtr,
}

impl FnNative {
    pub fn new(name: &str, arity: usize, function: NativeFnPtr) -> Self {
        Self {
            name: name.to_owned(),
            arity,
            function,
        }
    }

    pub fn invoke(&self, args: Vec<Value>) -> Value {
        (self.function)(args)
    }
}

impl fmt::Display for FnNative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(fn-native|{})", self.name)
    }
}
