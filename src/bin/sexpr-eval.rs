#![feature(slice_patterns)]
extern crate sexpr;

use std::env;
use std::io::prelude::*;
use std::io::stdin;
use std::path::Path;
use std::fs::File;
use sexpr::eval::VM;
use sexpr::eval::value::Value;
use sexpr::lexer::Lexer;
use sexpr::parser::Parser;

fn eval<R: Read>(reader: R) -> Option<Value> {
    let lexer = Lexer::new(reader);
    let parser = Parser::new(lexer);

    let mut vm = VM::new();
    vm.load_stdlib();

    let value = parser
                // .inspect(|node| println!("{:?}", node))
                .flat_map(|node| vm.eval(&node).into_iter())
                .last();
    value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    match args {
        [] => {
            let reader = stdin();
            eval(reader);
        }
        [ref filename, ..] => {
            let reader = File::open(&Path::new(filename)).unwrap();
            eval(reader);
        }
    }
}
