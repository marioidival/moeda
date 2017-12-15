extern crate moeda;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use moeda::repl;

fn main() {
    let mut repl = repl::Repl::new();
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let result = repl.eval(line);
                if !result.is_empty() {
                    println!("{}", result);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
