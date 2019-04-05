pub mod ast {
    use std::rc::Rc;
    use std::fmt;
    use std::slice::SliceConcatExt;

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

    pub enum Step {
        P(Rc<Term>),
        R,
        L,
        N
    }

    impl fmt::Display for Term {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Term::Ident(s) => write!(f, "{}", s),
                Term::Symbol(s) => write!(f, "{}", s),
                Term::Operation(s) => write!(f, "{}", s),
                Term::Machine(n, r) => write!(f, "(machine {} \n({}))", n, r),
                Term::Rule(mc, s, os, fc) =>
                    write!(f, "({} {} [{}] {})",
                        mc,
                        s,
                        os.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
                        fc),
                Term::Seq(fst, snd) => write!(f, "{}\n{}", fst, snd),
                Term::Table(seq) => write!(f, "table \n{}", seq),
            }
        }
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

    impl fmt::Display for Step {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Step::P(t) => write!(f, "P{}", t),
                Step::R => write!(f, "R"),
                Step::L => write!(f, "L"),
                Step::N => write!(f, "N"),
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
