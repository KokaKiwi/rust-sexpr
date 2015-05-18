use parser::Node;
use super::context::Context;
use super::value::Value;

pub struct VM {
    root: Context,
}

impl VM {
    pub fn new() -> VM {
        VM {
            root: super::context::new(),
        }
    }

    pub fn eval(&mut self, node: &Node) -> Option<Value> {
        eval_in_context(&mut self.root, node)
    }

    pub fn eval_in_context(ctx: &mut Context, node: &Node) -> Option<Value> {
        eval_in_context(ctx, node)
    }

    pub fn eval_expr(&mut self, expr: &str) -> Option<Value> {
        use lexer::Lexer;
        use parser::Parser;
        Parser::new(Lexer::new(expr.as_bytes())).parse().and_then(|node| self.eval(&node))
    }

    pub fn load_stdlib(&mut self) {
        use super::lib as stdlib;
        stdlib::register(&mut self.root);
    }
}

fn eval_in_context(ctx: &mut Context, node: &Node) -> Option<Value> {
    match *node {
        Node::Atom(ref name) => ctx.get(name).map(|value| value.clone()),
        Node::List(ref items) => eval_list(ctx, items.as_slice()),
        _ => Some(eval_value(node)),
    }
}

fn eval_list(ctx: &mut Context, items: &[Node]) -> Option<Value> {
    match items {
        [ref f, args..] => eval_call(ctx, f, args),
        [] => Some(Value::nil()),
    }
}

fn eval_call(ctx: &mut Context, f: &Node, args: &[Node]) -> Option<Value> {
    let f = match eval_in_context(ctx, f) {
        Some(f) => f,
        None => return None,
    };

    match f {
        Value::Function { context: mut fctx, args: argnames, body } => {
            for (name, node) in argnames.iter().zip(args.iter()) {
                let value = match eval_in_context(ctx, node) {
                    Some(value) => value,
                    None => return None,
                };
                fctx.insert(name.clone(), value);
            }

            eval_in_context(&mut fctx, &body)
        }
        Value::BuiltinFunction(f) => f(ctx, args),
        _ => None,
    }
}

fn eval_value(node: &Node) -> Value {
    match *node {
        Node::Atom(ref value) | Node::String(ref value) => Value::String(value.clone()),
        Node::Number(value) => Value::Number(value),
        Node::List(ref items) => Value::List(items.iter().map(eval_value).collect()),
    }
}

#[cfg(test)]
mod test {
    use eval::value::Value;
    use super::VM;

    fn assert_expr(expr: &str, expected: Option<Value>) {
        let mut vm = VM::new();
        vm.load_stdlib();

        assert_eq!(vm.eval_expr(expr), expected);
    }

    #[test]
    fn test_value() {
        assert_expr("25", Some(Value::Number(25.0)));
    }

    #[test]
    fn test_arith() {
        assert_expr("(+ 1 2)", Some(Value::Number(3.0)));
        assert_expr("(* (+ 2 2) (- 5 3))", Some(Value::Number(8.0)));
    }
}
