#![feature(io)]

mod lexer;
use lexer::*;

use std::cell::RefCell;
use std::io::{stdin, Read};
use std::iter::{Iterator, Peekable};
use std::rc::Rc;

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
            if let Some(Token::IDENT(func_ident)) = token_stream.next() {
                let value = match func_ident.as_ref() {
                    "quote" => syntax_quote(token_stream),
                    "lambda" => syntax_lambda(token_stream),
                    "cons" => syntax_cons(token_stream),
                    _ => {
                        let app = eval(token_stream);
                        if let Value::Procedure(args, body) = app {
                            unimplemented!();

                        } else {
                            panic!("invalid application");
                        }
                    },
                };
                if let Some(Token::RPER) = token_stream.next() {
                    return value;
                } else {
                    panic!("syntax error");
                }
            } else {
                panic!("invalid application");
            }
        }
        Token::RPER => panic!("syntax error"),
        Token::QUOTE => syntax_quote(token_stream),
        Token::DOT => panic!("syntax error"),
        Token::BOOL(b) => Value::Bool(b),
        Token::IDENT(ident) => panic!("unbound variable: {}", ident),
        Token::NUM(num) => Value::Num(num),
    }
}

fn syntax_quote<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    match token_stream.next().unwrap() {
        Token::LPER => (),
        Token::QUOTE => {
            return Value::Quoted(rr_new(syntax_quote(token_stream)));
        },
        Token::BOOL(b) => {
            return Value::Bool(b);
        },
        Token::IDENT(ident) => {
            return Value::Ident(ident);
        }
        Token::NUM(num) => {
            return Value::Num(num);
        }
        _ => panic!(),
    }
    if let Some(Token::IDENT(ident)) = token_stream.peek().map(|c| c.clone()) {
        if ident.eq("quote") {
            token_stream.next();
            let value = Value::Quoted(rr_new(syntax_quote(token_stream)));
            if let Some(Token::RPER) = token_stream.next() {
                return value;
            } else {
                panic!("syntax error");
            }
        }
    }
    let mut tail = rr_new(Value::Nil);
    let head = Value::Cons(rr_new(syntax_quote(token_stream)), tail.clone());
    while let Some(peek) = token_stream.peek().map(|c| c.clone()) {
        match peek {
            Token::RPER => {
                token_stream.next();
                return head;
            },
            Token::DOT => {
                token_stream.next();
                let value = syntax_quote(token_stream);
                tail.replace(value);
                if let Some(Token::RPER) = token_stream.next() {
                    return head;
                } else {
                    panic!();
                }
            },
            _ => {
                let value = syntax_quote(token_stream);
                let next_tail = rr_new(Value::Nil);
                tail.replace(Value::Cons(rr_new(value), next_tail.clone()));
                tail = next_tail;
            },
        }
    }
    panic!();
}

fn syntax_lambda<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    let args = record_one_value(token_stream);
    let body = record_one_value(token_stream);
    Value::Procedure(args, body)
}
fn record_one_value<T: Iterator<Item = Token>>(token_stream: &mut T) -> Vec<Token> {
    let mut vec = Vec::new();
    let mut per_count = 0;
    for token in token_stream {
        match &token {
            Token::LPER => per_count += 1,
            Token::RPER => per_count -= 1,
            Token::QUOTE => {
                vec.push(token);
                continue;
            },
            Token::DOT => panic!(),
            _ => (),
        }
        vec.push(token);
        if per_count == 0 {
            return vec;
        } else if per_count < 0 {
            panic!();
        }
    }
    panic!();
}

fn syntax_cons<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    let car = eval(token_stream);
    let cdr = eval(token_stream);
    return Value::Cons(rr_new(car), rr_new(cdr));
}

#[derive(Clone)]
enum Value {
    Cons(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    Nil,
    Quoted(Rc<RefCell<Value>>),
    Bool(bool),
    Num(f64),
    Ident(String),
    Procedure(Vec<Token>, Vec<Token>),
}
impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Cons(car, cdr) => write!(f, "({:?} . {:?})", car.borrow(), cdr.borrow()),
            Value::Quoted(value) => write!(f, "'{:?}", value.borrow()),
            Value::Nil => write!(f, "()"),
            Value::Bool(b) => if *b { write!(f, "#t") } else { write!(f, "#f") },
            Value::Num(num) => write!(f, "{}", num),
            Value::Ident(ident) => write!(f, "{}", ident),
            Value::Procedure(a, b) => write!(f, "<Procedure {:?} {:?}>", a, b),
        }
    }
}

fn rr_new<T>(t: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(t))
}
