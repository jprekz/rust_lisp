use super::lexer::Token;

use std::cell::RefCell;
use std::iter::{Iterator, Peekable};
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Cons(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    Nil,
    Quoted(Rc<RefCell<Value>>),
    Bool(bool),
    Num(f64),
    Ident(String),
    Syntax(String),
    Procedure(Vec<String>, Vec<Token>),
}
impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Value::Cons(car, cdr) => write!(f, "({:?} . {:?})", car.borrow(), cdr.borrow()),
            Value::Quoted(value) => write!(f, "'{:?}", value.borrow()),
            Value::Nil => write!(f, "()"),
            Value::Bool(b) => if *b { write!(f, "#t") } else { write!(f, "#f") },
            Value::Num(num) => write!(f, "{}", num),
            Value::Ident(ident) => write!(f, "{}", ident),
            Value::Syntax(name) => write!(f, "#<syntax {}>", name),
            Value::Procedure(a, b) => write!(f, "#<procedure {:?} {:?}>", a, b),
        }
    }
}

pub fn rr_new<T>(t: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(t))
}

pub fn parse<T: Iterator<Item = Token>>(token_stream: &mut Peekable<T>) -> Value {
    match token_stream.next().unwrap() {
        Token::LPER => (),
        Token::QUOTE => {
            return Value::Quoted(rr_new(parse(token_stream)));
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
            let value = Value::Quoted(rr_new(parse(token_stream)));
            if let Some(Token::RPER) = token_stream.next() {
                return value;
            } else {
                panic!("syntax error");
            }
        }
    }
    let mut tail = rr_new(Value::Nil);
    let head = Value::Cons(rr_new(parse(token_stream)), tail.clone());
    while let Some(peek) = token_stream.peek().map(|c| c.clone()) {
        match peek {
            Token::RPER => {
                token_stream.next();
                return head;
            },
            Token::DOT => {
                token_stream.next();
                let value = parse(token_stream);
                tail.replace(value);
                if let Some(Token::RPER) = token_stream.next() {
                    return head;
                } else {
                    panic!();
                }
            },
            _ => {
                let value = parse(token_stream);
                let next_tail = rr_new(Value::Nil);
                tail.replace(Value::Cons(rr_new(value), next_tail.clone()));
                tail = next_tail;
            },
        }
    }
    panic!();
}

pub fn parse_to_vec<T: Iterator<Item = Token>>(token_stream: &mut T) -> Vec<Token> {
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

