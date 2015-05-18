use std::fmt;
use std::string::ToString;
use parser::Node;
use super::context::Context;

pub type BuiltinFn = fn(ctx: &mut Context, args: &[Node]) -> Option<Value>;

pub enum Value {
    String(String),
    Number(f64),
    List(Vec<Value>),
    Function {
        context: Context,
        args: Vec<String>,
        body: Node,
    },
    BuiltinFunction(BuiltinFn),
}

impl Value {
    pub fn nil() -> Value {
        Value::List(vec![])
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::String(ref value) => write!(f, "String({:?})", value),
            Value::Number(value) => write!(f, "Number({:?})", value),
            Value::List(ref items) => write!(f, "List({:?})", items),
            Value::Function { ref args, ref body, .. } => write!(f, "Function({:?}, {:?})", args, body),
            Value::BuiltinFunction(function) => write!(f, "BuiltinFunction({:?})", function as *mut BuiltinFn),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::String(ref value) => write!(f, "{}", value),
            Value::Number(value) => write!(f, "{}", value),
            Value::List(ref items) => {
                let items: Vec<_> = items.iter().map(ToString::to_string).collect();
                write!(f, "{}", items.connect(" "))
            }
            Value::Function { .. } => write!(f, "<Function>"),
            Value::BuiltinFunction(function) => write!(f, "<BuiltinFunction@{:?}>", function as *mut BuiltinFn),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::String(ref left), &Value::String(ref right)) => left.eq(right),
            (&Value::Number(left), &Value::Number(right)) => left == right,
            (&Value::List(ref left), &Value::List(ref right)) => left.eq(right),
            (&Value::BuiltinFunction(left), &Value::BuiltinFunction(right)) => left as *mut BuiltinFn == right as *mut BuiltinFn,
            _ => false,
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match *self {
            Value::String(ref value) => Value::String(value.clone()),
            Value::Number(value) => Value::Number(value),
            Value::List(ref items) => Value::List(items.clone()),
            Value::Function { ref context, ref args, ref body } => Value::Function {
                context: context.clone(),
                args: args.clone(),
                body: body.clone(),
            },
            Value::BuiltinFunction(f) => Value::BuiltinFunction(f),
        }
    }
}
