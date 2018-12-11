use std::iter::Peekable;

/// Lexical token
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

/// Lisp lexer
pub struct Lexer<C: Iterator<Item = char>> {
    reader: Peekable<C>,
}
impl<C: Iterator<Item = char>> Lexer<C> {
    /// Create a new lexer that consumes `reader`.
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
            let token = match (ch, peek) {
                ('(', _) => Token::LPER,
                (')', _) => Token::RPER,
                ('{', _) => Token::LBRACE,
                ('}', _) => Token::RBRACE,
                ('\'', _) => Token::QUOTE,
                ('.', None) => Token::DOT,
                ('.', Some(peek)) if !is_identifier_char(peek) => Token::DOT,
                ('#', Some(_)) => match self.reader.next().unwrap() {
                    't' => Token::BOOL(true),
                    'f' => Token::BOOL(false),
                    _ => return Some(Err("lexer error".to_string())),
                },
                (_, _) if is_identifier_char(ch) => {
                    if let Some(peek) = peek {
                        if is_identifier_char(peek) {
                            continue;
                        }
                    }
                    if let Ok(num) = buf.parse() {
                        Token::NUM(num)
                    } else {
                        Token::IDENT(buf)
                    }
                }
                _ => continue,
            };
            return Some(Ok(token));
        }
        None
    }
}

/// return true if `ch` is one of extended identifier characters:
/// ```! $ % & * + - . / : < = > ? @ ^ _ ~```
#[rustfmt::skip]
fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() ||
    ('-' <= ch && ch <= ':') ||
    ('<' <= ch && ch <= '@') ||
    ch == '_' ||
    ch == '*' ||
    ch == '+' ||
    ch == '!' ||
    ch == '$' ||
    ch == '%' ||
    ch == '&' ||
    ch == '^' ||
    ch == '~'
}
