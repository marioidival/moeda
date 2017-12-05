use token::{Token, Kind, Tokenizer};
use ast;
use primitive::Type;

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
                    _ => {
                        let tok = self.tokenizer.advance().consume(Kind::Comparison);
                        let nodes = self.args_list();
                        self.tokenizer.consume(Kind::GroupEnd);
                        ast::Node::comparison(tok.value, nodes)
                    }
                }
            }
            _ => ast::Node::empty(),
        }
    }

    fn factor(&mut self) -> ast::Node {
        match self.tokenizer.advance().get() {
            Some(Token { kind: Kind::GroupBegin, .. }) => self.statements(),
            Some(Token { kind: Kind::Bolean, .. }) => ast::Node::constant(
                self.tokenizer.advance().consume(
                    Kind::Bolean,
                ),
            ),
            Some(Token { kind: Kind::Integer, .. }) => {
                ast::Node::constant(self.tokenizer.advance().consume(Kind::Integer))
            }
            _ => ast::Node::empty(),
        }
    }

    fn args_list(&mut self) -> Vec<ast::Node> {
        let mut args = vec![];

        match self.tokenizer.advance().get() {
            Some(Token { kind: Kind::Space, .. }) => {
                self.tokenizer.consume(Kind::Space);
            }
            Some(Token { kind: Kind::GroupEnd, .. }) => return args,
            _ => args.push(self.factor()),
        }
        args.extend(self.args_list());
        args
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
}
