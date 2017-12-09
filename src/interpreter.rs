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

    pub fn eval(&mut self, tree: Node) -> String {
        match self.eval_tree(tree) {
            Ok(result) => result.to_string(),
            Err(error) => error,
        }
    }

    fn scope(&mut self) -> &mut Frame {
        self.stack.current()
    }

    pub fn eval_tree(&mut self, tree: Node) -> Result<Type, String> {
        let Node { operation, .. } = tree;
        match *operation.clone() {
            Operation::Main(statements) => self.eval_tree(statements),
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
            Operation::When(condition, body) => {
                let result_condition = try!(self.eval_tree(condition));
                if result_condition.as_bool() {
                    Ok(
                        body.into_iter()
                            .filter(|stm| *stm.operation != Operation::Empty)
                            .map(|stm| self.eval_tree(stm).unwrap())
                            .last()
                            .unwrap(),
                    )
                } else {
                    Ok(Type::Nil)
                }
            }
            Operation::IfElse(condition, nodes) => {
                let result_condition = try!(self.eval_tree(condition));
                if result_condition.as_bool() {
                    self.eval_tree(nodes.into_iter().nth(0).unwrap())
                } else {
                    self.eval_tree(nodes.into_iter().nth(1).unwrap())
                }
            }
            Operation::Assign(name, nodes) => {
                let var_name = name.value;
                let value = try!(self.eval_tree(nodes));

                if self.scope().has(&*var_name) {
                    return Err(format!(
                        "Value error: variable {} has already defined.",
                        var_name
                    ));
                }

                self.scope().ilocals.insert(var_name, value.clone());
                Ok(Type::Nil)
            }
            Operation::Identifier(name) => {
                if let Some(value) = self.scope().get(&*name) {
                    Ok(value)
                } else {
                    Err(format!("Variable {} doesn't exist in this context", name))
                }
            }
            Operation::StdOut(stm) => {
                let result = try!(self.eval_tree(stm));
                println!("{}", result.to_string());
                Ok(Type::Nil)
            }
            Operation::DefineFunction(name, func) => {
                let var_name = name.value;

                if self.scope().has(&*var_name) {
                    return Err(format!(
                        "Value error: variable {} has already defined.",
                        var_name
                    ));
                }
                self.scope().ilocals.insert(var_name, func);
                Ok(Type::Nil)
            }
            Operation::CallFunction(name, params) => {
                let func_frame = self.scope().clone();
                self.stack.push(func_frame);

                let var_name = name.value;
                if let Some(Type::Func(fparams, block)) = self.scope().get(&*var_name).clone() {
                    for (pname, pvalue) in fparams.iter().zip(params.iter()) {
                        let value = try!(self.eval_tree(pvalue.clone()));
                        self.scope().locals.insert(pname.clone().value, value);
                    }
                    Ok(
                        block
                            .into_iter()
                            .filter(|stm| *stm.operation != Operation::Empty)
                            .map(|stm| self.eval_tree(stm).unwrap())
                            .last()
                            .unwrap(),
                    )
                } else {
                    return Err(format!("Value error: {} is not callable", var_name));
                }
            }
            Operation::Constant(var) => Ok(var),
            _ => Ok(Type::Nil),
        }
    }
}

// TODO: "%", "incf", "decf"
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
        ">" => {
            let result = nodes.into_iter().zip(node_clone.into_iter().skip(1)).all(
                |b| {
                    (b.0 > b.1) == true
                },
            );
            Ok(Type::Bool(result))
        }
        "<" => {
            let result = nodes.into_iter().zip(node_clone.into_iter().skip(1)).all(
                |b| {
                    (b.0 < b.1) == true
                },
            );
            Ok(Type::Bool(result))
        }
        ">=" => {
            let result = nodes.into_iter().zip(node_clone.into_iter().skip(1)).all(
                |b| {
                    (b.0 >= b.1) == true
                },
            );
            Ok(Type::Bool(result))
        }
        "<=" => {
            let result = nodes.into_iter().zip(node_clone.into_iter().skip(1)).all(
                |b| {
                    (b.0 <= b.1) == true
                },
            );
            Ok(Type::Bool(result))
        }
        "max" => Ok(nodes.into_iter().max().unwrap()),
        "min" => Ok(nodes.into_iter().min().unwrap()),
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



