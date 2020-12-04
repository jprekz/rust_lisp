mod builtins;
mod env;
mod eval;
mod lexer;
mod parser;
mod value;

use crate::env::Env;
use crate::eval::eval;
use crate::lexer::Lexer;
use crate::parser::parse;

use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::path::PathBuf;

use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust_lisp")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Script file to run
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let mut builder = env_logger::builder();
    if opt.debug {
        builder.filter_level(log::LevelFilter::Debug);
        builder.format_timestamp(None);
    } else {
        builder.format(|buf, record| {
            writeln!(
                buf,
                "{}: {}",
                buf.default_styled_level(record.level()),
                record.args()
            )
        });
    }
    builder.init();

    let input: Box<dyn Iterator<Item = char>> = {
        if let Some(path) = &opt.file {
            let file = File::open(path).unwrap();
            Box::new(buf_reader_to_chars(BufReader::new(file)))
        } else {
            Box::new(StdinIter::new())
        }
    };

    let mut lexer = Lexer::new(input).peekable();
    let env = Env::new_default();

    loop {
        if opt.file.is_none() {
            print!("> ");
            stdout().flush().unwrap();
        }

        if let None = lexer.peek() {
            break;
        }
        let parsed = match parse(&mut lexer) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                break;
            }
        };
        match eval(parsed, env.clone()) {
            Ok(value) => {
                if opt.file.is_none() {
                    println!("{:?}", value);
                    println!("");
                }
            }
            Err(e) => {
                log::error!("{:?}", e);
            }
        }
    }
}

fn buf_reader_to_chars(buf_reader: impl BufRead) -> impl Iterator<Item = char> {
    buf_reader
        .lines()
        .map(|s| -> Vec<char> { s.unwrap().chars().collect() })
        .flatten()
}

struct StdinIter {
    buf: Vec<char>,
}
impl StdinIter {
    fn new() -> StdinIter {
        StdinIter { buf: Vec::new() }
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
