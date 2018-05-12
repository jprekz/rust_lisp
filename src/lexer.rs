use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Token {
    LPER,
    RPER,
    LBRACE,
    RBRACE,
    QUOTE,
    DOT,
    BOOL(bool),
    IDENT(String),
    NUM(f64),
}

pub struct Lexer<C: Iterator<Item = char>> {
    reader: Peekable<C>,
}
impl<C: Iterator<Item = char>> Lexer<C> {
    pub fn new(reader: C) -> Lexer<C> {
        Lexer {
            reader: reader.peekable(),
        }
    }
}
impl<C: Iterator<Item = char>> Iterator for Lexer<C> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        while let Some(ch) = self.reader.next() {
            if is_identifier_char(ch) {
                buf.push(ch);
            }
            let peek = self.reader.peek().cloned();
            match (ch, peek) {
                ('(', _) => return Some(Ok(Token::LPER)),
                (')', _) => return Some(Ok(Token::RPER)),
                ('{', _) => return Some(Ok(Token::LBRACE)),
                ('}', _) => return Some(Ok(Token::RBRACE)),
                ('\'', _) => return Some(Ok(Token::QUOTE)),
                ('.', None) => return Some(Ok(Token::DOT)),
                ('.', Some(peek)) if !is_identifier_char(peek) => return Some(Ok(Token::DOT)),
                ('#', Some(_)) => match self.reader.next().unwrap() {
                    't' => return Some(Ok(Token::BOOL(true))),
                    'f' => return Some(Ok(Token::BOOL(false))),
                    _ => return Some(Err("lexer error".to_string())),
                },
                (_, _) if is_identifier_char(ch) => {
                    if let Some(peek) = peek {
                        if is_identifier_char(peek) {
                            continue;
                        }
                    }
                    if let Ok(num) = buf.parse() {
                        return Some(Ok(Token::NUM(num)));
                    } else {
                        return Some(Ok(Token::IDENT(buf)));
                    }
                }
                _ => (),
            }
        }
        None
    }
}

// return true if ch is one of extended identifier characters:
// ! $ % & * + - . / : < = > ? @ ^ _ ~
fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || //
    ('-' <= ch && ch <= ':') || //
    ('<' <= ch && ch <= '@') || //
    ch == '_' ||                //
    ch == '*' ||                //
    ch == '+' ||                //
    ch == '!' ||                //
    ch == '$' ||                //
    ch == '%' ||                //
    ch == '&' ||                //
    ch == '^' ||                // disable auto-format
    ch == '~'
}
