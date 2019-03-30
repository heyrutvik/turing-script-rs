#![feature(slice_concat_ext)]
mod core;

use std::rc::Rc;
use crate::core::ast::Term::*;
use crate::core::ast::Step::*;

fn main() {
    let rule1 = Rule(Rc::new(Ident("a".to_string())), Rc::new(Symbol("0".to_string())), vec!(Operation(R), Operation(N)), Rc::new(Ident("b".to_string())));
    let rule2 = Rule(Rc::new(Ident("b".to_string())), Rc::new(Symbol("1".to_string())), vec!(Operation(P(Rc::new(Ident("0".to_string())))), Operation(L)), Rc::new(Ident("a".to_string())));
    let table = Table(Rc::new(Seq(Rc::new(rule1), Rc::new(rule2))));
    let machine = Machine(Rc::new(Ident("m1".to_string())), Rc::new(table));
    println!("{}", machine);
}
