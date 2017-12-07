use token::Token;
use primitive::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Main(Node),
    Identifier(String),
    Operator(String, Vec<Node>),
    Comparison(String, Vec<Node>),
    Logical(String, Vec<Node>),
    Constant(Type),
    IfElse(Node, Vec<Node>),
    Assign(Node, Node),
    StdOut(Node),
    Empty,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub operation: Box<Operation>,
    pub value: String,
}

impl Node {
    pub fn main(statements: Node) -> Self {
        Node {
            operation: Box::new(Operation::Main(statements)),
            value: String::new(),
        }
    }
    pub fn operator(token: String, nodes: Vec<Node>) -> Self {
        Node {
            operation: Box::new(Operation::Operator(token.clone(), nodes)),
            value: token,
        }
    }
    pub fn comparison(token: String, nodes: Vec<Node>) -> Self {
        Node {
            operation: Box::new(Operation::Comparison(token.clone(), nodes)),
            value: token,
        }
    }
    pub fn logical(token: String, nodes: Vec<Node>) -> Self {
        Node {
            operation: Box::new(Operation::Logical(token.clone(), nodes)),
            value: token,
        }
    }
    pub fn constant(token: Token) -> Self {
        let primitive = Type::from(&token);
        Node {
            operation: Box::new(Operation::Constant(primitive)),
            value: token.value,
        }
    }
    pub fn indentifier(token: Token) -> Self {
        Node {
            operation: Box::new(Operation::Identifier(token.clone().value)),
            value: token.value,
        }
    }
    pub fn stdout(node: Node) -> Self {
        Node {
            operation: Box::new(Operation::StdOut(node)),
            value: String::new(),
        }
    }
    pub fn ifelse(condition: Node, nodes: Vec<Node>) -> Self {
        Node {
            operation: Box::new(Operation::IfElse(condition, nodes)),
            value: String::new(),
        }
    }
    pub fn assign(name: Node, node: Node) -> Self {
        Node {
            operation: Box::new(Operation::Assign(name, node)),
            value: String::new(),
        }
    }
    pub fn empty() -> Self {
        Node {
            operation: Box::new(Operation::Empty),
            value: String::new(),
        }
    }
}
