pub mod ast {
    use std::rc::Rc;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Term {
        // identifier: for machine name and configuration names
        Ident(String),
        // symbol: to write on tape
        Symbol(String),
        // execute: head operation on tape
        Exec(Step),
        // machine: name and instruction
        Machine(Rc<Term>, Rc<Term>),
        // table rule: m-config, symbol, operations, f-config
        Rule(Rc<Term>, Rc<Term>, Vec<Term>, Rc<Term>),
        // sequence:
        Seq(Rc<Term>, Rc<Term>),
        // table: sequence of rules
        Table(Rc<Term>)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Step {
        Print(Rc<Term>),
        Right,
        Left,
        None
    }
}
