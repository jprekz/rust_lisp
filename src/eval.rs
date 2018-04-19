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
