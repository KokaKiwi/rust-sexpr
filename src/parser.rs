use std::io::prelude::*;
use lexer::{Lexer, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    List(Vec<Node>),
    Atom(String),
    String(String),
    Number(f64),
}

pub struct Parser<R: Read> {
    lexer: Lexer<R>,
    current: Option<Token>,
}

impl<R: Read> Parser<R> {
    pub fn new(lexer: Lexer<R>) -> Parser<R> {
        let mut parser = Parser {
            lexer: lexer,
            current: None,
        };
        parser.consume();
        parser
    }

    pub fn current(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn consume(&mut self) -> Option<Token> {
        let current = self.current.take();
        self.current = self.lexer.next();
        current
    }

    fn expect(&mut self, token: Token) -> bool {
        if self.current() == Some(&token) {
            self.consume();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Option<Node> {
        self.parse_node()
    }

    fn parse_node(&mut self) -> Option<Node> {
        let current = match self.current.take() {
            Some(token) => token,
            None => return None,
        };

        match current {
            Token::LeftParen => self.parse_list(),
            Token::Ident(name) => {
                self.consume();
                Some(Node::Atom(name))
            }
            Token::String(value) => {
                self.consume();
                Some(Node::String(value))
            }
            Token::Number(value) => {
                self.consume();
                Some(Node::Number(value))
            }
            _ => {
                self.current = Some(current);
                None
            }
        }
    }

    fn parse_list(&mut self) -> Option<Node> {
        self.consume();

        let mut items = Vec::new();

        while let Some(item) = self.parse_node() {
            items.push(item);
        }

        if !self.expect(Token::RightParen) {
            return None;
        }

        Some(Node::List(items))
    }
}

impl<R: Read> Iterator for Parser<R> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        self.parse()
    }
}

#[cfg(test)]
mod test {
    use lexer::Lexer;
    use super::{Parser, Node};

    fn assert_expr(expr: &str, expected: Option<Node>) {
        let lexer = Lexer::new(expr.as_bytes());
        let mut parser = Parser::new(lexer);

        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_simple() {
        assert_expr("", None);
        assert_expr("()", Some(Node::List(vec![])));
        assert_expr("(f 42 \"hello\")", Some(
            Node::List(vec![
                Node::Atom("f".to_string()),
                Node::Number(42.0),
                Node::String("hello".to_string()),
            ])
        ));
    }
}
