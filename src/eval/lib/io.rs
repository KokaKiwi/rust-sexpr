use eval::context::Context;
use eval::value::Value;
use eval::vm::VM;
use parser::Node;

pub fn print<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    let values: Vec<_> = args.iter().map(|node| VM::eval_in_context(ctx, node).unwrap_or(Value::nil())).collect();
    let reprs: Vec<_> = values.iter().map(|value| format!("{}", value)).collect();
    print!("{}", reprs.connect(" "));
    Value::nil()
}

pub fn println<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    let values: Vec<_> = args.iter().map(|node| VM::eval_in_context(ctx, node).unwrap_or(Value::nil())).collect();
    let reprs: Vec<_> = values.iter().map(|value| format!("{}", value)).collect();
    println!("{}", reprs.connect(" "));
    Value::nil()
}
