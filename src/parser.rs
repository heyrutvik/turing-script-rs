use crate::core::ast::{Term, Step, Kind, Dir};
use combine::parser::char::{char, letter, spaces, alpha_num, string};
use combine::{many, choice, many1, sep_by, Parser, one_of, parser, between};
use combine::error::{ParseError};
use combine::stream::{Stream};
use combine::attempt;

/**

not a complete grammer, but enough for the context

term := machine | table | rule | symbol | ident
machine := "(" "machine" ident term ")"
table := "(" "table" many1(term) ")"
rule := "(" ident symbol try(operation | underscore) ident ")"
operation := "[" sep_by(right | left | none | print | erase, ",") "]"
**/

// keywords
static MACHINE: &str = "machine";
static TABLE: &str = "table";
static ANY: &str = "any";
// chars
static ROUND_OPEN: char = '(';
static ROUND_CLOSE: char = ')';
static BOX_OPEN: char = '[';
static BOX_CLOSE: char = ']';
static UNDERSCORE: char = '_';

pub fn parse(m: &str) -> Term {
    match term().parse(m) {
        Ok((t, _)) => t,
        Err(e) => panic!("[{}]", e),
    }
}

fn ident<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let allowed_chars = || one_of("!$%&|*+-/:<=>?@^_~#.".chars());
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
    let right = char('R').map(|_| Term::Exec(Step::Move(Dir::Right)));
    let left = char('L').map(|_| Term::Exec(Step::Move(Dir::Left)));
    let none = char('N').map(|_| Term::Exec(Step::Move(Dir::None)));
    let print = (char('P'), symbol()).map(|(_, sym)| Term::Exec(Step::Effect(Kind::Print(box sym))));
    let erase = char('E').map(|_| Term::Exec(Step::Effect(Kind::Erase)));
    choice((
        attempt(string("[]").map(|_| vec!(Term::Exec(Step::Move(Dir::None))))),
        attempt(char(UNDERSCORE).map(|_| vec!(Term::Exec(Step::Move(Dir::None))))),
        attempt(between(char(BOX_OPEN), char(BOX_CLOSE), sep_by(right.or(left).or(none).or(print).or(erase), char(',').skip(spaces())))),
    )).skip(spaces())
}

fn rule<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        char(ROUND_OPEN).skip(spaces()),
        ident().skip(spaces()),
        choice((symbol(), char(UNDERSCORE).map(|_| Term::Symbol(ANY.to_string())))).skip(spaces()),
        operation(),
        ident().skip(spaces()),
        char(ROUND_CLOSE)
    ).map(|(_, mc, sym, vop, fc, _)| {
            Term::Rule(
                box mc,
                box sym,
                vop,
                box fc
            )
        })
}

fn table<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char(ROUND_OPEN), string(TABLE).skip(spaces()), sep_by(term(), spaces()), char(ROUND_CLOSE))
        .map(|(_, _, rs, _): (_, _, Vec<Term>, _)| {
            Term::Table(box rule_seq(&rs))
         })
}

fn machine<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char(ROUND_OPEN).skip(spaces()), string(MACHINE).skip(spaces()), ident().skip(spaces()), term().skip(spaces()), char(ROUND_CLOSE))
        .map(|(_, _, name, t, _)| { Term::Machine(box name, box t) })
}

fn term_<I>() -> impl Parser<Input = I, Output = Term>
    where I: Stream<Item = char>, I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(machine()),
        attempt(table()),
        attempt(rule()),
        attempt(symbol()),
        attempt(ident()),
    ))
}

parser! {
    fn term[I]()(I) -> Term
    where [I: Stream<Item = char>]
    {
        term_()
    }
}

fn rule_seq(vs: &[Term]) -> Term {
    match vs {
        [last] => last.clone(),
        [head, rest..] => Term::Seq(box head.clone(), box rule_seq(rest)),
        [] => unreachable!(),
    }
}
