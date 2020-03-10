use crate::env::Env;
use crate::eval::VM;

use std::cell::RefCell;
use std::rc::Rc;

pub type BuiltinFn = fn(&mut VM) -> Result<(), String>;

#[derive(Clone)]
pub enum Value {
    Null,
    Cons(RefValue, RefValue),
    Bool(bool),
    Num(f64),
    Ident(String),
    Syntax(&'static str, BuiltinFn),
    Closure(RefValue, RefValue, Env),
    Subr(&'static str, BuiltinFn),
    Cont(Box<VM>),
}
impl Value {
    pub fn try_into_nil(self) -> Result<(), String> {
        match self {
            Value::Null => Ok(()),
            _ => Err("type mismatch".to_string()),
        }
    }
    pub fn try_into_cons(self) -> Result<(Value, Value), String> {
        match self {
            Value::Cons(car, cdr) => Ok((car.to_value(), cdr.to_value())),
            _ => Err("type mismatch".to_string()),
        }
    }
    pub fn try_into_bool(self) -> Result<bool, String> {
        match self {
            Value::Bool(b) => Ok(b),
            _ => Err("type mismatch".to_string()),
        }
    }
    pub fn try_into_num(self) -> Result<f64, String> {
        match self {
            Value::Num(a) => Ok(a),
            _ => Err("type mismatch".to_string()),
        }
    }
    pub fn try_into_ident(self) -> Result<String, String> {
        match self {
            Value::Ident(ident) => Ok(ident),
            _ => Err("type mismatch".to_string()),
        }
    }

    pub fn into_list_iter(self) -> impl Iterator<Item = Value> {
        ListIterator(self)
    }
}

pub struct ListIterator(Value);
impl Iterator for ListIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let (car, cdr) = match &self.0 {
            Value::Cons(car, cdr) => (car.to_value(), cdr.to_value()),
            _ => return None,
        };
        self.0 = cdr;
        Some(car)
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Value::Null => write!(f, "()"),
            Value::Cons(car, cdr) => {
                write!(f, "({:?}", car.0.borrow())?;
                fn fmt_l(next: RefValue, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match *next.0.borrow() {
                        Value::Cons(ref car, ref cdr) => {
                            write!(f, " {:?}", car.0.borrow())?;
                            fmt_l(cdr.clone(), f)
                        }
                        Value::Null => write!(f, ")"),
                        ref other => write!(f, " . {:?})", other),
                    }
                }
                fmt_l(cdr.clone(), f)
            }
            Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Num(num) => write!(f, "{}", num),
            Value::Ident(ident) => write!(f, "{}", ident),
            Value::Syntax(name, _) => write!(f, "#<syntax {}>", name),
            Value::Closure(a, b, _) => write!(f, "#<closure {:?} {:?}>", a, b),
            Value::Subr(name, _) => write!(f, "#<subr {}>", name),
            Value::Cont(_vm) => write!(f, "#<subr continuation>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Cons(car1, cdr1), Value::Cons(car2, cdr2)) => car1 == car2 && cdr1 == cdr2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Num(n1), Value::Num(n2)) => n1 == n2,
            (Value::Ident(i1), Value::Ident(i2)) => i1 == i2,
            (Value::Syntax(n1, f1), Value::Syntax(n2, f2)) => n1 == n2 && ::std::ptr::eq(f1, f2),
            (Value::Closure(a1, b1, e1), Value::Closure(a2, b2, e2)) => {
                a1 == a2 && b1 == b2 && e1 == e2
            }
            (Value::Subr(n1, f1), Value::Subr(n2, f2)) => n1 == n2 && ::std::ptr::eq(f1, f2),
            _ => false,
        }
    }
}

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
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0.borrow())
    }
}
