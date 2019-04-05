extern crate combine;

use std::rc::Rc;
use crate::core::ast::Term;
use crate::core::ast::Step;
use combine::parser::char::{char, letter, spaces, alpha_num, string};
use combine::{many, many1, sep_by, Parser};
use combine::error::{ParseError};
use combine::stream::{Stream};

/**

not a complete grammer, but enough for the context

term :=
    | ident
    | symbol
    | rule
    | table
    | machine
machine :=
    | "(" "machine" ident table ")"
table :=
    | "(" "table" many1(rule) ")"
rule :=
    | "(" ident symbol operation ident ")"
operation :=
    | "[" sep_by(right | left | none | print, ",") "]"
**/

pub fn parse(m: &str) -> Term {
    match term().parse(m) {
        Ok((t, _)) => t,
        Err(_) => panic!(""),
    }
}

fn term<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    ident().or(symbol()).or(rule()).or(table()).or(machine()).map(|t| t)
}

fn ident<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    letter().with(many(alpha_num())).map(|str| Term::Ident(str))
}

fn symbol<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many1(alpha_num()).map(|s| Term::Symbol(s))
}

fn operation<I>() -> impl Parser<Input = I, Output = Vec<Term>>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let right = char('R').map(|_| Term::Operation(Step::R));
    let left = char('L').map(|_| Term::Operation(Step::L));
    let none = char('N').map(|_| Term::Operation(Step::N));
    let print = (char('P'), many(letter())).map(|(_, sym)| Term::Operation(Step::P(Rc::new(Term::Symbol(sym)))));
    (char('['), sep_by(right.or(left).or(none).or(print), char(',')), char(']')).map(|(_, v,_ )| v)
}

fn rule<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char('('), spaces(), ident(), spaces(), symbol(), spaces(), operation(), spaces(), ident(), spaces(), char(')'))
    .map(|(_, _, mc, _, sym, _, vop, _, fc, _, _)| {
        Term::Rule(
            Rc::new(mc),
            Rc::new(sym),
            vop,
            Rc::new(fc)
        )
    })
}

fn rule_seq(vs: &[Term]) -> Term {
    match vs {
        [head, rest..] => Term::Seq(Rc::new(head.clone()), Rc::new(rule_seq(rest))),
        [last] => last.clone(),
    }
}

fn table<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char('('), string("table"), spaces(), many1(rule()))
    .map(|(_, _, _, rs): (_, _, _, Vec<Term>)| {
        Term::Table(Rc::new(rule_seq(&rs)))
     })
}

fn machine<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char('('), string("machine"), spaces(), ident(), spaces(), term())
    .map(|(_, _, _, name, _, t)| { Term::Machine(Rc::new(name), Rc::new(t)) })
}
