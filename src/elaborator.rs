use std::collections::HashMap;
use crate::core::ast::{Term, Step, Dir};
use crate::helper;

pub fn step(t: Term) -> Term {

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
        Term::Seq(box f, box s) => Term::Seq(box step(f), box step(s)),
        Term::Table(box is) => Term::Table(box step(is)),
        Term::Machine(name, box is) => Term::Machine(name, box step(is)),
        Term::Rule(mc, sy, os, fc) => Term::Rule(mc, sy, expand(&os[..]), fc),
    }
}

pub fn rule(t: Term) -> Term {

    struct Env {
        names: HashMap<String, Vec<String>>,
    }

    impl Env {
        pub fn last(&mut self, key: &str) -> String {
            let elements = self.names.entry(key.to_string()).or_insert(vec!(key.to_string()));
            elements.last().unwrap().to_string()
        }
        pub fn fresh(&mut self, key: &str) -> String {
            let elements = self.names.entry(key.to_string()).or_insert(vec!(key.to_string()));
            let n = format!("{}{}", elements.last().unwrap(), "'");
            elements.push(n.clone());
            n
        }
    }

    fn _rule(t: Term, e: &mut Env) -> Term {

        fn expand(t: &Term, e: &mut Env) -> Vec<Term> {
            match t {
                Term::Rule(box Term::Ident(mc), box Term::Symbol(s), os, box Term::Ident(fc)) => {
                    if os.len() == 2 { // already two steps
                        vec!(t.clone())
                    } else { // steps more than two
                        let mut vts: Vec<Term> = vec!();

                        let osc = os.chunks(2);
                        let osc_len = osc.len();

                        for (i, chu) in osc.enumerate() {
                            if i == 0 { // first
                                vts.push(
                                    Term::Rule(
                                        box Term::Ident(mc.to_string()),
                                        box Term::Symbol(s.to_string()),
                                        chu.to_vec(),
                                        box Term::Ident(e.fresh(mc))
                                    )
                                )
                            } else if i+1 == osc_len { // last
                                vts.push(
                                    Term::Rule(
                                        box Term::Ident(e.last(mc)),
                                        box Term::Symbol(s.to_string()),
                                        chu.to_vec(),
                                        box Term::Ident(fc.to_string())
                                    )
                                )
                            } else {
                                vts.push(
                                    Term::Rule(
                                        box Term::Ident(e.last(mc)),
                                        box Term::Symbol(s.to_string()),
                                        chu.to_vec(),
                                        box Term::Ident(e.fresh(mc))
                                    )
                                );
                            }
                        }
                        vts
                    }
                },
                _ => unreachable!()
            }
        }

        match t {
            Term::Ident(_) | Term::Symbol(_) | Term::Exec(_) => t,
            Term::Seq(box f, box s) => helper::flatten_seq(&_rule(f, e), _rule(s, e)),
            Term::Table(box is) => Term::Table(box _rule(is, e)),
            Term::Machine(name, box is) => Term::Machine(name, box _rule(is, e)),
            Term::Rule(_, _, ref os, _) if os.len() == 2 => t,
            Term::Rule(_, _, _, _) => helper::rule_seq(&expand(&t, e)),
        }
    }

    _rule(t, &mut Env {names : HashMap::new()})
}
