use parser::Node;
use super::context::Context;
use super::value::Value;
use super::vm::VM;

pub fn eval_args(ctx: &mut Context, args: &[Node]) -> Option<Vec<Value>> {
    args.iter().map(|arg| VM::eval_in_context(ctx, arg)).collect()
}
