use std::string::ToString;
use eval::context::Context;
use eval::helper;
use eval::value::Value;
use parser::Node;

pub fn print(ctx: &mut Context, args: &[Node]) -> Option<Value> {
    let args = match helper::eval_args(ctx, args) {
        Some(args) => args,
        None => return None,
    };
    let args: Vec<_> = args.iter().map(ToString::to_string).collect();
    print!("{}", args.connect(" "));

    Some(Value::nil())
}
