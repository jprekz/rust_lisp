use super::env::Env;
use super::eval::eval;
use super::value::{Value, RefValue};

pub static SYNTAX: &'static [(&'static str, fn(Value, &Env) -> Value)] = &[
    ("define", |mut args, env| {
        match args.next().unwrap() {
            Value::Ident(ident) => {
                let value = eval(args.next().unwrap(), env);
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
    ("quote", |args, _env| args.clone()),
    ("lambda", |args, env| {
        let (car, cdr) = args.try_into_cons().unwrap();
        let args = car.clone();
        let (car, cdr) = cdr.try_into_cons().unwrap();
        let body = car.clone();
        let _ = cdr.try_into_nil().unwrap();
        Value::Closure(RefValue::new(args), RefValue::new(body), env.clone())
    }),
    ("if", |mut args, env| {
        let cond = eval(args.next().unwrap(), env);
        let t = args.next().unwrap();
        let f = args.next().unwrap();
        if let Some(false) = cond.try_into_bool() {
            eval(f, env)
        } else {
            eval(t, env)
        }
    }),
    ("cons", |mut args, env| {
        let car = eval(args.next().unwrap(), env);
        let cdr = eval(args.next().unwrap(), env);
        Value::Cons(RefValue::new(car), RefValue::new(cdr))
    }),
    ("=", |mut args, env| {
        let first = args.next().map(|arg| eval(arg, env)).unwrap();
        for val in args.map(|arg| eval(arg, env)) {
            if first != val {
                return Value::Bool(false)
            }
        }
        Value::Bool(true)
    }),
    ("+", |args, env| {
        let mut acc = 0.0;
        for val in args.map(|arg| eval(arg, env)) {
            acc += val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("-", |args, env| {
        let mut args = args.map(|arg| eval(arg, env));
        let mut acc = args.next().unwrap().try_into_num().unwrap();
        for val in args.map(|arg| eval(arg, env)) {
            acc -= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("*", |args, env| {
        let mut acc = 1.0;
        for val in args.map(|arg| eval(arg, env)) {
            acc *= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("/", |args, env| {
        let mut args = args.map(|arg| eval(arg, env));
        let mut acc = args.next().unwrap().try_into_num().unwrap();
        for val in args.map(|arg| eval(arg, env)) {
            acc /= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
];
