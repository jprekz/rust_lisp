use super::env::Env;
use super::eval::eval;
use super::lexer::{Token, TokenStream};
use super::parser::{parse, parse_to_vec, rr_new, Value};

pub static SYNTAX: &'static [(&'static str, fn(&mut TokenStream, &Env) -> Value)] = &[
    ("define", |token_stream, env| {
        match token_stream.next() {
            Some(Token::IDENT(ident)) => {
                let value = eval(token_stream, env);
                env.insert(ident, value);
                Value::Bool(true)
            }
            Some(Token::LPER) => {
                let ident = match token_stream.next() {
                    Some(Token::IDENT(ident)) => ident,
                    _ => panic!("syntax error"),
                };
                let mut args = Vec::new();
                while let Some(token) = token_stream.next() {
                    match token {
                        Token::IDENT(ident) => {
                            args.push(ident);
                        }
                        Token::RPER => break,
                        _ => panic!("syntax error"),
                    }
                }
                let body = parse_to_vec(token_stream);
                let value = Value::Closure(args, body, env.clone());
                env.insert(ident, value);
                Value::Bool(true)
            }
            _ => panic!("syntax error"),
        }
    }),
    ("quote", |token_stream, _env| parse(token_stream)),
    ("lambda", |token_stream, env| {
        if token_stream.next() != Some(Token::LPER) {
            panic!("syntax error");
        }
        let mut args = Vec::new();
        while let Some(token) = token_stream.next() {
            match token {
                Token::IDENT(ident) => {
                    args.push(ident);
                }
                Token::RPER => break,
                _ => panic!("syntax error"),
            }
        }
        let body = parse_to_vec(token_stream);
        Value::Closure(args, body, env.clone())
    }),
    ("cons", |token_stream, env| {
        let car = eval(token_stream, env);
        let cdr = eval(token_stream, env);
        Value::Cons(rr_new(car), rr_new(cdr))
    }),
    ("+", |token_stream, env| {
        let mut acc = 0.0;
        while token_stream.peek() != Some(&Token::RPER) {
            acc += eval(token_stream, env).try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("-", |token_stream, env| {
        let mut acc = eval(token_stream, env).try_into_num().unwrap();
        while token_stream.peek() != Some(&Token::RPER) {
            acc -= eval(token_stream, env).try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("*", |token_stream, env| {
        let mut acc = 1.0;
        while token_stream.peek() != Some(&Token::RPER) {
            acc *= eval(token_stream, env).try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("/", |token_stream, env| {
        let mut acc = eval(token_stream, env).try_into_num().unwrap();
        while token_stream.peek() != Some(&Token::RPER) {
            acc /= eval(token_stream, env).try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
];
