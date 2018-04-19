#![feature(io)]

mod lexer;
use lexer::*;

mod parser;
use parser::*;

use std::io::{stdin, Read};
use std::iter::{Iterator, Peekable};

fn main() {
    let lexer = Lexer::new(stdin().chars().filter_map(|r| r.ok()));
    let _lexer = Lexer::new("(lambda (x) x)".chars());
    let value = eval(&mut lexer.peekable());
    println!("{:?}", value);
}

fn eval<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    match token_stream.next().unwrap() {
        Token::LPER => {
            if let Some(Token::RPER) = token_stream.peek() {
                token_stream.next();
                return Value::Nil;
            }
            let value = match eval(token_stream) {
                Value::Syntax(syntax) => match syntax.as_ref() {
                    "quote" => syntax_quote(token_stream),
                    "lambda" => syntax_lambda(token_stream),
                    "cons" => syntax_cons(token_stream),
                    _ => unreachable!(),
                },
                Value::Procedure(_args, _body) => {
                    unimplemented!()
                },
                _ => panic!("invalid application"),
            };
            if let Some(Token::RPER) = token_stream.next() {
                return value;
            } else {
                panic!("syntax error");
            }
        }
        Token::RPER => panic!("syntax error"),
        Token::QUOTE => syntax_quote(token_stream),
        Token::DOT => panic!("syntax error"),
        Token::BOOL(b) => Value::Bool(b),
        Token::IDENT(ident) => {
            match ident.as_ref() {
                "quote" => Value::Syntax(ident),
                "lambda" => Value::Syntax(ident),
                "cons" => Value::Syntax(ident),
                _ => panic!("unbound variable: {}", ident),
            }
        },
        Token::NUM(num) => Value::Num(num),
    }
}

fn syntax_quote<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    parse(token_stream)
}

fn syntax_lambda<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    let args = parse_to_vec(token_stream);
    let body = parse_to_vec(token_stream);
    Value::Procedure(args, body)
}

fn syntax_cons<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    let car = eval(token_stream);
    let cdr = eval(token_stream);
    return Value::Cons(rr_new(car), rr_new(cdr));
}

