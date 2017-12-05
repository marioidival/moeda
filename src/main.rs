extern crate moeda;

use std::io;
use std::io::prelude::*;

use moeda::repl;

fn main() {
    print!(">>");
    let stdin = io::stdin();
    let mut repl = repl::Repl::new();
    while let Some(line) = stdin.lock().lines().next() {
        if let Ok(source_code) = line {
            println!("{}", repl.eval(source_code));
        }
        print!(">>");
        io::stdout().flush().ok().expect(
            "Ops... Something went wrong. :(",
        );
    }
}
