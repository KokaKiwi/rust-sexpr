use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LeftParen, RightParen,
    Ident(String),
    String(String),
    Number(f64),
}

pub struct Lexer<R: Read> {
    reader: BufReader<R>,
    current: Option<char>,
}

fn unescape_char(c: char) -> Option<char> {
    match c {
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        _ => None,
    }
}

fn unescape_str(src: &str) -> String {
    let mut dst = String::new();

    let mut chars = src.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(c) = chars.next() {
                    if let Some(c) = unescape_char(c) {
                        dst.push(c);
                    } else {
                        dst.push('\\');
                        dst.push(c);
                    }
                } else {
                    dst.push('\\');
                }
            }
            c => dst.push(c),
        }
    }

    dst
}

const SEPARATORS: &'static [char] = &[' ', '\t', '\r', '\n'];

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Lexer<R> {
        let mut lexer = Lexer {
            reader: BufReader::new(reader),
            current: None,
        };
        lexer.consume();
        lexer
    }

    pub fn current(&self) -> Option<char> {
        self.current
    }

    fn consume(&mut self) -> Option<char> {
        let current = self.current;
        self.current = self.reader.by_ref().chars().next().and_then(Result::ok);
        current
    }

    fn expect(&mut self, c: char) -> bool {
        if self.current() == Some(c) {
            self.consume();
            true
        } else {
            false
        }
    }

    fn read_token(&mut self) -> Option<Token> {
        self.skip_separators();

        let current = match self.current() {
            Some(c) => c,
            None => return None,
        };

        match current {
            '(' | ')' => self.read_paren(),
            '"' => self.read_string(),
            '0'...'9' => self.read_number(),
            _ => self.read_ident(),
        }
    }

    fn skip_separators(&mut self) {
        while let Some(c) = self.current() {
            match c {
                c if SEPARATORS.contains(&c) => {}
                _ => break,
            }

            self.consume();
        }
    }

    fn read_paren(&mut self) -> Option<Token> {
        match self.current() {
            Some('(') => {
                self.consume();
                Some(Token::LeftParen)
            }
            Some(')') => {
                self.consume();
                Some(Token::RightParen)
            }
            _ => None,
        }
    }

    fn read_string(&mut self) -> Option<Token> {
        self.consume();

        let mut value = String::new();

        while let Some(c) = self.current() {
            match c {
                '"' => break,
                c => value.push(c),
            }

            self.consume();
        }

        if !self.expect('"') {
            return None;
        }

        Some(Token::String(unescape_str(&value)))
    }

    fn read_number(&mut self) -> Option<Token> {
        let mut value = String::new();

        while let Some(c) = self.current() {
            match c {
                '0'...'9' | '.' => value.push(c),
                _ => break,
            }

            self.consume();
        }

        value.parse().map(|value| Token::Number(value)).ok()
    }

    fn read_ident(&mut self) -> Option<Token> {
        let mut name = String::new();

        while let Some(c) = self.current() {
            match c {
                ')' => break,
                c if SEPARATORS.contains(&c) => break,
                c => name.push(c),
            }

            self.consume();
        }

        Some(Token::Ident(name))
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.read_token()
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Lexer};

    fn assert_tokens(expr: &str, expected: &[Token]) {
        let lexer = Lexer::new(expr.as_bytes());
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_simples() {
        assert_tokens("()", &[
            Token::LeftParen,
            Token::RightParen,
        ]);

        assert_tokens("(f 42 \"hello\")", &[
            Token::LeftParen,
            Token::Ident("f".to_string()),
            Token::Number(42.0),
            Token::String("hello".to_string()),
            Token::RightParen,
        ]);
    }

    #[test]
    fn test_unary_list() {
        assert_tokens("(toto)", &[
            Token::LeftParen,
            Token::Ident("toto".to_string()),
            Token::RightParen,
        ]);
    }

    #[test]
    fn test_newline() {
        assert_tokens("(toto\ntata)", &[
            Token::LeftParen,
            Token::Ident("toto".to_string()),
            Token::Ident("tata".to_string()),
            Token::RightParen,
        ]);
    }
}
