#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Integer,
    Str,
    Bolean,
    List,
    Alphanum,

    Operator,
    Logical,
    Comparison,

    GroupBegin,
    GroupEnd,
    ArgsBegin,
    ArgsEnd,

    ID,

    Comment,
    Space,
    Separator,

    StdOut,

    If,
    When,
    FnDefine,
    VarDefine,

    EndLine,
    EOF,
}

impl Kind {
    pub fn classify(character: &Option<char>) -> Kind {
        match *character {
            Some(value) => match value {
                ';' => Kind::Comment,
                ',' => Kind::Separator,
                '(' => Kind::GroupBegin,
                ')' => Kind::GroupEnd,
                '[' => Kind::ArgsBegin,
                ']' => Kind::ArgsEnd,
                ' ' => Kind::Space,
                '\n' => Kind::EndLine,
                '"' => Kind::Str,
                '\'' => Kind::List,
                '+' | '-' | '*' | '/' => Kind::Operator,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Kind::Integer,
                _ => Kind::Alphanum,
            },
            None => Kind::EOF,
        }
    }

    pub fn reserved(word: &String) -> Option<Kind> {
        match word.as_ref() {
            "defn" => Some(Kind::FnDefine),
            "def" => Some(Kind::VarDefine),
            "inc" | "rem" | "dec" => Some(Kind::Operator),
            "print" => Some(Kind::StdOut),
            "if" => Some(Kind::If),
            "when" => Some(Kind::When),
            "and" | "or" | "not" => Some(Kind::Logical),
            "=" | "/=" | ">" | "<" | "<=" | ">=" | "max" | "min" => Some(Kind::Comparison),
            "true" | "false" => Some(Kind::Bolean),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
}

impl Token {
    pub fn build(kind: Kind, value: String) -> Token {
        Token { kind, value }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    pub text: String,
    pub position: usize,
    current: Option<Token>,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Tokenizer {
            text: text,
            position: 0,
            current: None,
        }
    }
}

impl Tokenizer {
    pub fn current(&self) -> Option<char> {
        self.text.chars().nth(self.position)
    }

    pub fn advance(&mut self) -> &mut Self {
        if self.current.is_none() {
            self.current = self.next();
        }
        self
    }

    pub fn get(&mut self) -> Option<Token> {
        self.current.clone()
    }

    pub fn peek(&mut self) -> Option<Token> {
        let curr_position = self.position.clone();
        if self.position == 0 {
            self.position += 1;
        }
        let next = self.next();
        self.position = curr_position;
        next
    }

    pub fn consume(&mut self, expect_kind: Kind) -> Token {
        if let Some(token) = self.get() {
            self.current = None;
            if token.kind != expect_kind {
                panic!(
                    "Syntax error: expect token kind: {:?} found {:?} at position {}",
                    expect_kind, token, self.position
                );
            }
            return token;
        } else {
            panic!("Lexer error: expected {:?} found end of file", expect_kind)
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let current = self.current();
        let kind = Kind::classify(&current);

        self.position += 1;

        match kind {
            Kind::Comment => {
                while Kind::EndLine != Kind::classify(&self.current()) {
                    self.position += 1;
                }

                self.next()
            }
            Kind::Str => {
                let c: Vec<char> = self.text[self.position..]
                    .chars()
                    .take_while(|b| Kind::classify(&Some(*b)) != Kind::Str)
                    .collect();
                self.position += c.len() + 1;
                let w: String = c.clone().into_iter().collect();
                Some(Token::build(kind, w))
            }
            Kind::List => {
                let c: Vec<char> = self.text[self.position..]
                    .chars()
                    .skip(1) // Skip GroupBegin
                    .take_while(|b| Kind::classify(&Some(*b)) != Kind::GroupEnd)
                    .collect();
                self.position += c.len() + 2;
                let w: String = c.clone().into_iter().collect();
                let ww: Vec<&str> = w.split(' ').collect();
                Some(Token::build(kind, String::from(ww.join(","))))
            }
            Kind::Space | Kind::EndLine => self.next(),
            Kind::Operator => {
                if current == Some('/')
                    && self.peek() == Some(Token::build(Kind::Comparison, String::from("=")))
                {
                    self.position += 1;
                    Some(Token::build(Kind::Comparison, String::from("/=")))
                } else {
                    Some(Token::build(kind, format!("{}", current.unwrap())))
                }
            }
            Kind::GroupBegin | Kind::GroupEnd | Kind::Logical | Kind::Comparison => {
                Some(Token::build(kind, format!("{}", current.unwrap())))
            }
            Kind::Alphanum => {
                let mut chars = vec![current.unwrap()];
                let mut next = self.current();
                let mut kindnext = Kind::classify(&next);

                while kindnext == kind || kindnext == Kind::Integer {
                    chars.push(next.unwrap());
                    self.position += 1;
                    next = self.current();
                    kindnext = Kind::classify(&next);
                }

                let word: String = chars.clone().into_iter().collect();
                if let Some(reserved) = Kind::reserved(&word) {
                    Some(Token {
                        kind: reserved,
                        value: word,
                    })
                } else {
                    Some(Token {
                        kind: Kind::ID,
                        value: word,
                    })
                }
            }
            _ => {
                if current == None {
                    Some(Token::build(Kind::EOF, String::new()))
                } else {
                    let mut chars = vec![current.unwrap()];
                    let mut next = self.current();
                    let mut kindnext = Kind::classify(&next);

                    while kindnext == kind {
                        chars.push(next.unwrap());
                        self.position += 1;
                        next = self.current();
                        kindnext = Kind::classify(&next);
                    }

                    Some(Token::build(kind, chars.into_iter().collect()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_eof() {
        assert_eq!(Kind::EOF, Kind::classify(&None));
    }

    #[test]
    fn test_identify_comment() {
        assert_eq!(Kind::Comment, Kind::classify(&Some(';')));
    }

    #[test]
    fn test_identify_separator() {
        assert_eq!(Kind::Separator, Kind::classify(&Some(',')));
    }

    #[test]
    fn test_identify_space() {
        assert_eq!(Kind::Space, Kind::classify(&Some(' ')));
    }

    #[test]
    fn test_identify_integer() {
        let v = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        for i in v {
            assert_eq!(Kind::Integer, Kind::classify(&Some(i)));
        }
    }

    #[test]
    fn test_identify_groups() {
        assert_eq!(Kind::GroupBegin, Kind::classify(&Some('(')));
        assert_eq!(Kind::GroupEnd, Kind::classify(&Some(')')));
    }

    #[test]
    fn test_identify_args() {
        assert_eq!(Kind::ArgsBegin, Kind::classify(&Some('[')));
        assert_eq!(Kind::ArgsEnd, Kind::classify(&Some(']')));
    }

    #[test]
    fn test_identify_end_line() {
        assert_eq!(Kind::EndLine, Kind::classify(&Some('\n')));
    }

    #[test]
    fn test_identify_operators() {
        assert_eq!(Kind::Operator, Kind::classify(&Some('+')));
        assert_eq!(Kind::Operator, Kind::classify(&Some('-')));
        assert_eq!(Kind::Operator, Kind::classify(&Some('/')));
        assert_eq!(Kind::Operator, Kind::classify(&Some('*')));
        assert_eq!(Some(Kind::Operator), Kind::reserved(&String::from("inc")));
        assert_eq!(Some(Kind::Operator), Kind::reserved(&String::from("dec")));
        assert_eq!(Some(Kind::Operator), Kind::reserved(&String::from("rem")));
    }

    #[test]
    fn test_identify_str() {
        assert_eq!(Kind::Str, Kind::classify(&Some('"')))
    }

    #[test]
    fn test_identify_list() {
        assert_eq!(Kind::List, Kind::classify(&Some('\'')));
    }

    #[test]
    fn test_identify_function() {
        assert_eq!(Some(Kind::FnDefine), Kind::reserved(&String::from("defn")));
    }

    #[test]
    fn test_identify_immutable() {
        assert_eq!(Some(Kind::VarDefine), Kind::reserved(&String::from("def")));
    }

    #[test]
    fn test_identify_comparison() {
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from("<")));
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from(">")));
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from("<=")));
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from(">=")));
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from("/=")));
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from("=")));
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from("max")));
        assert_eq!(Some(Kind::Comparison), Kind::reserved(&String::from("min")));
    }

    #[test]
    fn test_identify_logical() {
        assert_eq!(Some(Kind::Logical), Kind::reserved(&String::from("and")));
        assert_eq!(Some(Kind::Logical), Kind::reserved(&String::from("or")));
        assert_eq!(Some(Kind::Logical), Kind::reserved(&String::from("not")));
    }

    #[test]
    fn test_identify_bool() {
        assert_eq!(Some(Kind::Bolean), Kind::reserved(&String::from("true")));
        assert_eq!(Some(Kind::Bolean), Kind::reserved(&String::from("false")));
    }

    #[test]
    fn test_identify_stdout() {
        assert_eq!(Some(Kind::StdOut), Kind::reserved(&String::from("print")));
    }

    #[test]
    fn test_identify_if() {
        assert_eq!(Some(Kind::If), Kind::reserved(&String::from("if")));
    }

    #[test]
    fn test_identify_when() {
        assert_eq!(Some(Kind::When), Kind::reserved(&String::from("when")));
    }

    #[test]
    fn test_tokenizer_new() {
        let text = "1 + 1";
        let tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(0, tokenizer.position);
        assert_eq!(text, tokenizer.text);
    }

    #[test]
    fn test_tokenizer_current() {
        let text = "1 + 1";
        let tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(Some('1'), tokenizer.current());
    }

    #[test]
    fn test_tokenizer_next() {
        let text = "(+ 1 1)";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Operator,
                value: String::from("+"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
    }

    #[test]
    fn test_tokenizer_get() {
        let text = "(+ 1 1)";
        let mut tokenizer = Tokenizer::new(String::from(text));
        tokenizer.advance();

        assert_eq!(
            Some(Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }),
            tokenizer.get()
        )
    }

    #[test]
    fn test_tokenizer_peek() {
        let text = "(+ 1 1)";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            Some(Token {
                kind: Kind::Operator,
                value: String::from("+"),
            }),
            tokenizer.peek()
        );
        assert_eq!(Some('('), tokenizer.current())
    }

    #[test]
    fn test_tokenizer_consume() {
        let text = "(+ 1 1)";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            },
            tokenizer.advance().consume(Kind::GroupBegin)
        );
    }

    #[test]
    fn test_tokenizer_consume_comparision_equals() {
        let text = "(= 1 1)";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            },
            tokenizer.advance().consume(Kind::GroupBegin)
        );
        assert_eq!(
            Token {
                kind: Kind::Comparison,
                value: String::from("="),
            },
            tokenizer.advance().consume(Kind::Comparison)
        );
        assert_eq!(
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            },
            tokenizer.advance().consume(Kind::Integer)
        );
        assert_eq!(
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            },
            tokenizer.advance().consume(Kind::Integer)
        );
        assert_eq!(
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            },
            tokenizer.advance().consume(Kind::GroupEnd)
        );
    }

    #[test]
    fn test_tokenizer_consume_comparision_different() {
        let text = "(/= 1 1)";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            },
            tokenizer.advance().consume(Kind::GroupBegin)
        );
        assert_eq!(
            Token {
                kind: Kind::Comparison,
                value: String::from("/="),
            },
            tokenizer.advance().consume(Kind::Comparison)
        );
        assert_eq!(
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            },
            tokenizer.advance().consume(Kind::Integer)
        );
        assert_eq!(
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            },
            tokenizer.advance().consume(Kind::Integer)
        );
        assert_eq!(
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            },
            tokenizer.advance().consume(Kind::GroupEnd)
        );
    }

    #[test]
    fn test_tokenizer_next_with_comments() {
        let text = ";;;this is a example of sum\n(+ 1 1)";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Operator,
                value: String::from("+"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
    }

    #[test]
    fn test_tokenizer_with_print() {
        let text = "(print (+ 1 1))";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::StdOut,
                value: String::from("print"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Operator,
                value: String::from("+"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("1"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        )
    }

    #[test]
    fn test_tokenizer_next_with_str() {
        let text = "(print \"ola\")";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::StdOut,
                value: String::from("print"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Str,
                value: String::from("ola"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
    }

    #[test]
    fn test_tokenizer_next_list() {
        let text = "(print '(1 2 true))";
        let mut tokenizer = Tokenizer::new(String::from(text));
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::StdOut,
                value: String::from("print"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::List,
                value: String::from("1,2,true"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
    }

    #[test]
    fn test_tokenizer_next_if_statement() {
        let text = "(if (= 9 9) (print \"eq\") (print \"neq\"))";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::If,
                value: String::from("if"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Comparison,
                value: String::from("="),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("9"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::StdOut,
                value: String::from("print"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Str,
                value: String::from("eq"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::StdOut,
                value: String::from("print"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Str,
                value: String::from("neq"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        )
    }

    #[test]
    fn test_tokenizer_next_with_when_statements() {
        let text = "(when (> 3 2) (print \"big\"))";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::When,
                value: String::from("when"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Comparison,
                value: String::from(">"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("3"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("2"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::StdOut,
                value: String::from("print"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Str,
                value: String::from("big"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
    }

    #[test]
    fn test_tokenizer_next_id() {
        let text = "(def x 2)";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::VarDefine,
                value: String::from("def"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::ID,
                value: String::from("x"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("2"),
            }
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        )
    }

    #[test]
    fn test_tokenizer_next_with_id() {
        let text = "(defn maior_que_dois [arg] (> arg 2))";
        let mut tokenizer = Tokenizer::new(String::from(text));

        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::FnDefine,
                value: String::from("defn"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::ID,
                value: String::from("maior_que_dois"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::ArgsBegin,
                value: String::from("["),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::ID,
                value: String::from("arg"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::ArgsEnd,
                value: String::from("]"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupBegin,
                value: String::from("("),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Comparison,
                value: String::from(">"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::ID,
                value: String::from("arg"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::Integer,
                value: String::from("2"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
        assert_eq!(
            tokenizer.next().unwrap(),
            Token {
                kind: Kind::GroupEnd,
                value: String::from(")"),
            }
        );
    }
}
