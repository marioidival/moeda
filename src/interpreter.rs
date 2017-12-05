use ast::{Node, Operation};
use primitive::Type;
use frame::{Frame, FrameStack};


pub struct Interpreter {
    pub stack: FrameStack,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter { stack: FrameStack::new() }
    }

    fn scope(&mut self) -> &mut Frame {
        self.stack.current()
    }

    pub fn eval(&mut self, tree: Node) -> String {
        match self.eval_tree(tree) {
            Ok(result) => result.to_string(),
            Err(error) => error,
        }
    }

    pub fn eval_tree(&mut self, tree: Node) -> Result<Type, String> {
        let Node { operation, .. } = tree;
        match *operation.clone() {
            Operation::Main(statements) => {
                let mut last_stm_return = Ok(Type::Nil);
                last_stm_return = self.eval_tree(statements);

                last_stm_return
            }
            Operation::Logical(tok, statements) => {
                let types_vec: Vec<Type> = statements
                    .into_iter()
                    .map(|stm| self.eval_tree(stm).unwrap())
                    .collect();
                exec_logical(tok, types_vec)
            }
            Operation::Operator(tok, statements) => {
                let types_vec: Vec<Type> = statements
                    .into_iter()
                    .map(|stm| self.eval_tree(stm).unwrap())
                    .collect();
                exec_operator(tok, types_vec)
            }
            Operation::Comparison(tok, statements) => {
                let types_vec: Vec<Type> = statements
                    .into_iter()
                    .map(|stm| self.eval_tree(stm).unwrap())
                    .collect();
                exec_comparison(tok, types_vec)
            }

            Operation::Constant(var) => Ok(var),
            _ => Ok(Type::Nil),
        }
    }
}

fn exec_operator(tok: String, nodes: Vec<Type>) -> Result<Type, String> {
    let node_clone = nodes.clone();
    match tok.as_ref() {
        "+" => Ok(nodes.into_iter().fold(Type::Int(0), |acc, x| acc + x)),
        "-" => {
            Ok(nodes.into_iter().skip(1).fold(
                node_clone.into_iter().nth(0).unwrap(),
                |acc, x| acc - x,
            ))
        }
        "*" => {
            Ok(nodes.into_iter().skip(1).fold(
                node_clone.into_iter().nth(0).unwrap(),
                |acc, x| acc * x,
            ))
        }
        "/" => {
            Ok(nodes.into_iter().skip(1).fold(
                node_clone.into_iter().nth(0).unwrap(),
                |acc, x| acc / x,
            ))
        }
        _ => Err(format!("Operator error: {} isn't operation token", tok)),
    }
}

// TODO: ">" | "<" | "<=" | ">=" | "max" | "min"
fn exec_comparison(tok: String, nodes: Vec<Type>) -> Result<Type, String> {
    let node_clone = nodes.clone();
    match tok.as_ref() {
        "=" => {
            let b = nodes.iter().take_while(
                |x| *x == node_clone.iter().last().unwrap(),
            );
            Ok(Type::Bool(b.count() == nodes.len()))
        }
        "/=" => {
            let b = nodes.iter().take_while(
                |x| *x == node_clone.iter().last().unwrap(),
            );
            Ok(Type::Bool(b.count() != nodes.len()))
        }
        _ => Err(format!("Comparison error: {} isn't comparison token", tok)),
    }
}


fn exec_logical(tok: String, nodes: Vec<Type>) -> Result<Type, String> {
    let node_clone = nodes.clone();
    match tok.as_ref() {
        "not" => Ok(!nodes.into_iter().nth(0).unwrap()),
        "and" => {
            let result = nodes.into_iter().skip(1).fold(
                node_clone.into_iter().nth(0).unwrap(),
                |acc, x| if !acc.as_bool() && x.as_bool() {
                    acc
                } else {
                    x
                },
            );
            Ok(result)
        }
        "or" => {
            let result = nodes.into_iter().skip(1).fold(
                node_clone.into_iter().nth(0).unwrap(),
                |acc, x| if acc.as_bool() || !x.as_bool() {
                    acc
                } else {
                    x
                },
            );
            Ok(result)
        }
        _ => Err(format!("Logicial error: {} isn't logical token", tok)),
    }
}
