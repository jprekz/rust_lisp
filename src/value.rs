use super::env::Env;

use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct RefValue(Rc<RefCell<Value>>);
impl RefValue {
    pub fn new(value: Value) -> RefValue {
        RefValue(Rc::new(RefCell::new(value)))
    }
}
impl RefValue {
    pub fn to_value(&self) -> Value {
        self.borrow().clone()
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
                Some((car.to_value(), cdr.to_value()))
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
            Value::Closure(a, b, _) => write!(f, "#<closure {:?} {:?}>", a, b),
        }
    }
}

impl Iterator for Value {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        match self.clone() {
            Value::Nil => None,
            Value::Cons(car, cdr) => {
                *self = cdr.borrow().clone();
                Some(car.borrow().clone())
            }
            other => {
                *self = Value::Nil;
                Some(other)
            }
        }
    }
}
