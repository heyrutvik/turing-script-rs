use crate::core::ast::Dir;
use crate::core::ast::Kind;
use crate::core::ast::Step;
use crate::core::ast::Sym as ASTSym;
use crate::core::ast::Term;
use crate::core::standard::Op;
use crate::core::standard::Rule;
use crate::core::standard::Sym;

pub fn compile(ast: Term) -> Env {
    let mut env = Env::new();
    _compile(&ast, &mut env);
    env
}

fn _compile(ast: &Term, c: &mut Env) {
    match ast {
        Term::Ident(q) => c.push_config(q.to_string()),
        Term::Symbol(s) => c.push_symbol(s.clone()),
        Term::Machine(_, box t) => _compile(t, c),
        Term::Table(box rs) => _compile(rs, c),
        Term::Rule(box Term::Ident(q1), box Term::Symbol(s), op, box Term::Ident(q2)) => {
            _compile(&Term::Ident(q1.to_string()), c);
            _compile(&Term::Symbol(s.clone()), c);
            _compile(&Term::Ident(q2.to_string()), c);
            c.push_rule(
                q1.to_string(),
                s.clone(),
                op[0].clone(),
                op[1].clone(),
                q2.to_string(),
            );
        }
        Term::Seq(box f, box s) => {
            _compile(f, c);
            _compile(s, c);
        }
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub struct Env {
    target: Vec<Rule>,
    config: Vec<String>,
    pub symbol: Vec<ASTSym>,
}

impl Env {
    fn new() -> Env {
        Env {
            target: vec![],
            config: vec![],
            symbol: vec![ASTSym::Blank],
        }
    }
    pub fn first(&self) -> Option<Rule> {
        self.target.first().map(|r| r.clone())
    }
    pub fn find(&self, mc: usize, sym: usize) -> Option<(Op, usize)> {
        self.target
            .iter()
            .find(|r| r.mc == mc && r.sym == sym)
            .map(|r| (r.op.clone(), r.fc.clone()))
    }
    fn push_config(&mut self, s: String) {
        if !self.config.contains(&s) {
            self.config.push(s);
        }
    }
    fn index_config(&mut self, s: &str) -> usize {
        self.config.iter().position(|r| r == s).unwrap()
    }
    fn push_symbol(&mut self, s: ASTSym) {
        if !self.symbol.contains(&s) {
            self.symbol.push(s);
        }
    }
    pub fn index_symbol(&self, s: ASTSym) -> usize {
        self.symbol.iter().position(|r| r == &s).unwrap()
    }
    fn push_rule(&mut self, q1: String, s: ASTSym, s1: Term, s2: Term, q2: String) {
        match (s1, s2) {
            (Term::Exec(o1), Term::Exec(o2)) => {
                let rq1 = self.index_config(&q1);
                let rs = self.index_symbol(s);
                let ro = self.dir(o1, o2);
                let rq2 = self.index_config(&q2);
                self.target.push(Rule {
                    mc: rq1,
                    sym: rs,
                    op: ro,
                    fc: rq2,
                })
            }
            _ => unreachable!(),
        }
    }
    fn eff(&mut self, p: Step) -> Sym {
        match p {
            Step::Effect(k) => match k {
                Kind::Print(box Term::Symbol(s)) => match s {
                    ASTSym::Blank => Sym::S(self.index_symbol(ASTSym::Blank)),
                    ASTSym::String(_) => {
                        let s1 = s.clone();
                        self.push_symbol(s);
                        Sym::S(self.index_symbol(s1))
                    }
                    _ => unreachable!(),
                },
                Kind::Erase => Sym::S(self.index_symbol(ASTSym::Blank)),
                _ => unreachable!(),
            },
            Step::Dynamic => Sym::D,
            _ => unreachable!(),
        }
    }
    fn dir(&mut self, p: Step, d: Step) -> Op {
        match d {
            Step::Move(d) => match d {
                Dir::Right => Op::R(self.eff(p)),
                Dir::Left => Op::L(self.eff(p)),
                Dir::None => Op::N(self.eff(p)),
            },
            _ => unreachable!(),
        }
    }
}
