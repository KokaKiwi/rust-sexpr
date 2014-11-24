use eval::context::Context;
use eval::value::Value;
// use eval::vm::VM;
use parser::Node;

pub fn quote<'a>(_ctx: &mut Context<'a>, args: &[Node<'a>]) -> Value<'a> {
    match args {
        [ref node] => {
            return Value::from_node(node);
        }
        _ => {}
    }

    Value::nil()
}
