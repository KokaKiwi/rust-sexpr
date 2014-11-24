#![feature(phase, macro_rules, if_let, while_let)]
#![experimental]

#[phase(plugin, link)]
extern crate log;

pub mod lexer;
pub mod parser;
pub mod eval;
