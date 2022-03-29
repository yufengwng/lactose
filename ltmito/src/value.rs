use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i32),
    Real(f64),
    Str(String),
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

    pub fn is_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => *b1 == *b2,
            (Value::Int(i1), Value::Int(i2)) => i1 == i2,
            (Value::Int(i), Value::Real(f)) => (*i as f64) == *f,
            (Value::Real(f), Value::Int(i)) => *f == (*i as f64),
            (Value::Real(f1), Value::Real(f2)) => *f1 == *f2,
            (Value::Str(s1), Value::Str(s2)) => *s1 == *s2,
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
        }
    }
}
