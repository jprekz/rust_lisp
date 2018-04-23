use super::env::Env;
use super::eval::eval;
use super::value::{Value, RefValue};

pub static SYNTAX: &'static [(&'static str, fn(Value, &Env) -> Value)] = &[
    ("define", |mut arg, env| {
        match arg.next().unwrap() {
            Value::Ident(ident) => {
                let value = eval(arg.next().unwrap(), env);
                env.insert(ident, value);
                Value::Bool(true)
            }
            Value::Cons(ident, args) => {
                let ident = ident.to_value().try_into_ident().unwrap();
                let body = arg.next().unwrap();
                let _ = arg.try_into_nil().unwrap();
                let value = Value::Closure(args, RefValue::new(body), env.clone());
                env.insert(ident, value);
                Value::Bool(true)
            }
            _ => panic!("syntax error"),
        }
    }),
    ("quote", |arg, _env| arg.clone()),
    ("lambda", |arg, env| {
        let (car, cdr) = arg.try_into_cons().unwrap();
        let args = car.clone();
        let (car, cdr) = cdr.try_into_cons().unwrap();
        let body = car.clone();
        let _ = cdr.try_into_nil().unwrap();
        Value::Closure(RefValue::new(args), RefValue::new(body), env.clone())
    }),
    ("cons", |arg, _env| {
        arg.clone()
    }),
    ("+", |mut arg, _env| {
        let mut acc = 0.0;
        while let Some((val, next_arg)) = arg.try_into_cons() {
            arg = next_arg;
            acc += val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("-", |arg, _env| {
        let (val, mut arg) = arg.try_into_cons().unwrap();
        let mut acc = val.try_into_num().unwrap();
        while let Some((val, next_arg)) = arg.try_into_cons() {
            arg = next_arg;
            acc -= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("*", |mut arg, _env| {
        let mut acc = 1.0;
        while let Some((val, next_arg)) = arg.try_into_cons() {
            arg = next_arg;
            acc *= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("/", |arg, _env| {
        let (val, mut arg) = arg.try_into_cons().unwrap();
        let mut acc = val.try_into_num().unwrap();
        while let Some((val, next_arg)) = arg.try_into_cons() {
            arg = next_arg;
            acc /= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
];
