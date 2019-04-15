use crate::core::ast::Term;

pub fn rule_seq(vs: &[Term]) -> Term {
    match vs {
        [last] => last.clone(),
        [head, rest..] => Term::Seq(box head.clone(), box rule_seq(rest)),
        [] => unreachable!(),
    }
}

pub fn flatten_seq(f: &Term, s: Term) -> Term {
    match f {
        Term::Rule(_, _, _, _) => Term::Seq(box f.clone(), box s),
        Term::Seq(f1, s1) => Term::Seq(f1.clone(), box flatten_seq(s1, s)),
        _ => unreachable!(),
    }
}
