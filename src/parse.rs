use ast::BinMathOp::{self, *};
use ast::Term::{self, *};
use nom::digit;
use nom::types::CompleteStr;
use std::num::ParseIntError;

named!(variable<&str, Term>, do_parse!(
    not!(tag!("if")) >>
    not!(tag!("then")) >>
    not!(tag!("else")) >>
    not!(tag!("endif")) >>
    not!(tag!("true")) >>
    not!(tag!("false")) >>
    var_str: re_find!(r"^(?i:[a-z_][a-z0-9_]*)") >>
    (Var(var_str.to_string()))));

named!(number<&str, Term>, map_res!(
    digit, |d| {
        let a: Result<Term, ParseIntError> = Ok(NumConst(u64::from_str_radix(d, 10)?));
        a
    }));

named!(boolean<&str, Term>, map_res!(alt!( tag!("true") | tag!("false")),
    |s| {let a: Result<Term, ()> = Ok(BoolConst(s == "true")); a}));

named!(lambda<&str, Term>, ws!(do_parse!(
    tag!(r"\") >>
    var: variable >>
    tag!(".") >>
    term: term >>
    ({
        match var {
            Var(var) => Lambda { var_name: var, expr: Box::new(term) },
            _ => unreachable!(),
        }
    }))));

named!(terminal<&str, Term>, alt!(
    variable | number | boolean | delimited!(char!('('), term, char!(')'))));

named!(application<&str, Term>, ws!(do_parse!(
    first: terminal >>
    rest: many0!(ws!(terminal)) >>
    (rest.into_iter().fold(first, |acc, i| {
        Apply { var_term: Box::new(i), function: Box::new(acc) }
    })))));

named!(multiplicand<&str, Term>, alt!(
    do_parse!(
        tag!("if") >>
        c: term >>
        tag!("then") >>
        t: term >>
        tag!("else") >>
        f: term >>
        tag!("endif") >>
        (IfStmt { test: Box::new(c), then_body: Box::new(t), else_body: Box::new(f) }))
    | application));

named!(addend<&str, Term>, ws!(do_parse!(
    first: multiplicand >>
    rest: many0!(ws!(tuple!(one_of!("*/"), multiplicand))) >>
    (rest.into_iter().fold(first, |acc, (op, i)| {
        let op = match op {
            '*' => Multiply,
            '/' => Divide,
            _ => unreachable!(),
        };
        MathOp { opr: op, t1: Box::new(acc), t2: Box::new(i) }
    })))));

named!(equalend<&str, Term>, ws!(do_parse!(
    first: addend >>
    rest: many0!(ws!(tuple!(one_of!("+-"), addend))) >>
    (rest.into_iter().fold(first, |acc, (op, i)| {
        let op = match op {
            '+' => Add,
            '-' => Minus,
            _ => unreachable!(),
        };
        MathOp { opr: op, t1: Box::new(acc), t2: Box::new(i) }
    })))));

named!(term<&str, Term>, ws!(do_parse!(
    left: equalend >>
    right: opt!(ws!(tuple!(alt!(tag!("==")|tag!("!=")), equalend))) >>
    (match right {
        None => left,
        Some((op, right)) => {
            match op {
                "==" => Equals { left_side: Box::new(left), right_side: Box::new(right) },
                "!=" => NotEquals { left_side: Box::new(left), right_side: Box::new(right) },
                _ => unreachable!(),
            }
        }
    }))));

#[test]
fn test_variable() {
    use nom::{
        Context::Code, Err::Error, ErrorKind::{Not, RegexpFind},
    };

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
    assert_eq!(variable("endif"), Err(Error(Code("endif", Not))));
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
fn test_lambda() {
    use nom::{Context::Code, Err::Error, ErrorKind::Alt};

    assert_eq!(
        lambda(r"\ x . x + 1;"),
        Ok((
            ";",
            Lambda {
                var_name: "x".to_string(),
                expr: Box::new(MathOp {
                    opr: Add,
                    t1: Box::new(Var("x".to_string())),
                    t2: Box::new(NumConst(1))
                })
            }
        ))
    );
}

#[test]
fn test_term() {
    use nom::{Context::Code, Err::Error, ErrorKind::Alt};

    assert_eq!(
        term(" 1 + 2 - 3 ;"),
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
        term("1 + 2 * 3 - 4 ;"),
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

    assert_eq!(
        term("1 + 2 == 3 - 4 ;"),
        //      ==
        //    /    \
        //   +      -
        //  / \    / \
        // 1   2  3   4
        Ok((
            ";",
            Equals {
                left_side: Box::new(MathOp {
                    opr: Add,
                    t1: Box::new(NumConst(1)),
                    t2: Box::new(NumConst(2))
                }),
                right_side: Box::new(MathOp {
                    opr: Minus,
                    t1: Box::new(NumConst(3)),
                    t2: Box::new(NumConst(4))
                })
            }
        ))
    );

    assert_eq!(
        term("if 1 == 2 then false else true endif ;"),
        Ok((
            ";",
            IfStmt {
                test: Box::new(Equals {
                    left_side: Box::new(NumConst(1)),
                    right_side: Box::new(NumConst(2))
                }),
                then_body: Box::new(BoolConst(false)),
                else_body: Box::new(BoolConst(true))
            }
        ))
    );
}

#[test]
fn test_apply() {
    assert_eq!(
        term("a b c + 5;"),
        Ok((
            ";",
            MathOp {
                opr: Add,
                t1: Box::new(Apply {
                    var_term: Box::new(Var("c".to_string())),
                    function: Box::new(Apply {
                            var_term: Box::new(Var("b".to_string())),
                            function: Box::new(Var("a".to_string()))
                    })
                }),
                t2: Box::new(NumConst(5))
            }
        ))
    );
}
