use super::env::Env;
use super::eval::eval;
use super::value::{Value, RefValue};

pub static SYNTAX: &'static [(&'static str, fn(Value, Env) -> Value)] = &[
    ("define", |mut args, env| {
        match args.next().unwrap() {
            Value::Ident(ident) => {
                let value = eval(args.next().unwrap(), env.clone());
                env.insert(ident, value);
                Value::Bool(true)
            }
            Value::Cons(defun_ident, defun_args) => {
                let body = args.next().unwrap();
                let _ = args.try_into_nil().unwrap();
                let defun_ident = defun_ident.to_value().try_into_ident().unwrap();
                let value = Value::Closure(defun_args, RefValue::new(body), env.clone());
                env.insert(defun_ident, value);
                Value::Bool(true)
            }
            _ => panic!("syntax error"),
        }
    }),
    ("quote", |mut args, _env| args.next().unwrap().clone()),
    ("lambda", |args, env| {
        let (car, cdr) = args.try_into_cons().unwrap();
        let args = car.clone();
        let (car, cdr) = cdr.try_into_cons().unwrap();
        let body = car.clone();
        let _ = cdr.try_into_nil().unwrap();
        Value::Closure(RefValue::new(args), RefValue::new(body), env.clone())
    }),
    ("if", |mut args, env| {
        let cond = args.next().unwrap();
        let t = args.next().unwrap();
        let f = args.next().unwrap();
        if let Some(false) = cond.try_into_bool() {
            f
        } else {
            t
        }
    }),
];

pub static SUBR: &'static [(&'static str, fn(&mut Iterator<Item=Value>) -> Value)] = &[
    ("cons", |args| {
        let car = args.next().unwrap();
        let cdr = args.next().unwrap();
        Value::Cons(RefValue::new(car), RefValue::new(cdr))
    }),
    ("=", |args| {
        let first = args.next().unwrap();
        for val in args {
            if first != val {
                return Value::Bool(false)
            }
        }
        Value::Bool(true)
    }),
    ("+", |args| {
        let mut acc = 0.0;
        for val in args {
            acc += val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("-", |args| {
        let mut acc = args.next().unwrap().try_into_num().unwrap();
        for val in args {
            acc -= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("*", |args| {
        let mut acc = 1.0;
        for val in args {
            acc *= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("/", |args| {
        let mut acc = args.next().unwrap().try_into_num().unwrap();
        for val in args {
            acc /= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
];
