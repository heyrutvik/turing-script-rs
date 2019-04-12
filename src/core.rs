pub mod ast {
    use std::fmt;
    use std::slice::SliceConcatExt;
    use itertools::Itertools;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Term {
        // identifier: for machine name and configuration names
        Ident(String),
        // symbol: to write on tape
        Symbol(Sym),
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

    impl Term {
        pub fn symbols(&self) -> Vec<Sym> {
            let mut vs = vec!();
            self._symbols(&mut vs);
            vs.into_iter().unique().collect()
        }

        fn _symbols(&self, ss:&mut Vec<Sym>) {
            match self {
                Term::Ident(_) | Term::Exec(_) => {},
                Term::Symbol(s) => {
                    match s {
                        Sym::Any => {},
                        Sym::Blank => {},
                        Sym::Dyn => {},
                        _ => ss.push(s.clone()),
                    }
                },
                Term::Rule(box _, box sym, _, box _) => {
                    sym._symbols(ss);
                },
                Term::Machine(_, box t) => t._symbols(ss),
                Term::Table(box rs) => rs._symbols(ss),
                Term::Seq(box f, box s) => {
                        f._symbols(ss);
                        s._symbols(ss);
                },
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Sym {
        String(String),
        Blank,
        Any,
        Dyn,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Step {
        Effect(Kind),
        Dynamic,
        Move(Dir),
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

    impl fmt::Display for Sym {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Sym::String(s) => write!(f, "{}", s),
                Sym::Blank => write!(f, "blank"),
                Sym::Any => write!(f, "any"),
                Sym::Dyn => write!(f, "$$"),
            }
        }
    }

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
                Term::Ident(s) => write!(f, "{}", s),
                Term::Symbol(s) => write!(f, "{}", s),
                Term::Exec(s) => write!(f, "{}", s),
                Term::Rule(mc, s, os, fc) =>
                    write!(f, "({} {} [{}] {})",
                        mc,
                        s,
                        os.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
                        fc),
                Term::Seq(fst, snd) => write!(f, "{}\n{}", fst, snd),
                Term::Table(seq) => write!(f, "table \n{}", seq),
                Term::Machine(n, r) => write!(f, "(machine {} ({}))", n, r),
            }
        }
    }
}

pub mod standard {
    #[derive(Debug, Clone)]
    pub struct Rule { pub mc: usize, pub sym: usize, pub op: Op, pub fc: usize }
    #[derive(Debug, Clone)]
    pub enum Op { R(Sym), L(Sym), N(Sym) }
    #[derive(Debug, Clone)]
    pub enum Sym { S(usize), D}
}
