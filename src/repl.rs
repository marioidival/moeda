use token::Tokenizer;
use parser::Parser;
use interpreter::Interpreter;

pub struct Repl {
    interpreter: Interpreter,
}

impl Repl {
    pub fn new() -> Self {
        Repl { interpreter: Interpreter::new() }
    }

    pub fn eval(&mut self, source: String) -> String {
        let tokenizer = Tokenizer::new(source);
        let mut parser = Parser::new(tokenizer);
        self.interpreter.eval(parser.parse())
    }
}
