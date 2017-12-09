use token::{Token, Kind, Tokenizer};
use ast;

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(lexer: Tokenizer) -> Self {
        Parser { tokenizer: lexer }
    }

    fn statements(&mut self) -> ast::Node {
        match self.tokenizer.advance().get() {
            Some(Token { kind: Kind::GroupBegin, .. }) => {
                self.tokenizer.consume(Kind::GroupBegin);

                match self.tokenizer.advance().get() {
                    Some(Token { kind: Kind::Operator, .. }) => {
                        let tok_operator = self.tokenizer.advance().consume(Kind::Operator);
                        let nodes = self.args_list();
                        self.tokenizer.consume(Kind::GroupEnd);
                        ast::Node::operator(tok_operator.value, nodes)
                    }
                    Some(Token { kind: Kind::Logical, .. }) => {
                        let tok = self.tokenizer.advance().consume(Kind::Logical);
                        let nodes = self.args_list();
                        self.tokenizer.consume(Kind::GroupEnd);
                        ast::Node::logical(tok.value, nodes)
                    }
                    Some(Token { kind: Kind::StdOut, .. }) => {
                        self.tokenizer.consume(Kind::StdOut);
                        ast::Node::stdout(self.statements())
                    }
                    Some(Token { kind: Kind::If, .. }) => {
                        self.tokenizer.consume(Kind::If);
                        let condition = self.factor();
                        let lnode = self.factor();
                        let rnode = self.factor();
                        ast::Node::ifelse(condition, vec![lnode, rnode])
                    }
                    Some(Token { kind: Kind::When, .. }) => {
                        self.tokenizer.consume(Kind::When);
                        let condition = self.factor();
                        let mut body: Vec<ast::Node> = vec![];
                        body.push(self.statements());

                        while self.tokenizer.current() != None {
                            body.push(self.statements());
                        }
                        ast::Node::when(condition, body)
                    }
                    Some(Token { kind: Kind::VarDefine, .. }) => {
                        self.tokenizer.consume(Kind::VarDefine);
                        let var = self.def();
                        let node = self.statements();
                        ast::Node::assign(var, node)
                    }
                    Some(Token { kind: Kind::FnDefine, .. }) => self.define_function(),
                    Some(Token { kind: Kind::ID, .. }) => self.function_call(),
                    Some(Token { kind: Kind::Comparison, .. }) => {
                        let tok = self.tokenizer.advance().consume(Kind::Comparison);
                        let nodes = self.args_list();
                        self.tokenizer.consume(Kind::GroupEnd);
                        ast::Node::comparison(tok.value, nodes)
                    }
                    _ => self.statements(),
                }
            }
            Some(Token { kind: Kind::Str, .. }) |
            Some(Token { kind: Kind::Integer, .. }) |
            Some(Token { kind: Kind::List, .. }) |
            Some(Token { kind: Kind::Bolean, .. }) => self.factor(),
            Some(Token { kind: Kind::ID, .. }) => {
                let id = self.def();
                self.tokenizer.advance().consume(Kind::GroupEnd);
                id
            }

            Some(Token { kind: Kind::GroupEnd, .. }) => {
                self.tokenizer.consume(Kind::GroupEnd);
                ast::Node::empty()
            }
            _ => ast::Node::empty(),
        }
    }

    fn define_function(&mut self) -> ast::Node {
        self.tokenizer.consume(Kind::FnDefine);
        let name = self.def();

        self.tokenizer.advance().consume(Kind::ArgsBegin);
        let params = self.params_list();
        self.tokenizer.consume(Kind::ArgsEnd);
        self.tokenizer.advance();
        let mut body: Vec<ast::Node> = vec![];

        while self.tokenizer.current() != None {
            body.push(self.statements());
        }
        ast::Node::function_define(name, params, body)
    }

    fn function_call(&mut self) -> ast::Node {
        let name = self.def();
        let args = self.args_list();
        ast::Node::function_call(name, args)
    }

    fn factor(&mut self) -> ast::Node {
        match self.tokenizer.advance().get() {
            Some(Token { kind: Kind::GroupBegin, .. }) |
            Some(Token { kind: Kind::ArgsBegin, .. }) => self.statements(),
            Some(Token { kind: Kind::Bolean, .. }) => ast::Node::constant(
                self.tokenizer.advance().consume(
                    Kind::Bolean,
                ),
            ),
            Some(Token { kind: Kind::Integer, .. }) => {
                ast::Node::constant(self.tokenizer.advance().consume(Kind::Integer))
            }
            Some(Token { kind: Kind::Str, .. }) => {
                ast::Node::constant(self.tokenizer.advance().consume(Kind::Str))
            }
            Some(Token { kind: Kind::List, .. }) => {
                ast::Node::constant(self.tokenizer.advance().consume(Kind::List))
            }
            Some(Token { kind: Kind::ID, .. }) => self.def(),
            Some(Token { kind: Kind::GroupEnd, .. }) => {
                self.tokenizer.consume(Kind::GroupEnd);
                self.statements()
            }
            _ => ast::Node::empty(),
        }
    }

    fn args_list(&mut self) -> Vec<ast::Node> {
        let mut args = vec![];

        match self.tokenizer.advance().get() {
            Some(Token { kind: Kind::GroupEnd, .. }) => return args,
            _ => args.push(self.factor()),
        }
        args.extend(self.args_list());
        args
    }

    fn params_list(&mut self) -> Vec<ast::Node> {
        let mut args = vec![];

        match self.tokenizer.advance().get() {
            Some(Token { kind: Kind::ArgsEnd, .. }) => return args,
            _ => args.push(self.factor()),
        }
        args.extend(self.params_list());
        args
    }

    fn def(&mut self) -> ast::Node {
        let token = self.tokenizer.advance().consume(Kind::ID);
        ast::Node::indentifier(token)
    }

    pub fn parse(&mut self) -> ast::Node {
        ast::Node::main(self.statements())
    }
}

