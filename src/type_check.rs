use ast::*;
use value::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum TermType {
    Int,
    Bool,
    Func {t1: Box<TermType>, t2:Box<TermType>},
}

/// This represents a binding between names and TermTypes.
type TyEnv = HashMap <String, TermType>;

/// Main Type checking function.
/// This evaluates a term to a TermType or throw an error.
fn type_check(node: Term, env: &TyEnv) -> Result<(TermType, &TyEnv), String> {
    match node {
        Term::NumConst (n) => Ok((TermType::Int, env)),
        Term::BoolConst(b) => Ok((TermType::Bool, env)),
        Term::MathOp {opr, t1, t2} => type_check_bin_math_op(type_check(*t1, env)?.0, type_check(*t2, env)?.0, env),
        Term::Equals {left_side: t1, right_side: t2} => type_check_bin_logic_op (type_check(*t1, env)?.0, type_check(*t2, env)?.0, env),
        Term::NotEquals {left_side: t1, right_side: t2} => type_check_bin_logic_op (type_check(*t1, env)?.0,type_check(*t2, env)?.0, env),
        Term::IfStmt {test: c, then_body: tb, else_body: eb} => type_check_if (type_check(*c, env)?.0, type_check(*tb, env)?.0, type_check(*eb, env)?.0, env),
        _ => Err("Type checker: Invalid term".to_string()),
   }
}

fn type_check_bin_math_op(t1: TermType, t2: TermType, env: &TyEnv) -> Result<(TermType, &TyEnv), String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok((TermType::Int, env)),
        (_,_) => Err("Mathematical binary operators require both parameters to be integers".to_string()),
    }
}

fn type_check_bin_logic_op(t1: TermType, t2: TermType, env: &TyEnv) -> Result<(TermType, &TyEnv), String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok((TermType::Bool, env)),
        (TermType::Bool, TermType::Bool) => Ok((TermType::Bool, env)),
        (_,_) => Err("Logical binary operators require both parameters to be integers or booleans".to_string()),
    }
}

fn type_check_if(c: TermType, tb: TermType, eb: TermType, env: &TyEnv) -> Result<(TermType, &TyEnv), String> {
    match c {
       TermType::Bool => match_type (tb, eb, env),
       _ => Err("Condition to if must be a boolean".to_string()),
    }
}

fn match_type (t1: TermType, t2: TermType, env: &TyEnv) -> Result<(TermType, &TyEnv), String> {
    if t1 == t2{
        Ok((t1,env))
    }
    else {
        Err("Types do not match".to_string()) // be more descriptive. TODO: write a pretty printer for types
    }
}
