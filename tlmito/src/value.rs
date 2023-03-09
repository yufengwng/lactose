use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i32),
    Real(f64),
    Str(String),
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

    pub fn as_native(self) -> Rc<FnNative> {
        match self {
            Self::Native(native) => native,
            _ => panic!(),
        }
    }

    pub fn is_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => *b1 == *b2,
            (Value::Int(i1), Value::Int(i2)) => i1 == i2,
            (Value::Int(i), Value::Real(f)) => (*i as f64) == *f,
            (Value::Real(f), Value::Int(i)) => *f == (*i as f64),
            (Value::Real(f1), Value::Real(f2)) => *f1 == *f2,
            (Value::Str(s1), Value::Str(s2)) => *s1 == *s2,
            (Value::Native(n1), Value::Native(n2)) => Rc::ptr_eq(n1, n2),
            (Value::Unit, Value::Unit) => true,
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
            Self::Native(native) => fmt::Display::fmt(native, f),
        }
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
        write!(f, "(native {})", self.name)
    }
}
