use crate::core::ast::{Term, Step, Dir};

pub fn exec_step(t: Term) -> Term {

    fn expand(os: &[Term]) -> Vec<Term> {
        match os {
            [Term::Exec(e1), Term::Exec(e2), rest..] => {
                match (e1, e2) {
                    (Step::Effect(_), Step::Effect(_)) => {
                        let v1 = vec!(Term::Exec(e2.clone()), Term::Exec(Step::Move(Dir::None)));
                        [&v1[..], &expand(rest)[..]].concat()
                    },
                    (Step::Effect(_), Step::Move(_)) => {
                        let v1 = vec!(Term::Exec(e1.clone()), Term::Exec(e2.clone()));
                        [&v1[..], &expand(rest)[..]].concat()
                    },
                    (Step::Move(_), Step::Effect(_)) => {
                        let v1 = vec!(Term::Exec(Step::Dynamic), Term::Exec(e1.clone()),
                                      Term::Exec(e2.clone()), Term::Exec(Step::Move(Dir::None)));
                        [&v1[..], &expand(rest)[..]].concat()
                    },
                    (Step::Move(_), Step::Move(_)) => {
                        let v1 = vec!(Term::Exec(Step::Dynamic), Term::Exec(e1.clone()),
                                      Term::Exec(Step::Dynamic), Term::Exec(e2.clone()));
                        [&v1[..], &expand(rest)[..]].concat()
                    },
                    _ => unreachable!()
                }
            },
            [Term::Exec(e1)] => {
                match e1 {
                    Step::Effect(_) => {
                        vec!(Term::Exec(e1.clone()), Term::Exec(Step::Move(Dir::None)))
                    },
                    Step::Move(_) => {
                        vec!(Term::Exec(Step::Dynamic), Term::Exec(e1.clone()))
                    },
                    _ => unreachable!()
                }
            },
            _ => vec!()
        }
    }

    match t {
        Term::Ident(_) | Term::Symbol(_) | Term::Exec(_) => t,
        Term::Seq(box f, box s) => Term::Seq(box exec_step(f), box exec_step(s)),
        Term::Table(box is) => Term::Table(box exec_step(is)),
        Term::Machine(name, box is) => Term::Machine(name, box exec_step(is)),
        Term::Rule(mc, sy, os, fc) => Term::Rule(mc, sy, expand(&os[..]), fc),
    }
}
