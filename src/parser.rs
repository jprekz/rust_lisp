use super::lexer::{Token, TokenStream};
use super::value::{Value, RefValue};

pub fn parse(token_stream: &mut TokenStream) -> Value {
    match token_stream.next() {
        Some(Token::LPER) => (),
        Some(Token::QUOTE) => {
            return Value::Quoted(RefValue::new(parse(token_stream)));
        }
        Some(Token::BOOL(b)) => {
            return Value::Bool(b);
        }
        Some(Token::IDENT(ident)) => {
            return Value::Ident(ident);
        }
        Some(Token::NUM(num)) => {
            return Value::Num(num);
        }
        _ => panic!("syntax error"),
    }
    if let Some(Token::IDENT(ident)) = token_stream.peek().map(|c| c.clone()) {
        if ident.eq("quote") {
            token_stream.next();
            let value = Value::Quoted(RefValue::new(parse(token_stream)));
            if let Some(Token::RPER) = token_stream.next() {
                return value;
            } else {
                panic!("syntax error");
            }
        }
    }
    let mut tail = RefValue::new(Value::Nil);
    let head = Value::Cons(RefValue::new(parse(token_stream)), tail.clone());
    while let Some(peek) = token_stream.peek().map(|c| c.clone()) {
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
                let next_tail = RefValue::new(Value::Nil);
                tail.replace(Value::Cons(RefValue::new(value), next_tail.clone()));
                tail = next_tail;
            }
        }
    }
    panic!("syntax error");
}