#[cfg(test)]
mod operator {
    use super::*;

    #[test]
    fn test_exec_operator_plus() {
        let values = vec![Type::Int(2), Type::Int(4)];
        assert_eq!(Ok(Type::Int(6)), exec_operator(String::from("+"), values))
    }

    #[test]
    fn test_exec_operator_minus() {
        let values = vec![Type::Int(2), Type::Int(4)];
        assert_eq!(Ok(Type::Int(-2)), exec_operator(String::from("-"), values))
    }

    #[test]
    fn test_exec_operator_mul() {
        let values = vec![Type::Int(2), Type::Int(4)];
        assert_eq!(Ok(Type::Int(8)), exec_operator(String::from("*"), values))
    }

    #[test]
    fn test_exec_operator_div() {
        let values = vec![Type::Int(6), Type::Int(2)];
        assert_eq!(Ok(Type::Int(3)), exec_operator(String::from("/"), values))
    }
}

#[cfg(test)]
mod comparison {
    use super::*;

    #[test]
    fn test_exec_comparison_eq() {
        let values = vec![Type::Int(6), Type::Int(2)];
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_comparison(String::from("="), values)
        )
    }

    #[test]
    fn test_exec_comparison_different() {
        let values = vec![Type::Int(6), Type::Int(2)];
        assert_eq!(
            Ok(Type::Bool(true)),
            exec_comparison(String::from("/="), values)
        )
    }

    #[test]
    fn test_exec_comparison_gt() {
        let values = vec![Type::Int(6), Type::Int(2)];
        assert_eq!(
            Ok(Type::Bool(true)),
            exec_comparison(String::from(">"), values)
        )
    }

    #[test]
    fn test_exec_comparison_lt() {
        let values = vec![Type::Int(6), Type::Int(2)];
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_comparison(String::from("<"), values)
        )
    }

    #[test]
    fn test_exec_comparison_gte() {
        let values = vec![Type::Int(6), Type::Int(2)];
        assert_eq!(
            Ok(Type::Bool(true)),
            exec_comparison(String::from(">="), values)
        )
    }

    #[test]
    fn test_exec_comparison_lte() {
        let values = vec![Type::Int(6), Type::Int(2)];
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_comparison(String::from("<="), values)
        )
    }

    #[test]
    fn test_exec_comparison_max() {
        let values = vec![Type::Int(6), Type::Int(2), Type::Int(55)];
        assert_eq!(
            Ok(Type::Int(55)),
            exec_comparison(String::from("max"), values)
        )
    }

    #[test]
    fn test_exec_comparison_min() {
        let values = vec![Type::Int(6), Type::Int(2), Type::Int(55)];
        assert_eq!(
            Ok(Type::Int(2)),
            exec_comparison(String::from("min"), values)
        )
    }
}

#[cfg(test)]
mod logical {
    use super::*;

    #[test]
    fn test_exec_logical_and() {
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_logical(
                String::from("and"),
                vec![Type::Bool(false), Type::Bool(true)],
            )
        );
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_logical(
                String::from("and"),
                vec![Type::Bool(true), Type::Bool(false)],
            )
        );
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_logical(
                String::from("and"),
                vec![Type::Bool(false), Type::Bool(false)],
            )
        );
        assert_eq!(
            Ok(Type::Bool(true)),
            exec_logical(
                String::from("and"),
                vec![Type::Bool(true), Type::Bool(true)],
            )
        )
    }

    #[test]
    fn test_exec_logical_or() {
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_logical(
                String::from("or"),
                vec![Type::Bool(false), Type::Bool(false)],
            )
        );
        assert_eq!(
            Ok(Type::Bool(true)),
            exec_logical(
                String::from("or"),
                vec![Type::Bool(true), Type::Bool(false)],
            )
        );
        assert_eq!(
            Ok(Type::Bool(true)),
            exec_logical(
                String::from("or"),
                vec![Type::Bool(false), Type::Bool(true)],
            )
        );
        assert_eq!(
            Ok(Type::Bool(true)),
            exec_logical(String::from("or"), vec![Type::Bool(true), Type::Bool(true)])
        )
    }

    #[test]
    fn test_exec_logical_not() {
        let values = vec![Type::Bool(true)];
        assert_eq!(
            Ok(Type::Bool(false)),
            exec_logical(String::from("not"), values)
        )
    }
}
