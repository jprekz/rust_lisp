#![feature(io)]

mod env;
mod eval;
mod lexer;
mod parser;
mod syntax;
mod value;

use env::Env;
use eval::eval;
use lexer::Lexer;
use parser::parse;

use std::io::{stdin, stdout, Read, Write};

fn main() {
    let lexer = Lexer::new(stdin().chars().filter_map(|r| r.ok()));
    let mut lexer = lexer.peekable();
    let env = Env::new_default();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        println!("{:?}", eval(parse(&mut lexer), env.clone()));
        println!("");
        if let None = lexer.peek() {
            break;
        }
    }
}