#[allow(dead_code)]
fn build_node_operator(operator: String, nodes: Vec<ast::Node>) -> ast::Node {
    ast::Node::operator(operator, nodes)
}

#[allow(dead_code)]
fn build_node_comparision(tok_value: String, nodes: Vec<ast::Node>) -> ast::Node {
    ast::Node::comparison(tok_value, nodes)
}

#[allow(dead_code)]
fn build_node_logical(tok_value: String, nodes: Vec<ast::Node>) -> ast::Node {
    ast::Node::logical(tok_value, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_sum_as_node() {
        let text = "(+ 1 9 7)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("7"),
            }),
        ];

        let expected = build_node_operator(String::from("+"), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_expr_sub_as_node() {
        let text = "(- 1 9 7)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("7"),
            }),
        ];

        let expected = build_node_operator(String::from("-"), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_expr_mul_as_node() {
        let text = "(* 1 9 7)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("7"),
            }),
        ];

        let expected = build_node_operator(String::from("*"), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_expr_div_as_node() {
        let text = "(/ 1 9 7)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("7"),
            }),
        ];

        let expected = build_node_operator(String::from("/"), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_expr_mod_as_node() {
        let text = "(% 9 7)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("7"),
            }),
        ];

        let expected = build_node_operator(String::from("%"), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_expr_sum_complex_as_node() {
        let text = "(+ 9 (- 10 7))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let nodes_sub = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("10"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("7"),
            }),
        ];
        let sub = build_node_operator(String::from("-"), nodes_sub);
        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }),
            sub,
        ];

        let expected = build_node_operator(String::from("+"), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_statement_with_comparision_as_node() {
        let text = "(= 1 1)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
        ];

        let expected = build_node_comparision(String::from("="), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_statement_with_complex_comparision_as_node() {
        let text = "(= 1 (* 1 5))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let node_mul = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("5"),
            }),
        ];
        let mul = build_node_operator(String::from("*"), node_mul);

        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            mul,
        ];

        let expected = build_node_comparision(String::from("="), nodes);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_logical_not_as_node() {
        let text = "(not true)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let node = vec![
            ast::Node::constant(Token {
                kind: Kind::Bolean,
                value: String::from("true"),
            }),
        ];
        let expected = build_node_logical(String::from("not"), node);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_logical_and_as_node() {
        let text = "(and 1 5)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let node = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("5"),
            }),
        ];
        let expected = build_node_logical(String::from("and"), node);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_logical_or_as_node() {
        let text = "(or 1 5)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let node = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("5"),
            }),
        ];
        let expected = build_node_logical(String::from("or"), node);
        assert_eq!(expected, parser.statements())
    }

    #[test]
    fn test_stdout_as_node() {
        let text = "(print (+ 1 1))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let nodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
        ];

        let sum_node = build_node_operator(String::from("+"), nodes);
        assert_eq!(ast::Node::stdout(sum_node), parser.statements())
    }

    #[test]
    fn test_stdout_with_str_as_node() {
        let text = "(print \"ola\")";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let nodes = ast::Node::constant(Token {
            kind: Kind::Str,
            value: String::from("ola"),
        });

        assert_eq!(ast::Node::stdout(nodes), parser.statements())
    }

    #[test]
    fn test_stdout_with_int_as_node() {
        let text = "(print 1)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let nodes = ast::Node::constant(Token {
            kind: Kind::Integer,
            value: String::from("1"),
        });

        assert_eq!(ast::Node::stdout(nodes), parser.statements())
    }

    #[test]
    fn test_stdout_with_bool_as_node() {
        let text = "(print true)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let nodes = ast::Node::constant(Token {
            kind: Kind::Bolean,
            value: String::from("true"),
        });

        assert_eq!(ast::Node::stdout(nodes), parser.statements())
    }

    #[test]
    fn test_stdout_with_empty_as_node() {
        let text = "(print )";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!(ast::Node::stdout(ast::Node::empty()), parser.statements())
    }

    #[test]
    fn test_stdout_with_list_as_node() {
        let text = "(print '(1 2 true))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let nodes = ast::Node::constant(Token {
            kind: Kind::List,
            value: String::from("1,2,true"),
        });

        assert_eq!(ast::Node::stdout(nodes), parser.statements())
    }

    #[test]
    fn test_if_as_node() {
        let text = "(if (= 1 1) (print (+ 1 1)) (print (- 1 1)))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let condition_node = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
        ];
        let anodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
        ];
        let snodes = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
        ];

        let add_node = ast::Node::stdout(build_node_operator(String::from("+"), anodes));
        let sub_node = ast::Node::stdout(build_node_operator(String::from("-"), snodes));
        let condition_node = build_node_comparision(String::from("="), condition_node);
        assert_eq!(
            ast::Node::ifelse(condition_node, vec![add_node, sub_node]),
            parser.statements()
        )
    }

    #[test]
    fn test_when_as_node() {
        let text = "(when (= 1 1) (print \"eq\"))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let condition_node = vec![
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
            ast::Node::constant(Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }),
        ];

        let condition_node = build_node_comparision(String::from("="), condition_node);
        let stdout = ast::Node::stdout(ast::Node::constant(Token {
            kind: Kind::Str,
            value: String::from("eq"),
        }));
        assert_eq!(
            ast::Node::when(
                condition_node,
                vec![stdout, ast::Node::empty(), ast::Node::empty()],
            ),
            parser.statements()
        )
    }

    #[test]
    fn test_assign_as_node() {
        let text = "(def x 1)";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!(
            ast::Node::assign(
                ast::Node::indentifier(Token {
                    kind: Kind::ID,
                    value: String::from("x"),
                }),
                ast::Node::constant(Token {
                    kind: Kind::Integer,
                    value: String::from("1"),
                }),
            ),
            parser.statements()
        )
    }

    #[test]
    fn test_define_function() {
        let text = "(defn hello [name] (print name))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!(
            ast::Node::function_define(
                ast::Node::indentifier(Token {
                    kind: Kind::ID,
                    value: String::from("hello"),
                }),
                vec![
                    ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("name"),
                    }),
                ],
                vec![
                    ast::Node::stdout(ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("name"),
                    })),
                    ast::Node::empty(),
                ],
            ),
            parser.statements()
        )
    }

    #[test]
    fn test_define_function_with_args() {
        let text = "(defn hello [name surname] (print name) (print surname))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!(
            ast::Node::function_define(
                ast::Node::indentifier(Token {
                    kind: Kind::ID,
                    value: String::from("hello"),
                }),
                vec![
                    ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("name"),
                    }),
                    ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("surname"),
                    }),
                ],
                vec![
                    ast::Node::stdout(ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("name"),
                    })),
                    ast::Node::stdout(ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("surname"),
                    })),
                    ast::Node::empty(),
                ],
            ),
            parser.statements()
        )
    }

    #[test]
    fn test_define_function_with_complex() {
        let text = "(defn hello [a b] (print (= b a)) (print b))";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        let nodes = vec![
            ast::Node::indentifier(Token {
                kind: Kind::ID,
                value: String::from("b"),
            }),
            ast::Node::indentifier(Token {
                kind: Kind::ID,
                value: String::from("a"),
            }),
        ];

        let eq_comparison = build_node_comparision(String::from("="), nodes);

        assert_eq!(
            ast::Node::function_define(
                ast::Node::indentifier(Token {
                    kind: Kind::ID,
                    value: String::from("hello"),
                }),
                vec![
                    ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("a"),
                    }),
                    ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("b"),
                    }),
                ],
                vec![
                    ast::Node::stdout(eq_comparison),
                    ast::Node::empty(),
                    ast::Node::stdout(ast::Node::indentifier(Token {
                        kind: Kind::ID,
                        value: String::from("b"),
                    })),
                    ast::Node::empty(),
                ],
            ),
            parser.statements()
        )
    }
}
