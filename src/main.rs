#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(box_syntax, box_patterns)]
#[macro_use]
#[warn(deprecated)]
extern crate combine;
extern crate itertools;

mod compiler;
mod core;
mod elaborator;
mod helper;
mod interpreter;
mod parser;

use crate::compiler::compile;
use crate::interpreter::eval;
use crate::parser as tms_parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let steps: u32 = args[2].parse().unwrap();
    if filename.ends_with(".tms") {
        let contents = fs::read_to_string(filename).expect("File read error...");
        let c = compile(elaborator::elaborate(tms_parser::parse(&contents)));
        println!("{:?}", eval(&c, steps));
    } else {
        panic!("Not a `.tms` file!");
    }
}
