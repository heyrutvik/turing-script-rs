use crate::compiler::Env;
use crate::core::ast::Sym as ASTSym;
use crate::core::standard::{Op, Sym};

pub fn eval(e: &Env, steps: u32) -> Vec<String> {
    _eval(steps, e, &mut CompleteConfig::new(e))
}

fn _eval(steps: u32, e: &Env, cc: &mut CompleteConfig) -> Vec<String> {
    for _ in 1..steps {
        cc.update(e)
    }
    cc.make_tape(&e.symbol[..])
}

struct CompleteConfig {
    tape: Vec<usize>,
    head: usize,
    config: usize,
}

impl CompleteConfig {
    fn new(e: &Env) -> CompleteConfig {
        let r = e.first().unwrap();
        CompleteConfig {
            tape: vec![e.index_symbol(ASTSym::Blank)],
            head: 0,
            config: r.mc,
        }
    }
    fn make_tape(&self, ss: &[ASTSym]) -> Vec<String> {
        self.tape
            .iter()
            .map(|&s| match ss[s].clone() {
                ASTSym::String(s) => s,
                ASTSym::Blank => "_".to_string(),
                _ => unreachable!(),
            })
            .collect::<Vec<String>>()
    }
    fn add_square(&mut self, e: &Env) {
        if self.tape.len() <= self.head + 1 {
            self.head = self.head + 1;
            self.tape.push(e.index_symbol(ASTSym::Blank));
        } else {
            self.head = self.head + 1;
        }
    }
    fn print(&mut self, u: usize) {
        self.tape[self.head] = u;
    }
    fn update(&mut self, e: &Env) {
        let (op, fc) = e.find(self.config, self.tape[self.head]).unwrap();
        match op {
            Op::R(Sym::S(u)) => {
                self.print(u);
                self.add_square(e);
            }
            Op::R(Sym::D) => {
                self.add_square(e);
            }
            Op::L(Sym::S(u)) => {
                self.print(u);
                self.head = self.head - 1;
            }
            Op::L(Sym::D) => {
                self.head = self.head - 1;
            }
            Op::N(Sym::S(u)) => {
                self.print(u);
            }
            Op::N(Sym::D) => {}
        }
        self.config = fc;
    }
}
