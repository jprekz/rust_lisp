use super::env::Env;
use super::lexer::{TokenStream, Token};
use super::parser::{parse, Value};

pub fn eval(token_stream: &mut TokenStream, env: &Env) -> Value {
    match token_stream.next().unwrap() {
        Token::LPER => {
            if let Some(Token::RPER) = token_stream.peek() {
                token_stream.next();
                return Value::Nil;
            }
            let value = match eval(token_stream, env) {
                Value::Syntax(_name, f) => f(token_stream, env),
                Value::Subr(_name, f) => {
                    let mut eval_iter = EvalIter {
                        token_stream,
                        env,
                    };
                    f(&eval_iter)
                },
                Value::Closure(args, body, closure_env) => {
                    let mut extended_env = closure_env.extend();
                    for arg in args {
                        let value = eval(token_stream, env);
                        extended_env.insert(arg, value);
                    }
                    eval(&mut body.into_iter().peekable(), &extended_env)
                }
                _ => panic!("invalid application"),
            };
            if let Some(Token::RPER) = token_stream.next() {
                return value;
            } else {
                panic!("syntax error");
            }
        }
        Token::RPER => panic!("syntax error"),
        Token::LBRACE => panic!("syntax error"),
        Token::RBRACE => panic!("syntax error"),
        Token::QUOTE => parse(token_stream),
        Token::DOT => panic!("syntax error"),
        Token::BOOL(b) => Value::Bool(b),
        Token::IDENT(ident) => {
            if let Some(value) = env.get(ident.clone()) {
                value.clone()
            } else {
                panic!("unbound variable: {}", ident);
            }
        }
        Token::NUM(num) => Value::Num(num),
    }
}

pub struct EvalIter<'a> {
    token_stream: &'a mut TokenStream,
    env: &'a Env,
}
impl<'a> Iterator for EvalIter<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        match self.token_stream.peek() {
            Some(Token::RPER) => return None,
            Some(Token::DOT) => return None,
            None => return None,
            _ => (),
        }
        Some(eval(self.token_stream, self.env))
    }
}

pub fn eval_value(value: Value, env: &Env) -> Value {
    match value {
        Value::Cons(car, cdr) => car.borrow().clone(),
        Value::Quoted(value) => value.borrow().clone(),
        Value::Ident(ident) => {
            env.get(ident).unwrap()
        },
        other => other,
    }
}
