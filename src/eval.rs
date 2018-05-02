use super::env::Env;
use super::value::{Value, RefValue, SyntaxFn};

pub fn eval(value: Value, env: Env) -> Value {
    println!("start eval");
    let mut stack = Vec::new();
    stack.push(StackFrame {
        func: Value::Syntax("vreturn", SyntaxFn::new(|mut v, _| v.next().unwrap())),
        vp: Value::Cons(RefValue::new(value), RefValue::new(Value::Nil)),
        remain: 1,
        arg: Vec::new(),
        env: env,
    });
    loop {
        println!("{:?}", stack);
        let mut stack_p = stack.pop().unwrap();
        if stack_p.remain == 0 {
            if let Value::Closure(closure_args, closure_body, closure_env) = stack_p.func.clone() {
                let mut extended_env = closure_env.extend();
                for (i, closure_arg) in closure_args.to_value().enumerate() {
                    let ident = closure_arg.try_into_ident().expect("syntax error");
                    let value = stack_p.arg[i].clone();
                    extended_env.insert(ident, value);
                }
                stack.push(StackFrame {
                    func: Value::Syntax("vreturn", SyntaxFn::new(|mut v, _| v.next().unwrap())),
                    vp: Value::Cons(RefValue::new(closure_body.to_value()), RefValue::new(Value::Nil)),
                    remain: 1,
                    arg: Vec::new(),
                    env: extended_env,
                });
            } else if let Value::Syntax(_name, f) = stack_p.func.clone() {
                if let Some(mut stack_pp) = stack.last_mut() {
                    stack_pp.remain -= 1;
                    stack_pp.arg.push(f(stack_p.vp, stack_p.env));
                } else {
                    return f(vec_to_list(stack_p.arg), stack_p.env);
                }
            } else {
                panic!("unexcepted value: {:?}", stack_p.func.clone());
            }
        } else {
            let v = stack_p.vp.next().unwrap();
            let ret_value = match v {
                Value::Cons(car, cdr) => {
                    let func = eval(car.to_value(), stack_p.env.clone());
                    let mut args = cdr.to_value();
                    if let Value::Syntax(_name, f) = func {
                        Some(f(args, stack_p.env.clone()))
                    } else {
                        stack.push(StackFrame {
                            func: func,
                            vp: args.clone(),
                            remain: args.count() as u32,
                            arg: Vec::new(),
                            env: stack_p.env.clone(),
                        });
                        None
                    }
                }
                Value::Quoted(value) => Some(value.to_value()),
                Value::Ident(ident) => Some(stack_p.env.get(ident.clone()).expect("unbound variable")),
                other => Some(other),
            };
            if let Some(ret_value) = ret_value {
                stack_p.remain -= 1;
                stack_p.arg.push(ret_value);
                stack.push(stack_p);
            }
        }
    }
}
#[derive(Debug)]
struct StackFrame {
    func: Value,
    vp: Value,
    remain: u32,
    arg: Vec<Value>,
    env: Env,
}

fn vec_to_list(v: Vec<Value>) -> Value {
    let mut list = Value::Nil;
    for value in v {
        list = Value::Cons(RefValue::new(value), RefValue::new(list));
    }
    list
}
