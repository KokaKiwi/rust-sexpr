use eval::context::Context;
use eval::value::Value;
use eval::VM;
use parser::Node;

pub fn define(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    let (name, value) = match args {
        [Node::Atom(ref name), ref value] => (name.to_string(), value),
        _ => return None,
    };

    if let Some(value) = VM::eval_in_context(ctx, value) {
        ctx.insert(name, value);
    }

    Some(Value::nil())
}

pub fn lambda(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    let (args, body) = match args {
        [Node::List(ref args), ref body] => (args, body),
        _ => return None,
    };

    let args: Option<Vec<_>> = args.iter().map(|arg| match *arg {
        Node::Atom(ref name) => Some(name.to_string()),
        _ => None,
    }).collect();
    let args = match args {
        Some(args) => args,
        None => return None,
    };

    let fctx = ctx.clone();
    let body = body.clone();

    Some(Value::Function {
        context: fctx,
        args: args,
        body: body,
    })
}
