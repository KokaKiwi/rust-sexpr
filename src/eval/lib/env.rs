use eval::context::Context;
use eval::value::Value;
use eval::vm::VM;
use parser::Node;

pub fn define<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    match args {
        [ref name, ref value] => {
            let name = match name {
                &Node::Atom(name) => Some(name),
                &Node::StringLiteral(value) => Some(value.as_slice()),
                _ => None,
            };
            let value = VM::eval_in_context(ctx, value);

            match (name, value) {
                (Some(name), Some(value)) => ctx.set(name, value),
                _ => {}
            }
        }
        _ => {}
    }

    Value::nil()
}

pub fn lambda<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    match args {
        [ref args, ref body] => {
            let args: Option<Vec<String>> = match args {
                &Node::List(ref items) => {
                    items.iter().map(|item| match item {
                        &Node::Atom(name) => Some(name.into_string()),
                        &Node::StringLiteral(value) => Some(value.into_string()),
                        _ => None,
                    }).collect()
                }
                _ => None,
            };

            if let Some(args) = args {
                let fctx = ctx.clone();
                let body = body.clone();

                return Value::Function(fctx, args, body);
            }
        }
        _ => {}
    }

    Value::nil()
}
