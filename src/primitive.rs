use token::{Kind, Token};
use ast::Node;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Eq;
use std::ops::{Add, Sub, Mul, Div, Rem, Not};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Str(String),
    Int(i32),
    Bool(bool),

    // Func(Vec<Node>, Node),
    Nil,
}


impl Type {
    pub fn from(token: &Token) -> Type {
        match token.clone() {
            Token {
                kind: Kind::Integer,
                value,
            } => Type::Int(value.parse::<i32>().expect("Invalid integer value.")),
            Token {
                kind: Kind::Bolean,
                value,
            } => Type::Bool(value == "true"),
            _ => Type::Nil,
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Type::Str(s) => format!("{}", s),
            Type::Int(s) => format!("{}", s),
            Type::Bool(s) => format!("{}", s),
            _ => String::new(),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self.clone() {
            Type::Int(s) => (s > 0),
            Type::Bool(s) => s,
            Type::Nil => false,
            _ => panic!("Value error: type {:?} cannot be used as boolean", self),
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Type) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Type) -> Ordering {
        match (self.clone(), other.clone()) {
            (Type::Bool(s), Type::Bool(o)) => s.cmp(&o),
            (Type::Int(s), Type::Int(o)) => s.cmp(&o),
            _ => panic!("Operation error: invalid comparison {:?} {:?}", self, other),
        }
    }
}

impl Eq for Type {}

impl Add for Type {
    type Output = Type;

    fn add(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s + o),
            _ => {
                panic!(
                    "Operation error: invalid add operation between
                       {:?} and {:?}",
                    self,
                    other
                )
            }
        }
    }
}

impl Sub for Type {
    type Output = Type;

    fn sub(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s - o),
            _ => {
                panic!(
                    "Operation error: invalid add operation between
                       {:?} and {:?}",
                    self,
                    other
                )
            }
        }
    }
}

impl Mul for Type {
    type Output = Type;

    fn mul(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s * o),
            _ => {
                panic!(
                    "Operation error: invalid add operation between
                       {:?} and {:?}",
                    self,
                    other
                )
            }
        }
    }
}

impl Div for Type {
    type Output = Type;

    fn div(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s / o),
            _ => {
                panic!(
                    "Operation error: invalid add operation between
                       {:?} and {:?}",
                    self,
                    other
                )
            }
        }
    }
}

impl Rem for Type {
    type Output = Type;

    fn rem(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s % o),
            _ => {
                panic!(
                    "Operation error: invalid add operation between
                       {:?} and {:?}",
                    self,
                    other
                )
            }
        }
    }
}

impl Not for Type {
    type Output = Type;

    fn not(self) -> Type {
        match self {
            Type::Bool(o) => Type::Bool(!o),
            Type::Str(o) => Type::Str(o),
            Type::Int(o) => Type::Int(o * -1),
            _ => panic!("Operation error: invalid not operation"),
        }
    }
}
