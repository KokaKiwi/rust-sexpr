#![feature(phase)]
#![cfg(not(test))]

extern crate sexpr;

use sexpr::parser::Parser;
use sexpr::eval::VM;

fn load_file(filename: &str) -> String {
    use std::io::File;

    let mut file = File::open(&Path::new(filename));
    file.read_to_string().unwrap()
}

fn main() {
    let args = ::std::os::args();

    let ref filename = args[1];
    let expr = load_file(filename.as_slice());

    let mut parser = Parser::new(expr.as_slice());
    let mut vm = VM::new();
    vm.load_stdlib();

    for node in parser {
        vm.eval(&node);
    }
}
