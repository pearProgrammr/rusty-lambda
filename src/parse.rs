use ast::BinMathOp::{self, *};
use ast::Term::{self, *};
use nom::digit;
use nom::types::CompleteStr;
use std::num::ParseIntError;

named!(variable<&str, Term>, do_parse!(
    var_str: re_find!(r"^(?i:[a-z_][a-z0-9_]*)") >>
    (Var(var_str.to_string()))));

named!(number<&str, Term>, map_res!(digit, |d| {let a: Result<Term, ParseIntError> = Ok(NumConst(u64::from_str_radix(d, 10)?)); a}));

named!(boolean<&str, Term>, map_res!(alt!( tag!("true") | tag!("false")),
    |s| {let a: Result<Term, ()> = Ok(BoolConst(s == "true")); a}));

named!(multiplicand<&str, Term>, do_parse!(n: number >> (n)));

named!(addend<&str, Term>, do_parse!(
    first: multiplicand >>
    rest: many0!(tuple!(one_of!("*/"), multiplicand)) >>
    (rest.into_iter().fold(first, |acc, (op, i)| {
        let op = match op {
            '*' => Multiply,
            '/' => Divide,
            _ => unreachable!(),
        };
        MathOp { opr: op, t1: Box::new(acc), t2: Box::new(i) }
    }))));

named!(term<&str, Term>, do_parse!(
    first: addend >>
    rest: many0!(tuple!(one_of!("+-"), addend)) >>
    (rest.into_iter().fold(first, |acc, (op, i)| {
        let op = match op {
            '+' => Add,
            '-' => Minus,
            _ => unreachable!(),
        };
        MathOp { opr: op, t1: Box::new(acc), t2: Box::new(i) }
    }))));

#[test]
fn test_variable() {
    use nom::{Context::Code, Err::Error, ErrorKind::RegexpFind};

    assert_eq!(
        variable("_things{}"),
        Ok(("{}", Var("_things".to_string())))
    );
    assert_eq!(
        variable("_things _stuff"),
        Ok((" _stuff", Var("_things".to_string())))
    );
    assert_eq!(
        variable("1_things::/"),
        Err(Error(Code("1_things::/", RegexpFind)))
    );
}

#[test]
fn test_number() {
    use nom::{Context::Code, Err::Error, ErrorKind::Digit};

    assert_eq!(number("13potato"), Ok(("potato", NumConst(13))));
    assert_eq!(number("potato13"), Err(Error(Code("potato13", Digit))));
}

#[test]
fn test_boolean() {
    use nom::{Context::Code, Err::Error, ErrorKind::Alt};

    assert_eq!(boolean("truefalse"), Ok(("false", BoolConst(true))));
    assert_eq!(boolean("falsefalse"), Ok(("false", BoolConst(false))));
    assert_eq!(boolean("falsfalse"), Err(Error(Code("falsfalse", Alt))));
}

#[test]
fn test_term() {
    use nom::{Context::Code, Err::Error, ErrorKind::Alt};

    assert_eq!(
        term("1+2-3;"),
        Ok((
            ";",
            //     -
            //    / \
            //   +   3
            //  / \
            // 1   2
            MathOp {
                opr: Minus,
                t1: Box::new(MathOp {
                    opr: Add,
                    t1: Box::new(NumConst(1)),
                    t2: Box::new(NumConst(2))
                }),
                t2: Box::new(NumConst(3))
            }
        ))
    );

    assert_eq!(
        term("1+2*3-4;"),
        //     -
        //    / \
        //   +   4
        //  / \
        // 1   *
        //    / \
        //   2   3
        Ok((
            ";",
            MathOp {
                opr: Minus,
                t1: Box::new(MathOp {
                    opr: Add,
                    t1: Box::new(NumConst(1)),
                    t2: Box::new(MathOp {
                        opr: Multiply,
                        t1: Box::new(NumConst(2)),
                        t2: Box::new(NumConst(3))
                    })
                }),
                t2: Box::new(NumConst(4))
            }
        ))
    );
}
