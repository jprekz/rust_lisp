use crate::lexer::Token;
use crate::value::{RefValue, Value};

use std::iter::Peekable;

pub fn parse<T>(token_stream: &mut Peekable<T>) -> Result<Value, String>
where
    T: Iterator<Item = Result<Token, String>>,
{
    let first_token = match token_stream.next() {
        Some(item) => item?,
        None => return Err("unexpected end of input".to_string()),
    };

    let value = match first_token {
        Token::LPER => {
            if let Some(Ok(Token::RPER)) = token_stream.peek().cloned() {
                token_stream.next();
                Value::Null
            } else {
                parse_list(token_stream)?
            }
        }
        Token::BOOL(b) => Value::Bool(b),
        Token::IDENT(ident) => Value::Ident(ident),
        Token::NUM(num) => Value::Num(num),
        Token::QUOTE => {
            let quoted = parse(token_stream)?;
            Value::Cons(
                RefValue::new(Value::Ident("quote".to_string())),
                RefValue::new(Value::Cons(
                    RefValue::new(quoted),
                    RefValue::new(Value::Null),
                )),
            )
        }
        _ => return Err("syntax error".to_string()),
    };

    Ok(value)
}

fn parse_list<T>(token_stream: &mut Peekable<T>) -> Result<Value, String>
where
    T: Iterator<Item = Result<Token, String>>,
{
    let mut tail = RefValue::new(Value::Null);
    let head = parse(token_stream)?;
    let head = Value::Cons(RefValue::new(head), tail.clone());
    while let Some(peek) = token_stream.peek().cloned() {
        match peek? {
            Token::RPER => {
                token_stream.next();
                return Ok(head);
            }
            Token::DOT => {
                token_stream.next();
                let value = parse(token_stream)?;
                tail.replace(value);
                if let Some(next) = token_stream.next() {
                    if let Token::RPER = next? {
                        return Ok(head);
                    }
                } else {
                    return Err("syntax error".to_string());
                }
            }
            _ => {
                let value = parse(token_stream)?;
                let next_tail = RefValue::new(Value::Null);
                tail.replace(Value::Cons(RefValue::new(value), next_tail.clone()));
                tail = next_tail;
            }
        }
    }
    return Err("syntax error".to_string());
}
