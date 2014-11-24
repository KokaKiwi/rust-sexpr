use std::fmt;
use parser::Node;
use eval::context::Context;

pub type BuiltinFn = for<'a> fn(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a>;

#[deriving(Clone)]
pub enum Value<'a> {
    StringLiteral(String),
    NumberLiteral(f64),
    List(Vec<Value<'a>>),
    Function(Context<'a>, Vec<String>, Node<'a>),
    BuiltinFunction(BuiltinFn),
}

impl<'a> Value<'a> {
    pub fn from_node(node: &Node) -> Value<'a> {
        match *node {
            Node::Atom(name) => Value::StringLiteral(name.into_string()),
            Node::StringLiteral(value) => Value::StringLiteral(value.into_string()),
            Node::NumberLiteral(value) => Value::NumberLiteral(value),
            Node::List(ref items) => {
                let items: Vec<_> = items.iter().map(Value::from_node).collect();
                Value::List(items)
            }
        }
    }

    pub fn to_node(&'a self) -> Option<Node<'a>> {
        match *self {
            Value::StringLiteral(ref value) => Some(Node::StringLiteral(value.as_slice())),
            Value::NumberLiteral(value) => Some(Node::NumberLiteral(value)),
            Value::List(..) => None,
            Value::Function(..) => None,
            Value::BuiltinFunction(..) => None,
        }
    }
}

impl<'a> Value<'a> {
    pub fn nil() -> Value<'a> {
        Value::List(Vec::new())
    }
}

impl<'a> fmt::Show for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::StringLiteral(ref value) => write!(f, "\"{}\"", value),
            Value::NumberLiteral(value) => write!(f, "{}", value),
            Value::List(ref items) => {
                let reprs: Vec<_> = items.iter().map(|item| format!("{}", item)).collect();
                write!(f, "({})", reprs.connect(" "))
            }
            Value::Function(_, ref args, ref body) => write!(f, "{} -> {}", args, body.to_src()),
            Value::BuiltinFunction(_) => write!(f, "BuiltinFunction"),
        }
    }
}
