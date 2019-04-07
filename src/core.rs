pub mod ast {
    use std::fmt;
    use std::slice::SliceConcatExt;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Term {
        // identifier: for machine name and configuration names
        Ident(String),
        // symbol: to write on tape
        Symbol(String),
        // execute: head operation on tape
        Exec(Step),
        // table rule: m-config, symbol, operations, f-config
        Rule(Box<Term>, Box<Term>, Vec<Term>, Box<Term>),
        // sequence:
        Seq(Box<Term>, Box<Term>),
        // table: sequence of rules
        Table(Box<Term>),
        // machine: name and instruction
        Machine(Box<Term>, Box<Term>),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Step {
        Effect(Kind),
        Move(Dir),
        Dynamic,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Kind {
        Print(Box<Term>),
        Erase,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dir {
        Right,
        Left,
        None,
    }

    // Display
    impl fmt::Display for Kind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Kind::Print(s) => write!(f, "P{}", s),
                Kind::Erase => write!(f, "E"),
            }
        }
    }

    impl fmt::Display for Dir {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Dir::Right => write!(f, "R"),
                Dir::Left => write!(f, "L"),
                Dir::None => write!(f, "N"),
            }
        }
    }

    impl fmt::Display for Step {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Step::Effect(s) => write!(f, "{}", s),
                Step::Move(d) => write!(f, "{}", d),
                Step::Dynamic => write!(f, "DYNAMIC"),
            }
        }
    }

    impl fmt::Display for Term {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Term::Ident(s) | Term::Symbol(s) => write!(f, "{}", s),
                Term::Exec(s) => write!(f, "{}", s),
                Term::Rule(mc, s, os, fc) =>
                    write!(f, "({} {} [{}] {})",
                        mc,
                        s,
                        os.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
                        fc),
                Term::Seq(fst, snd) => write!(f, "{} {}", fst, snd),
                Term::Table(seq) => write!(f, "table {}", seq),
                Term::Machine(n, r) => write!(f, "(machine {} ({}))", n, r),
            }
        }
    }
}
