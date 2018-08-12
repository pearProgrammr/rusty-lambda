use ast::BinMathOp::*;
use ast::Term::{self, *};
use nom::digit;
use nom::types::CompleteStr;
use std::num::ParseIntError;

named!(variable<CompleteStr, Term>, do_parse!(
    not!(tag!("if")) >>
    not!(tag!("then")) >>
    not!(tag!("else")) >>
    not!(tag!("endif")) >>
    not!(tag!("true")) >>
    not!(tag!("false")) >>
    var_str: re_find!(r"^(?i:[a-z_][a-z0-9_]*)") >>
    (Var(var_str.to_string()))));

named!(number<CompleteStr, Term>, map_res!(
    digit, |d: CompleteStr| {
        let a: Result<Term, ParseIntError> = Ok(NumConst(u64::from_str_radix(*d, 10)?));
        a
    }));

named!(boolean<CompleteStr, Term>, map_res!(alt!( tag!("true") | tag!("false")),
    |s: CompleteStr| {let a: Result<Term, ()> = Ok(BoolConst(*s == "true")); a}));

named!(lambda<CompleteStr, Term>, ws!(do_parse!(
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

named!(terminal<CompleteStr, Term>, alt!(
    variable | number | boolean | delimited!(char!('('), alt!(lambda | term), char!(')'))));

named!(application<CompleteStr, Term>, ws!(do_parse!(
    first: terminal >>
    rest: many0!(ws!(terminal)) >>
    (rest.into_iter().fold(first, |acc, i| {
        Apply { var_term: Box::new(i), function: Box::new(acc) }
    })))));

named!(multiplicand<CompleteStr, Term>, alt!(
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

named!(addend<CompleteStr, Term>, ws!(do_parse!(
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

named!(equalend<CompleteStr, Term>, ws!(do_parse!(
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

named!(term<CompleteStr, Term>, ws!(do_parse!(
    left: equalend >>
    right: opt!(ws!(tuple!(alt!(tag!("==")|tag!("!=")), equalend))) >>
    (match right {
        None => left,
        Some((op, right)) => {
            match *op {
                "==" => Equals { left_side: Box::new(left), right_side: Box::new(right) },
                "!=" => NotEquals { left_side: Box::new(left), right_side: Box::new(right) },
                _ => unreachable!(),
            }
        }
    }))));

named!(assignment<CompleteStr, Term>, ws!(do_parse!(
    var_name: variable >>
    tag!(":=") >>
    expr: term >>
    (match var_name {
        Var(var_name) => Assignm { var_name, expr: Box::new(expr) },
        _ => unreachable!(),
    }))));

#[test]
fn test_variable() {
    use nom::{
        Context::Code, Err::Error, ErrorKind::{Not, RegexpFind},
    };

    assert_eq!(
        variable(CompleteStr("_things{}")),
        Ok((CompleteStr("{}"), Var("_things".to_string())))
    );
    assert_eq!(
        variable(CompleteStr("_things _stuff")),
        Ok((CompleteStr(" _stuff"), Var("_things".to_string())))
    );
    assert_eq!(
        variable(CompleteStr("1_things::/")),
        Err(Error(Code(CompleteStr("1_things::/"), RegexpFind)))
    );
    assert_eq!(
        variable(CompleteStr("endif")),
        Err(Error(Code(CompleteStr("endif"), Not)))
    );
}

#[test]
fn test_number() {
    use nom::{Context::Code, Err::Error, ErrorKind::Digit};

    assert_eq!(
        number(CompleteStr("13potato")),
        Ok((CompleteStr("potato"), NumConst(13)))
    );
    assert_eq!(
        number(CompleteStr("potato13")),
        Err(Error(Code(CompleteStr("potato13"), Digit)))
    );
}

#[test]
fn test_boolean() {
    use nom::{Context::Code, Err::Error, ErrorKind::Alt};

    assert_eq!(
        boolean(CompleteStr("truefalse")),
        Ok((CompleteStr("false"), BoolConst(true)))
    );
    assert_eq!(
        boolean(CompleteStr("falsefalse")),
        Ok((CompleteStr("false"), BoolConst(false)))
    );
    assert_eq!(
        boolean(CompleteStr("falsfalse")),
        Err(Error(Code(CompleteStr("falsfalse"), Alt)))
    );
}

#[test]
fn test_lambda() {
    assert_eq!(
        lambda(CompleteStr(r"\ x . x + 1")),
        Ok((
            CompleteStr(""),
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
    assert_eq!(
        term(CompleteStr(" 1 + 2 - 3  ")),
        Ok((
            CompleteStr(""),
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
        term(CompleteStr("1 + 2 * 3 - 4  ")),
        //     -
        //    / \
        //   +   4
        //  / \
        // 1   *
        //    / \
        //   2   3
        Ok((
            CompleteStr(""),
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
        term(CompleteStr("1 + 2 == 3 - 4  ")),
        //      ==
        //    /    \
        //   +      -
        //  / \    / \
        // 1   2  3   4
        Ok((
            CompleteStr(""),
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
        term(CompleteStr("if 1 == 2 then false else true endif  ")),
        Ok((
            CompleteStr(""),
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
        term(CompleteStr("a b c + 5")),
        Ok((
            CompleteStr(""),
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

#[test]
fn test_assignment() {
    assert_eq!(
        assignment(CompleteStr("_a := 1 + 1")),
        Ok((
            CompleteStr(""),
            Assignm {
                var_name: "_a".to_string(),
                expr: Box::new(MathOp {
                    opr: Add,
                    t1: Box::new(NumConst(1)),
                    t2: Box::new(NumConst(1))
                })
            }
        ))
    );
}
