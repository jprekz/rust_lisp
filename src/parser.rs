use super::env::Env;
use super::lexer::{Token, TokenStream};

use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct RefValue(Rc<RefCell<Value>>);
impl RefValue {
    pub fn new(value: Value) -> RefValue {
        RefValue(Rc::new(RefCell::new(value)))
    }
}
impl Deref for RefValue {
    type Target = Rc<RefCell<Value>>;
    fn deref(&self) -> &Rc<RefCell<Value>> {
        &self.0
    }
}

#[derive(Clone)]
pub enum Value {
    Nil,
    Cons(RefValue, RefValue),
    Quoted(RefValue),
    Bool(bool),
    Num(f64),
    Ident(String),
    Syntax(&'static str, fn(Value, &Env) -> Value),
    Subr(&'static str, fn(&Iterator<Item=Value>) -> Value),
    Closure(RefValue, RefValue, Env),
}
impl Value {
    pub fn try_into_nil(self) -> Option<()> {
        match self {
            Value::Nil => Some(()),
            _ => None,
        }
    }
    pub fn try_into_cons(self) -> Option<(Value, Value)> {
        match self {
            Value::Cons(car, cdr) => {
                Some((car.borrow().clone(), cdr.borrow().clone()))
            }
            _ => None,
        }
    }
    pub fn try_into_num(self) -> Option<f64> {
        match self {
            Value::Num(a) => Some(a),
            _ => None,
        }
    }
    pub fn try_into_ident(self) -> Option<String> {
        match self {
            Value::Ident(ident) => Some(ident),
            _ => None,
        }
    }
}
impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Value::Nil => write!(f, "()"),
            Value::Cons(car, cdr) => write!(f, "({:?} . {:?})", car.borrow(), cdr.borrow()),
            Value::Quoted(value) => write!(f, "'{:?}", value.borrow()),
            Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Num(num) => write!(f, "{}", num),
            Value::Ident(ident) => write!(f, "{}", ident),
            Value::Syntax(name, _) => write!(f, "#<syntax {}>", name),
            Value::Subr(name, _) => write!(f, "#<subr {}>", name),
            Value::Closure(a, b, _) => write!(f, "#<closure {:?} {:?}>", a, b),
        }
    }
}

pub fn rr_new<T>(t: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(t))
}

pub fn parse(token_stream: &mut TokenStream) -> Value {
    match token_stream.next().unwrap() {
        Token::LPER => (),
        Token::QUOTE => {
            return Value::Quoted(RefValue::new(parse(token_stream)));
        }
        Token::BOOL(b) => {
            return Value::Bool(b);
        }
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
            let value = Value::Quoted(RefValue::new(parse(token_stream)));
            if let Some(Token::RPER) = token_stream.next() {
                return value;
            } else {
                panic!("syntax error");
            }
        }
    }
    let mut tail = RefValue::new(Value::Nil);
    let head = Value::Cons(RefValue::new(parse(token_stream)), tail.clone());
    while let Some(peek) = token_stream.peek().map(|c| c.clone()) {
        match peek {
            Token::RPER => {
                token_stream.next();
                return head;
            }
            Token::DOT => {
                token_stream.next();
                let value = parse(token_stream);
                tail.replace(value);
                if let Some(Token::RPER) = token_stream.next() {
                    return head;
                } else {
                    panic!();
                }
            }
            _ => {
                let value = parse(token_stream);
                let next_tail = RefValue::new(Value::Nil);
                tail.replace(Value::Cons(RefValue::new(value), next_tail.clone()));
                tail = next_tail;
            }
        }
    }
    panic!();
}

pub fn parse_to_vec<T: Iterator<Item = Token>>(token_stream: T) -> Vec<Token> {
    let mut vec = Vec::new();
    let mut per_count = 0;
    for token in token_stream {
        match &token {
            Token::LPER => per_count += 1,
            Token::RPER => per_count -= 1,
            Token::QUOTE => {
                vec.push(token);
                continue;
            }
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
