use eval::context::Context;
use eval::helper;
use eval::value::Value;
use parser::Node;

fn do_arith<F>(ctx: &mut Context, args: &[Node], f: F) -> Option<Value>
    where F: Fn(f64, f64) -> Option<f64> {
    let args = match helper::eval_args(ctx, args) {
        Some(args) => args,
        None => return None,
    };
    let (a, b) = match args.as_slice() {
        [Value::Number(a), Value::Number(b)] => (a, b),
        _ => return None,
    };

    f(a, b).map(|value| Value::Number(value))
}

pub fn add(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    do_arith(ctx, args, |a, b| Some(a + b))
}

pub fn sub(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    do_arith(ctx, args, |a, b| Some(a - b))
}

pub fn mul(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    do_arith(ctx, args, |a, b| Some(a * b))
}

pub fn div(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    do_arith(ctx, args, |a, b| Some(a / b))
}
