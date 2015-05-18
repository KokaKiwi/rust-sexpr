use eval::context::Context;
use eval::helper;
use eval::value::Value;
use parser::Node;

pub fn do_(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    let args = match helper::eval_args(ctx, args) {
        Some(args) => args,
        None => return None,
    };

    args.into_iter().last().map(|value| value)
}
