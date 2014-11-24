use super::value::Value;
use super::context::Context;
use super::lib as stdlib;
use parser::Node;

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

    pub fn load_stdlib(&mut self) {
        stdlib::register(&mut self.root);
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
