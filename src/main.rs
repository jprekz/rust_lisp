#![feature(transpose_result)]

mod env;
mod eval;
mod lexer;
mod parser;
mod syntax;
mod value;

use crate::env::Env;
use crate::eval::eval;
use crate::lexer::Lexer;
use crate::parser::parse;

use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        let file = File::open(arg).unwrap();
        run(buf_reader_to_chars(BufReader::new(file)));
    } else {
        repr();
    }
}

fn buf_reader_to_chars(buf_reader: impl BufRead) -> impl Iterator<Item = char> {
    buf_reader
        .lines()
        .map(|s| -> Vec<char> { s.unwrap().chars().collect() })
        .flatten()
}

fn run(input: impl Iterator<Item = char>) {
    let lexer = Lexer::new(input);
    let mut lexer = lexer.peekable();
    let env = Env::new_default();
    loop {
        if let None = lexer.peek() {
            break;
        }
        let parsed = match parse(&mut lexer) {
            Ok(v) => v,
            Err(e) => {
                println!("\nError: {}", e);
                break;
            }
        };
        eval(parsed, env.clone());
    }
}

struct StdinIter {
    buf: Vec<char>,
}
impl StdinIter {
    fn new() -> StdinIter {
        StdinIter {
            buf: Vec::new(),
        }
    }
}
impl Iterator for StdinIter {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if self.buf.is_empty() {
            let mut buf = String::new();
            let bytes = stdin().read_line(&mut buf).unwrap();
            if bytes == 0 {
                return None;
            }
            self.buf = buf.chars().collect();
        }
        let mut buf = self.buf.split_off(1);
        std::mem::swap(&mut buf, &mut self.buf);
        Some(buf[0])
    }
}

fn repr() {
    let lexer = Lexer::new(StdinIter::new());
    let mut lexer = lexer.peekable();
    let env = Env::new_default();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        if let None = lexer.peek() {
            break;
        }
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
