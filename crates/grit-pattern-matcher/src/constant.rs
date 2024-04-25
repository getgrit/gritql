use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Constant {
    Boolean(bool),
    String(String),
    Integer(i64),
    Float(f64),
    Undefined,
}

impl Constant {
    pub fn is_truthy(&self) -> bool {
        match self {
            Constant::Integer(i) => *i != 0,
            Constant::Float(d) => *d != 0.0,
            Constant::Boolean(b) => *b,
            Constant::String(s) => !s.is_empty(),
            Constant::Undefined => false,
        }
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Boolean(b) => write!(f, "{}", b),
            Constant::String(s) => write!(f, "{}", s),
            Constant::Integer(n) => write!(f, "{}", n),
            Constant::Float(n) => write!(f, "{}", n),
            Constant::Undefined => write!(f, ""),
        }
    }
}

impl PartialEq for Constant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Constant::Boolean(b1), Constant::Boolean(b2)) => b1 == b2,
            (Constant::String(s1), Constant::String(s2)) => s1 == s2,
            (Constant::Integer(n1), Constant::Integer(n2)) => n1 == n2,
            (Constant::Float(n1), Constant::Float(n2)) => n1 == n2,
            _ => false,
        }
    }
}
