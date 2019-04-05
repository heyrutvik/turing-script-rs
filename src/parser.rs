extern crate combine;

use std::rc::Rc;
use crate::core::ast::Term;
use crate::core::ast::Step;
use combine::parser::char::{char, letter, spaces, alpha_num, string};
use combine::{many, many1, sep_by, Parser, one_of, choice};
use combine::error::{ParseError};
use combine::stream::{Stream};

/**

not a complete grammer, but enough for the context

term := machine | table | rule | symbol | ident
machine := "(" "machine" ident term ")"
table := "(" "table" many1(term) ")"
rule := "(" ident symbol operation ident ")"
operation := "[" sep_by(right | left | none | print, ",") "]"
**/

static MACHINE: &str = "machine";
static TABLE: &str = "table";
static ROUND_OPEN: char = '(';
static ROUND_CLOSE: char = ')';
static BOX_OPEN: char = '[';
static BOX_CLOSE: char = ']';

pub fn parse(m: &str) -> Term {
    match machine().parse(m) {
        Ok((t, _)) => t,
        Err(_) => panic!(""),
    }
}

fn term<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    machine()
        .or(table())
        .or(rule())
        .or(symbol())
        .or(ident())
}

fn ident<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let allowed_chars = || one_of("!$%&|*+-/:<=>?@^_~#".chars());
    (letter(), many(alpha_num().or(allowed_chars()))).map(|(c, str): (char, String)| Term::Ident(format!("{}{}", c, str)))
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
    let print = (char('P'), symbol()).map(|(_, sym)| Term::Operation(Step::P(Rc::new(sym))));
    (char(BOX_OPEN), sep_by(right.or(left).or(none).or(print), char(',').skip(spaces())), char(BOX_CLOSE)).map(|(_, v,_ )| v)
}

fn rule<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char(ROUND_OPEN).skip(spaces()), ident().skip(spaces()), symbol().skip(spaces()), operation().skip(spaces()), ident().skip(spaces()), char(ROUND_CLOSE))
        .map(|(_, mc, sym, vop, fc, _)| {
            Term::Rule(
                Rc::new(mc),
                Rc::new(sym),
                vop,
                Rc::new(fc)
            )
        })
}

fn table<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char(ROUND_OPEN), string(TABLE).skip(spaces()), sep_by(term(), spaces()), char(ROUND_CLOSE))
        .map(|(_, _, rs, _): (_, _, Vec<Term>, _)| {
            Term::Table(Rc::new(rule_seq(&rs)))
         })
}

fn machine<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char(ROUND_OPEN).skip(spaces()), string(MACHINE).skip(spaces()), ident().skip(spaces()), term(), char(ROUND_CLOSE))
        .map(|(_, _, name, t, _)| { Term::Machine(Rc::new(name), Rc::new(t)) })
}

fn rule_seq(vs: &[Term]) -> Term {
    match vs {
        [last] => last.clone(),
        [head, rest..] => Term::Seq(Rc::new(head.clone()), Rc::new(rule_seq(rest))),
        [] => unreachable!(),
    }
}
