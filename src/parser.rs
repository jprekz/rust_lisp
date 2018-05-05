use super::lexer::{Token, TokenStream};
use super::value::{RefValue, Value};

pub fn parse(token_stream: &mut TokenStream) -> Value {
    match (token_stream.next(), token_stream.peek().cloned()) {
        (Some(Token::LPER), Some(Token::RPER)) => {
            token_stream.next();
            return Value::Null;
        }
        (Some(Token::LPER), _) => (),
        (Some(Token::BOOL(b)), _) => {
            return Value::Bool(b);
        }
        (Some(Token::IDENT(ident)), _) => {
            return Value::Ident(ident);
        }
        (Some(Token::NUM(num)), _) => {
            return Value::Num(num);
        }
        (Some(Token::QUOTE), _) => {
            return Value::Cons(
                RefValue::new(Value::Ident("quote".to_string())),
                RefValue::new(Value::Cons(
                    RefValue::new(parse(token_stream)),
                    RefValue::new(Value::Null),
                )),
            );
        }
        _ => panic!("syntax error"),
    }
    let mut tail = RefValue::new(Value::Null);
    let head = Value::Cons(RefValue::new(parse(token_stream)), tail.clone());
    while let Some(peek) = token_stream.peek().cloned() {
        match peek {
            Token::RPER => {
                token_stream.next();
                return head;
            }
            Token::DOT => {
                token_stream.next();
                let value = parse(token_stream);
                tail.replace(value);
                if let Some(Token::RPER) = token_stream.next() {
                    return head;
                } else {
                    panic!("syntax error");
                }
            }
            _ => {
                let value = parse(token_stream);
                let next_tail = RefValue::new(Value::Null);
                tail.replace(Value::Cons(RefValue::new(value), next_tail.clone()));
                tail = next_tail;
            }
        }
    }
    panic!("syntax error");
}
