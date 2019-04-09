#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(box_syntax, box_patterns)]
#[macro_use]
#[warn(deprecated)]
extern crate combine;

mod core;
mod parser;
mod elaborator;
mod helper;

use std::env;
use std::fs;
use crate::parser as ap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    if filename.ends_with(".tms") {
        let contents = fs::read_to_string(filename)
            .expect("File read error...");
        println!("File content:");
        print!("{}", contents);
        let ast = ap::parse(&contents);
        let ast = elaborator::step(ast);
        println!("AST:");
        println!("{:?}", ast);
        println!("Code:");
        println!("{}", ast);
        let ast = elaborator::rule(ast);
        println!("AST:");
        println!("{:?}", ast);
        println!("Code:");
        println!("{}", ast);
    } else {
        panic!("Not a `.tms` file!");
    }
}
