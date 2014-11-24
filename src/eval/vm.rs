use super::value::Value;
use super::context::Context;
use super::lib as stdlib;
use parser::{Parser, Node};

pub struct VM<'a> {
    root: Context<'a>,
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        VM {
            root: Context::new(),
        }
    }

    pub fn eval(&mut self, node: &Node<'a>) -> Option<Value<'a>> {
        VM::eval_in_context(&mut self.root, node)
    }

    pub fn eval_expr(&mut self, expr: &'a str) -> Option<Value<'a>> {
        let node = Parser::new(expr).parse();
        node.and_then(|node| self.eval(&node))
    }

    pub fn load_stdlib(&mut self) -> &mut VM<'a> {
        stdlib::register(&mut self.root);
        self
    }
}

impl<'a> VM<'a> {
    pub fn eval_in_context(ctx: &mut Context<'a>, node: &Node<'a>) -> Option<Value<'a>> {
        debug!("Eval in context: {} {}", node, ctx);
        match *node {
            Node::Atom(name) => ctx.get(name).map(|value| value.clone()),
            Node::List(ref items) => VM::eval_call_in_context(ctx, items.as_slice()),
            ref node @ _ => Some(Value::from_node(node)),
        }
    }

    pub fn eval_call_in_context(ctx: &mut Context<'a>, items: &[Node<'a>]) -> Option<Value<'a>> {
        match items {
            [ref f, args..] => {
                match VM::eval_in_context(ctx, f) {
                    Some(f) => VM::call_in_context(ctx, f, args),
                    None => None,
                }
            }
            [] => Some(Value::nil()),
        }
    }

    pub fn call_in_context(ctx: &mut Context<'a>, f: Value<'a>, args: &[Node<'a>]) -> Option<Value<'a>> {
        match f {
            Value::Function(ref fctx, ref argnames, ref body) => {
                let mut fctx = fctx.clone();

                for (name, node) in argnames.iter().zip(args.iter()) {
                    debug!("Eval arg: {}", node);
                    let value = VM::eval_in_context(ctx, node).expect("Invalid arg.");
                    fctx.set(name.as_slice(), value);
                }

                VM::eval_in_context(&mut fctx, body)
            }
            Value::BuiltinFunction(f) => {
                Some(f(ctx, args))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::VM;
    use eval::value::Value;

    fn test_expr_in_vm<'a>(vm: &mut VM<'a>, expr: &'a str, expected_result: Option<Value<'a>>) {
        let result = vm.eval_expr(expr);

        if result != expected_result {
            panic!("Mismatch expression result for `{}`. Expected: {}, got: {}", expr, expected_result, result);
        }
    }

    fn test_expr<'a>(expr: &'a str, expected_result: Option<Value<'a>>) {
        let mut vm = VM::new();
        vm.load_stdlib();
        test_expr_in_vm(&mut vm, expr, expected_result)
    }

    #[test]
    fn test_simple() {
        test_expr("", None);
        test_expr("25", Some(Value::NumberLiteral(25.0)));
    }

    #[test]
    fn test_arith() {
        test_expr("(+ 1 1)", Some(Value::NumberLiteral(2.0)));
        test_expr("(+ 1 (- 5 3))", Some(Value::NumberLiteral(3.0)));
    }

    #[test]
    fn test_env() {
        let mut vm = VM::new();
        vm.load_stdlib();

        test_expr_in_vm(&mut vm, "(define value 5)", Some(Value::nil()));
        test_expr_in_vm(&mut vm, "value", Some(Value::NumberLiteral(5.0)));
    }
}
