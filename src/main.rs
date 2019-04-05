#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
mod core;
mod parser;

use std::rc::Rc;
use crate::core::ast::Term;
use crate::core::ast::Term::*;
use crate::core::ast::Step::*;
use crate::parser as ap;

fn main() {
    println!("{}", ap::parse("(machine name (table (a 0 [P0, R] b) (b 1 [P1, L] a)))"));
}

fn m() -> Term {
    let rule1 = Rule(Rc::new(Ident("a".to_string())), Rc::new(Symbol("0".to_string())), vec!(Operation(P(Rc::new(Ident("1".to_string())))), Operation(R)), Rc::new(Ident("b".to_string())));
    let rule2 = Rule(Rc::new(Ident("b".to_string())), Rc::new(Symbol("1".to_string())), vec!(Operation(P(Rc::new(Ident("0".to_string())))), Operation(L)), Rc::new(Ident("a".to_string())));
    let table = Table(Rc::new(Seq(Rc::new(rule1), Rc::new(rule2))));
    Machine(Rc::new(Ident("name".to_string())), Rc::new(table))
}
