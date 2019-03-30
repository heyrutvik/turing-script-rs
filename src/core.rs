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
                Term::Table(seq) => write!(f, "table \n{}", seq)
            }
        }
    }
}
