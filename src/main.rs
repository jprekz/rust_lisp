#![feature(io)]
#![feature(transpose_result)]

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
        repr(bytes_to_chars(file));
    } else {
        repr(bytes_to_chars(stdin()));
    }
}

// FIXME: deprecated
fn bytes_to_chars(bytes: impl Read) -> impl Iterator<Item=char> {
    bytes.chars().filter_map(|r| r.ok())
}

fn repr<T>(input: T)
where T: Iterator<Item=char> + 'static {
    let lexer = Lexer::new(input);
    let mut lexer = lexer.peekable();
    let env = Env::new_default();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let parsed = match parse(&mut lexer) {
            Ok(v) => v,
            Err(e) => {
                println!("\nError: {}", e);
                break;
            }
        };
        println!("{:?}", eval(parsed, env.clone()));
        println!("");
    }
}

