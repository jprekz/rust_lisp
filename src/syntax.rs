use super::env::Env;
use super::eval::eval;
use super::parser::{Value, RefValue};

pub static SYNTAX: &'static [(&'static str, fn(&Value, &Env) -> Value)] = &[
    ("define", |arg, env| {
        let (car, cdr) = arg.try_into_cons().unwrap();
        match car {
            Value::Ident(ident) => {
                let value = eval(&cdr, env);
                env.insert(ident, value);
                Value::Bool(true)
            }
            Value::Cons(ident, args) => {
                let ident = ident.borrow().try_into_ident().unwrap();
                let args = args.clone();
                let (body, body_cdr) = cdr.try_into_cons().unwrap();
                let _ = body_cdr.try_into_nil().unwrap();
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
    ("cons", |arg, env| {
        arg.clone()
    }),
    ("+", |arg, env| {
        let mut acc = 0.0;
        while let Some((val, arg)) = arg.try_into_cons() {
            acc += val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("-", |arg, env| {
        let Some((val, arg)) = arg.try_into_cons();
        let mut acc = val.try_into_num().unwrap();
        while let Some((val, arg)) = arg.try_into_cons() {
            acc -= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("*", |arg, env| {
        let mut acc = 1.0;
        while let Some((val, arg)) = arg.try_into_cons() {
            acc *= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("/", |arg, env| {
        let Some((val, arg)) = arg.try_into_cons();
        let mut acc = val.try_into_num().unwrap();
        while let Some((val, arg)) = arg.try_into_cons() {
            acc /= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
];
