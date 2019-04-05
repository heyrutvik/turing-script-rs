pub mod ast {
    use std::rc::Rc;

    #[derive(Debug)]
    pub enum Term {
        // identifier: for machine name and configuration names
        Ident(String),
        // symbol: to write on tape
        Symbol(String),
        // operation: head operation on tape
        Operation(Step),
        // machine: name and instruction
        Machine(Rc<Term>, Rc<Term>),
        // table rule: m-config, symbol, operations, f-config
        Rule(Rc<Term>, Rc<Term>, Vec<Term>, Rc<Term>),
        // sequence:
        Seq(Rc<Term>, Rc<Term>),
        // table: sequence of rules
        Table(Rc<Term>)
    }

    #[derive(Debug)]
    pub enum Step {
        P(Rc<Term>),
        R,
        L,
        N
    }

    impl Clone for Term {
        fn clone(&self) -> Term {
            match self {
                Term::Ident(s) => Term::Ident(s.to_string()),
                Term::Symbol(s) => Term::Symbol(s.to_string()),
                Term::Operation(s) => Term::Operation(s.clone()),
                Term::Machine(n, r) => Term::Machine(n.clone(), r.clone()),
                Term::Rule(mc, s, os, fc) => Term::Rule(mc.clone(), s.clone(), os.clone(), fc.clone()),
                Term::Seq(fst, snd) => Term::Seq(fst.clone(), snd.clone()),
                Term::Table(seq) => Term::Table(seq.clone()),
            }
        }
    }

    impl Clone for Step {
        fn clone(&self) -> Step {
            match self {
                Step::P(t) => Step::P(t.clone()),
                Step::R => Step::R,
                Step::L => Step::L,
                Step::N => Step::N,
            }
        }
    }
}
