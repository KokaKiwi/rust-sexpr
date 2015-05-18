use super::context::Context;
use super::value::Value;

mod arith;
mod context;
mod io;
mod run;

macro_rules! register_builtin(
    ($ctx:ident, $name:expr => $f:path) => (
        $ctx.insert($name.to_string(), Value::BuiltinFunction($f));
    );
);

pub fn register(ctx: &mut Context) {
    // Context operations
    register_builtin!(ctx, "define" => context::define);
    register_builtin!(ctx, "lambda" => context::lambda);

    // Arithmetic operations
    register_builtin!(ctx, "+" => arith::add);
    register_builtin!(ctx, "-" => arith::sub);
    register_builtin!(ctx, "*" => arith::mul);
    register_builtin!(ctx, "/" => arith::div);

    // I/O operations
    register_builtin!(ctx, "print" => io::print);

    // Run operations
    register_builtin!(ctx, "do" => run::do_);
}
