use super::env::Env;
use super::value::Value;

pub fn eval(value: Value, env: &Env) -> Value {
    match value {
        Value::Cons(car, cdr) => {
            let func = eval(car.to_value(), env);
            let mut args = cdr.to_value();
            match func {
                Value::Syntax(_name, f) => {
                    f(args, env)
                }
                Value::Closure(closure_args, closure_body, closure_env) => {
                    let mut extended_env = closure_env.extend();
                    for closure_arg in closure_args.to_value() {
                        let ident = closure_arg.try_into_ident().expect("syntax error");
                        let value = eval(args.next().expect("syntax error"), env);
                        extended_env.insert(ident, value);
                    }
                    eval(closure_body.to_value(), &extended_env)
                }
                other => panic!("unexcepted value: {:?}", other)
            }
        }
        Value::Quoted(value) => value.to_value(),
        Value::Ident(ident) => {
            match env.get(ident.clone()) {
                Some(value) => value,
                None => panic!("unbound variable: {}", ident)
            }
        },
        other => other,
    }
}
