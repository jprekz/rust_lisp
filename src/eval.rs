use super::env::Env;
use super::value::{Value, RefValue, SyntaxFn};

/*
pub fn eval(value: Value, env: Env) -> Value {
    println!("start eval");
    let mut stack = Vec::new();
    stack.push(StackFrame {
        func: Value::Syntax("vreturn", SyntaxFn::new(|mut v, _| v.next().unwrap())),
        vp: Value::Cons(RefValue::new(value), RefValue::new(Value::Nil)),
        arg: Vec::new(),
        env: env,
    });
    loop {
        println!("{:?}", stack);
        let mut stack_p = stack.pop().unwrap();
        if stack_p.vp.clone().count() == 0 {
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
                    arg: Vec::new(),
                    env: extended_env,
                });
            } else if let Value::Syntax(_name, f) = stack_p.func.clone() {
                if let Some(mut stack_pp) = stack.last_mut() {
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
    arg: Vec<Value>,
    env: Env,
}
*/
fn stack_to_list(v: &[StackData]) -> Value {
    let mut list = Value::Nil;
    for value in v {
        if let StackData::Val(value) = value {
            list = Value::Cons(RefValue::new(value.clone()), RefValue::new(list));
        } else {
            panic!();
        }
    }
    list
}

#[derive(Clone, Debug)]
enum StackData {
    Val(Value),
    PP(Value),
    SP(i64),
    Env(Env),
}
pub fn eval(mut pp: Value, mut env: Env) -> Value {
    println!("eval start");
    let mut rr = Value::Nil;
    let mut sp = 0i64;
    let mut stack: Vec<StackData> = Vec::new();
    loop {
        println!("rr:{:?} sp:{} pp:{:?} {:?}", rr, sp, pp, stack);
        if let Some(next) = pp.next() {
            match next {
                Value::Cons(_, _) => {
                    stack.push(StackData::SP(sp));
                    stack.push(StackData::PP(pp));
                    sp = stack.len() as i64;
                    pp = next;
                }
                Value::Quoted(value) => {
                    stack.push(StackData::Val(value.to_value()));
                }
                Value::Ident(ident) => {
                    stack.push(StackData::Val(env.get(ident.clone()).expect("unbound variable")));
                }
                other => {
                    stack.push(StackData::Val(other));
                }
            }
        } else {
            if sp < 0 {
                return rr;
            }
            if stack.len() == sp as usize {
                stack.push(StackData::Val(Value::Nil));
            }
            match stack[sp as usize].clone() {
                StackData::Val(Value::Closure(closure_args, closure_body, closure_env)) => {
                    let mut extended_env = closure_env.extend();
                    for (i, closure_arg) in closure_args.to_value().enumerate() {
                        let ident = closure_arg.try_into_ident().expect("syntax error");
                        if let StackData::Val(value) = stack[sp as usize + 1 + i].clone() {
                            extended_env.insert(ident, value);
                        } else {
                            panic!();
                        }
                    }
                    while stack.len() > sp as usize { stack.pop(); }
                    stack.push(StackData::Env(env.clone()));
                    pp = closure_body.to_value();
                    env = extended_env;
                }
                StackData::Val(Value::Syntax(name, f)) => {
                    println!(" exec syntax {}", name);
                    rr = f(stack_to_list(&stack[sp as usize + 1 ..]), env.clone());
                    while stack.len() > sp as usize { stack.pop(); }
                    sp -= 1;
                }
                StackData::Val(Value::Subr(_name, f)) => {
                    rr = f(&mut stack[sp as usize + 1 ..].iter().map(|d| {
                        if let StackData::Val(v) = d { v.clone() } else { panic!() }
                    }));
                    while stack.len() > sp as usize { stack.pop(); }
                    sp -= 1;
                }
                StackData::Val(value) => {
                    rr = value;
                    sp -= 1;
                }
                StackData::SP(v) => {
                    println!("unexcepted [SP] ...");
                    while stack.len() > sp as usize { stack.pop(); }
                    stack.push(StackData::Val(rr.clone()));
                    sp = v;
                }
                StackData::PP(v) => {
                    pp = v;
                    sp -= 1;
                    if let StackData::SP(v) = stack[sp as usize] {
                        while stack.len() > sp as usize { stack.pop(); }
                        stack.push(StackData::Val(rr.clone()));
                        sp = v;
                    }
                }
                StackData::Env(e) => {
                    env = e;
                    sp -= 1;
                }
            }
        }
    }
}

