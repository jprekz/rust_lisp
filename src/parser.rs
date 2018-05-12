use super::lexer::{Token};
use super::value::{RefValue, Value};

use std::iter::Peekable;
pub fn parse<T>(token_stream: &mut Peekable<T>) -> Result<Value, String>
where T: Iterator<Item = Result<Token, String>> {
    match token_stream.next().transpose()? {
        Some(Token::LPER) => {
            if let Some(Token::RPER) = token_stream.peek().cloned().transpose()? {
                token_stream.next();
                return Ok(Value::Null);
            }
        }
        Some(Token::BOOL(b)) => {
            return Ok(Value::Bool(b));
        }
        Some(Token::IDENT(ident)) => {
            return Ok(Value::Ident(ident));
        }
        Some(Token::NUM(num)) => {
            return Ok(Value::Num(num));
        }
        Some(Token::QUOTE) => {
            let quoted = parse(token_stream)?;
            return Ok(Value::Cons(
                RefValue::new(Value::Ident("quote".to_string())),
                RefValue::new(Value::Cons(
                    RefValue::new(quoted),
                    RefValue::new(Value::Null),
                )),
            ));
        }
        None => return Err("unexpected end of input".to_string()),
        _ => return Err("syntax error".to_string()),
    }
    let mut tail = RefValue::new(Value::Null);
    let head = parse(token_stream)?;
    let head = Value::Cons(RefValue::new(head), tail.clone());
    while let Some(peek) = token_stream.peek().cloned().transpose()? {
        match peek {
            Token::RPER => {
                token_stream.next();
                return Ok(head);
            }
            Token::DOT => {
                token_stream.next();
                let value = parse(token_stream)?;
                tail.replace(value);
                if let Some(Token::RPER) = token_stream.next().transpose()? {
                    return Ok(head);
                } else {
                    panic!("syntax error");
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
