use lexer::{Lexer, Token};

#[deriving(Clone, Show, PartialEq)]
pub enum Node<'a> {
    Atom(&'a str),
    StringLiteral(&'a str),
    NumberLiteral(f64),
    List(Vec<Node<'a>>),
}

impl<'a> Node<'a> {
    pub fn to_src(&self) -> String {
        match *self {
            Node::Atom(name) => name.to_string(),
            Node::StringLiteral(value) => format!("\"{}\"", value),
            Node::NumberLiteral(value) => format!("{}", value),
            Node::List(ref items) => {
                let items: Vec<String> = items.iter().map(|item| item.to_src()).collect();
                format!("({})", items.connect(" "))
            }
        }
    }
}

#[deriving(Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let mut lexer = Lexer::new(input);
        lexer.consume();

        Parser {
            lexer: lexer,
        }
    }

    pub fn parse(&mut self) -> Option<Node<'a>> {
        match self.lexer.current {
            Some(Token::LeftParen) => {
                self.lexer.consume();

                let items = self.parse_list();

                match self.lexer.current {
                    Some(Token::RightParen) => {
                        self.lexer.consume();
                        Some(Node::List(items))
                    }
                    _ => return None,
                }
            }
            Some(Token::Atom(name)) => {
                self.lexer.consume();
                Some(Node::Atom(name))
            }
            Some(Token::StringLiteral(value)) => {
                self.lexer.consume();
                Some(Node::StringLiteral(value))
            }
            Some(Token::NumberLiteral(value)) => {
                self.lexer.consume();
                Some(Node::NumberLiteral(value))
            }
            _ => None,
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_list(&mut self) -> Vec<Node<'a>> {
        let mut items = Vec::new();

        loop {
            match self.parse() {
                Some(node) => items.push(node),
                None => break,
            }
        }

        items
    }
}

impl<'a> Iterator<Node<'a>> for Parser<'a> {
    fn next(&mut self) -> Option<Node<'a>> {
        self.parse()
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::{Parser, Node};

    fn test_expr<'a>(expr: &'a str, expected_result: Option<Node<'a>>) {
        let result = Parser::new(expr).parse();

        if result != expected_result {
            panic!("Mismatch expression result for `{}`. Expected: {}, got: {}", expr, expected_result, result);
        }
    }

    #[test]
    fn test_simple() {
        test_expr("", None);
        test_expr("()", Some(Node::List(vec![])));
        test_expr("(+ 1 1)", Some(Node::List(vec![Node::Atom("+"), Node::NumberLiteral(1.0), Node::NumberLiteral(1.0)])));
        test_expr("(+ 1 (- 2 1))", Some(Node::List(vec![
            Node::Atom("+"), Node::NumberLiteral(1.0),
            Node::List(vec![
                Node::Atom("-"), Node::NumberLiteral(2.0),
                Node::NumberLiteral(1.0),
            ]),
        ])));
    }
}
