use eval::context::Context;
use eval::value::Value;
use eval::vm::VM;
use parser::Node;

pub fn add<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    let args: Option<Vec<f64>> = args.iter().map(|arg| VM::eval_in_context(ctx, arg)).map(|arg| if let Some(Value::NumberLiteral(value)) = arg { Some(value) } else { None }).collect();

    if let Some(args) = args {
        let result = args.into_iter().fold(0f64, |a, b| a + b);
        return Value::NumberLiteral(result);
    }

    Value::nil()
}

pub fn sub<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    let args: Option<Vec<f64>> = args.iter().map(|arg| VM::eval_in_context(ctx, arg)).map(|arg| if let Some(Value::NumberLiteral(value)) = arg { Some(value) } else { None }).collect();

    if let Some(args) = args {
        let mut iter = args.into_iter();
        let init = iter.next().unwrap_or(0f64);
        let result = iter.fold(init, |a, b| a - b);
        return Value::NumberLiteral(result);
    }

    Value::nil()
}

pub fn mul<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    let args: Option<Vec<f64>> = args.iter().map(|arg| VM::eval_in_context(ctx, arg)).map(|arg| if let Some(Value::NumberLiteral(value)) = arg { Some(value) } else { None }).collect();

    if let Some(args) = args {
        let result = args.into_iter().fold(1f64, |a, b| a * b);
        return Value::NumberLiteral(result);
    }

    Value::nil()
}

pub fn div<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    let args: Option<Vec<f64>> = args.iter().map(|arg| VM::eval_in_context(ctx, arg)).map(|arg| if let Some(Value::NumberLiteral(value)) = arg { Some(value) } else { None }).collect();

    if let Some(args) = args {
        let mut iter = args.into_iter();
        let init = iter.next().unwrap_or(0f64);
        let result = iter.fold(init, |a, b| a / b);
        return Value::NumberLiteral(result);
    }

    Value::nil()
}

pub fn mod_<'a>(ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    let args: Option<Vec<f64>> = args.iter().map(|arg| VM::eval_in_context(ctx, arg)).map(|arg| if let Some(Value::NumberLiteral(value)) = arg { Some(value) } else { None }).collect();

    if let Some(args) = args {
        let mut iter = args.into_iter();
        let init = iter.next().unwrap_or(0f64);
        let result = iter.fold(init, |a, b| a % b);
        return Value::NumberLiteral(result);
    }

    Value::nil()
}
