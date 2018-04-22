#![feature(io)]

mod env;
mod eval;
mod lexer;
mod parser;
mod syntax;

use env::Env;
use eval::eval;
use lexer::Lexer;

use std::io::{self, stdin, Read, Write};

fn main() {
    let lexer = Lexer::new(stdin().chars().filter_map(|r| r.ok()));
    let mut lexer = lexer.peekable();
    let env = Env::new_default();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        println!("{:?}", eval(&mut lexer, &env));
        println!("");
    }
}
