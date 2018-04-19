#![feature(io)]

mod lexer;
use lexer::*;

mod parser;
use parser::*;

use std::io::{stdin, Read};
use std::iter::{Iterator, Peekable};
use std::collections::HashMap;

fn main() {
    let lexer = Lexer::new(stdin().chars().filter_map(|r| r.ok()));
    let mut lexer = lexer.peekable();
    let mut env = new_default_env();
    loop {
        println!("{:?}", eval(&mut lexer, &mut env));
    }
}

struct Env<'a> {
    inner: HashMap<String, Value>,
    outer: Option<&'a Env<'a>>,
}
impl<'a> Env<'a> {
    fn extend(&'a self) -> Env<'a> {
        Env {
            inner: HashMap::new(),
            outer: Some(self),
        }
    }
    fn insert(&mut self, key: String, value: Value) {
        self.inner.insert(key, value);
    }
    fn get(&self, key: String) -> Option<Value> {
        if let Some(value) = self.inner.get(&key) {
            Some(value.clone())
        } else {
            self.outer.and_then(|outer| outer.get(key))
        }
    }
}
fn new_default_env() -> Env<'static> {
    let mut hash_map = HashMap::new();
    let syntax_list = ["define", "quote", "lambda", "cons"];
    for s in &syntax_list {
        hash_map.insert(s.to_string(), Value::Syntax(s.to_string()));
    }
    Env {
        inner: hash_map,
        outer: None,
    }
}

fn eval<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>, env: &mut Env) -> Value {
    match token_stream.next().unwrap() {
        Token::LPER => {
            if let Some(Token::RPER) = token_stream.peek() {
                token_stream.next();
                return Value::Nil;
            }
            let value = match eval(token_stream, env) {
                Value::Syntax(syntax) => match syntax.as_ref() {
                    "define" => syntax_define(token_stream, env),
                    "quote" => syntax_quote(token_stream),
                    "lambda" => syntax_lambda(token_stream),
                    "cons" => syntax_cons(token_stream, env),
                    _ => unreachable!(),
                },
                Value::Procedure(args, body) => {
                    let mut extended_env = env.extend();
                    for arg in args {
                        let value = eval(token_stream, &mut extended_env);
                        extended_env.insert(arg, value);
                    }
                    eval(&mut body.iter().cloned().peekable(), &mut extended_env)
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
            if let Some(value) = env.get(ident.clone()) {
                value.clone()
            } else {
                panic!("unbound variable: {}", ident);
            }
        },
        Token::NUM(num) => Value::Num(num),
    }
}

fn syntax_define<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>, env: &mut Env) -> Value {
    let symbol = match token_stream.next() {
        Some(Token::IDENT(ident)) => ident,
        _ => panic!("syntax error"),
    };
    let value = eval(token_stream, env);
    env.insert(symbol, value);
    Value::Bool(true)
}

fn syntax_quote<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    parse(token_stream)
}

fn syntax_lambda<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    if token_stream.next() != Some(Token::LPER) {
        panic!("syntax error");
    }
    let mut args = Vec::new();
    while let Some(token) = token_stream.next() {
        match token {
            Token::IDENT(ident) => {
                args.push(ident);
            },
            Token::RPER => break,
            _ => panic!("syntax error"),
        }
    }
    let body = parse_to_vec(token_stream);
    Value::Procedure(args, body)
}

fn syntax_cons<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>, env: &mut Env) -> Value {
    let car = eval(token_stream, env);
    let cdr = eval(token_stream, env);
    return Value::Cons(rr_new(car), rr_new(cdr));
}

