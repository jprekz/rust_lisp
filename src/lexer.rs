use std::iter::{Iterator, Peekable};

#[derive(Debug, Clone, PartialEq)]
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

pub trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}
impl<T: Iterator> PeekableIterator for Peekable<T> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }
}

pub type TokenStream = PeekableIterator<Item = Token>;

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
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut buf = String::new();
        while let Some(ch) = self.reader.next() {
            if is_identifier_char(ch) {
                buf.push(ch);
            }
            let peek = self.reader.peek().map(|o| o.clone());
            match (ch, peek) {
                ('(', _) => return Some(Token::LPER),
                (')', _) => return Some(Token::RPER),
                ('{', _) => return Some(Token::LBRACE),
                ('}', _) => return Some(Token::RBRACE),
                ('\'', _) => return Some(Token::QUOTE),
                ('.', None) => return Some(Token::DOT),
                ('.', Some(peek)) if !is_identifier_char(peek) => return Some(Token::DOT),
                ('#', Some(_)) => match self.reader.next().unwrap() {
                    't' => return Some(Token::BOOL(true)),
                    'f' => return Some(Token::BOOL(false)),
                    _ => panic!("lexer error"),
                },
                (_, _) if is_identifier_char(ch) => {
                    if let Some(peek) = peek {
                        if is_identifier_char(peek) {
                            continue;
                        }
                    }
                    if let Ok(num) = buf.parse() {
                        return Some(Token::NUM(num));
                    } else {
                        return Some(Token::IDENT(buf));
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
