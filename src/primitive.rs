use token::{Kind, Token};
use ast::Node;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Eq;
use std::ops::{Add, Sub, Mul, Div, Rem, Not};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Str(String),
    Int(i64),
    Bool(bool),
    Func(Vec<Node>, Vec<Node>),
    List(Vec<Type>),

    Nil,
}


impl Type {
    pub fn from(token: &Token) -> Type {
        match token.clone() {
            Token {
                kind: Kind::List,
                value,
            } => {
                let v: Vec<&str> = value.as_str().split(',').collect();
                let tokens: Vec<Token> = v.into_iter()
                    .map(|t| if let Some(result) = Kind::reserved(&String::from(t)) {
                        Token::build(result, String::from(t))
                    } else {
                        let kind = Kind::classify(&t.chars().nth(0));
                        Token::build(kind, String::from(t))
                    })
                    .collect();

                let types: Vec<Type> = tokens.iter().map(|t| Type::from(t)).collect();
                Type::List(types)
            }
            Token {
                kind: Kind::Integer,
                value,
            } => Type::Int(value.parse::<i64>().expect("Invalid integer value.")),
            Token {
                kind: Kind::Bolean,
                value,
            } => Type::Bool(value == "true"),
            Token {
                kind: Kind::Str,
                value,
            } => Type::Str(String::from(value)),
            _ => Type::Nil,
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Type::Str(s) => format!("{}", s),
            Type::Int(s) => format!("{}", s),
            Type::Bool(s) => format!("{}", s),
            Type::List(s) => {
                let i: Vec<String> = s.into_iter().map(|t| t.to_string()).collect();
                let items = i.join(" ");
                format!("({})", items)
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_int_to_string() {
        let type_ = Type::Int(1);
        assert_eq!(String::from("1"), type_.to_string())
    }

    #[test]
    fn test_type_bool_to_string() {
        let type_ = Type::Bool(true);
        assert_eq!(String::from("true"), type_.to_string())
    }

    #[test]
    fn test_type_nil_to_string() {
        let type_ = Type::Nil;
        assert_eq!(String::from(""), type_.to_string())
    }

    #[test]
    fn test_type_int_as_bool() {
        let type_ = Type::Int(1);
        assert_eq!(true, type_.as_bool())
    }

    #[test]
    fn test_type_nil_as_bool() {
        let type_ = Type::Nil;
        assert_eq!(false, type_.as_bool())
    }
}
