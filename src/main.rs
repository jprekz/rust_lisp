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
use std::fs::File;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        let file = File::open(arg).unwrap();
        repr(file.chars().filter_map(|r| r.ok()));
    } else {
        repr(stdin().chars().filter_map(|r| r.ok()));
    }
}

fn repr<T>(input: T)
where T: Iterator<Item=char> + 'static {
    let lexer = Lexer::new(input);
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

