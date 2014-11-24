use eval::context::Context;
use eval::value::Value;

mod arith;
mod ast;
mod env;
mod io;

macro_rules! register_builtin(
    ($ctx:ident, $name:expr => $f:path) => (
        $ctx.set($name, Value::BuiltinFunction($f));
    );
)

pub fn register(ctx: &mut Context) {
    // I/O
    register_builtin!(ctx, "print"  => io::print);
    register_builtin!(ctx, "println"=> io::println);

    // Env (functions, etc...)
    register_builtin!(ctx, "define" => env::define);
    register_builtin!(ctx, "lambda" => env::lambda);

    // AST operations
    register_builtin!(ctx, "quote"  => ast::quote);

    // Arithmetic operations
    register_builtin!(ctx, "+"      => arith::add);
    register_builtin!(ctx, "-"      => arith::sub);
    register_builtin!(ctx, "*"      => arith::mul);
    register_builtin!(ctx, "/"      => arith::div);
    register_builtin!(ctx, "%"      => arith::mod_);
}
