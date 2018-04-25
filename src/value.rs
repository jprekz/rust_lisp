use super::env::Env;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct RefValue(Rc<RefCell<Value>>);
impl RefValue {
    pub fn new(value: Value) -> RefValue {
        RefValue(Rc::new(RefCell::new(value)))
    }

    pub fn to_value(&self) -> Value {
        self.0.borrow().clone()
    }

    pub fn replace(&self, value: Value) -> Value {
        self.0.replace(value)
    }
}
impl PartialEq for RefValue {
    fn eq(&self, other: &RefValue) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl ::std::fmt::Debug for RefValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0.borrow())
    }
}

#[derive(Clone)]
pub struct SyntaxFn(fn(Value, &Env) -> Value);
impl SyntaxFn {
    pub fn new(f: fn(Value, &Env) -> Value) -> SyntaxFn {
        SyntaxFn(f)
    }
}
impl ::std::ops::Deref for SyntaxFn {
    type Target = fn(Value, &Env) -> Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PartialEq for SyntaxFn {
    fn eq(&self, other: &SyntaxFn) -> bool {
        ::std::ptr::eq(self, other)
    }
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Nil,
    Cons(RefValue, RefValue),
    Quoted(RefValue),
    Bool(bool),
    Num(f64),
    Ident(String),
    Syntax(&'static str, SyntaxFn),
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
    pub fn try_into_bool(self) -> Option<bool> {
        match self {
            Value::Bool(b) => {
                Some(b)
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
            Value::Cons(car, cdr) => {
                write!(f, "({:?}", car.0.borrow())?;
                fn fmt_l(next: RefValue, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *next.0.borrow() {
                        Value::Cons(ref car, ref cdr) => {
                            write!(f, " {:?}", car.0.borrow())?;
                            fmt_l(cdr.clone(), f)
                        }
                        Value::Nil => write!(f, ")"),
                        ref other => write!(f, " . {:?})", other),
                    }
                }
                fmt_l(cdr.clone(), f)
            },
            Value::Quoted(value) => write!(f, "'{:?}", value.0.borrow()),
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
                *self = cdr.to_value();
                Some(car.to_value())
            }
            other => {
                *self = Value::Nil;
                Some(other)
            }
        }
    }
}

