#[deriving(Clone, Show, PartialEq)]
pub enum Token<'a> {
    LeftParen, RightParen,
    Atom(&'a str),
    StringLiteral(&'a str),
    NumberLiteral(f64),
}

#[deriving(Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    index: uint,
    pub current: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            index: 0,
            current: None,
        }
    }

    /// Consume one token. Return previous token.
    pub fn consume(&mut self) -> Option<Token<'a>> {
        let previous = self.current.take();

        self.skip();

        if self.index < self.input.len() {
            self.current = self.read_token();
        }

        previous
    }
}

impl<'a> Lexer<'a> {
    fn skip(&mut self) {
        while self.index < self.input.len() {
            match self.input.char_at(self.index) {
                ' ' | '\t' | '\n' | '\r' => {
                    self.index += 1;
                }
                ';' => {
                    while self.index < self.input.len() && self.input.char_at(self.index) != '\n' {
                        self.index += 1;
                    }
                }
                _ => break,
            }
        }
    }

    fn read_token(&mut self) -> Option<Token<'a>> {
        match self.input.char_at(self.index) {
            '(' | ')' => self.read_paren(),
            '0'...'9' => self.read_number(),
            '"' => self.read_string(),
            _ => self.read_atom(),
        }
    }

    fn read_paren(&mut self) -> Option<Token<'a>> {
        let token = if self.input.char_at(self.index) == '(' {
            Some(Token::LeftParen)
        } else {
            Some(Token::RightParen)
        };

        self.index += 1;
        token
    }

    fn read_number(&mut self) -> Option<Token<'a>> {
        let start = self.index;

        while self.index < self.input.len() {
            match self.input.char_at(self.index) {
                '0'...'9' | '.' => {
                    self.index += 1;
                }
                _ => break,
            }
        }

        let value = self.input.slice(start, self.index);
        let value = from_str(value).unwrap();

        Some(Token::NumberLiteral(value))
    }

    fn read_string(&mut self) -> Option<Token<'a>> {
        self.index += 1;
        let start = self.index;

        while self.index < self.input.len()
            && self.input.char_at(self.index) != '"' {
            self.index += 1;
        }

        let value = self.input.slice(start, self.index);
        self.index += 1;

        Some(Token::StringLiteral(value))
    }

    fn read_atom(&mut self) -> Option<Token<'a>> {
        let start = self.index;

        while self.index < self.input.len() {
            match self.input.char_at(self.index) {
                ' ' | '\t' | '\n' | '\r' | '(' | ')' => break,
                _ => {
                    self.index += 1;
                }
            }
        }

        let name = self.input.slice(start, self.index);

        Some(Token::Atom(name))
    }
}

impl<'a> Iterator<Token<'a>> for Lexer<'a> {
    fn next(&mut self) -> Option<Token<'a>> {
        self.consume();
        self.current.clone()
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::{Lexer, Token};

    fn test_expr<'a>(expr: &'a str, expected_result: &[Token<'a>]) {
        let result: Vec<Token> = Lexer::new(expr).collect();

        if result.as_slice() != expected_result {
            panic!("Mismatch expression result for `{}`. Expected: {}, got: {}", expr, expected_result, result);
        }
    }

    #[test]
    fn test_simple() {
        test_expr("", &[]);
        test_expr("()", &[Token::LeftParen, Token::RightParen]);
        test_expr("(+ 1 1)", &[Token::LeftParen, Token::Atom("+"), Token::NumberLiteral(1.0), Token::NumberLiteral(1.0), Token::RightParen]);
        test_expr("(print \"hello\")", &[Token::LeftParen, Token::Atom("print"), Token::StringLiteral("hello"), Token::RightParen]);
        test_expr("(print a)", &[Token::LeftParen, Token::Atom("print"), Token::Atom("a"), Token::RightParen]);
    }

    #[test]
    fn test_comment() {
        test_expr("; toto", &[]);
        test_expr("(print   ; Something\n a)", &[Token::LeftParen, Token::Atom("print"), Token::Atom("a"), Token::RightParen]);
    }
}
