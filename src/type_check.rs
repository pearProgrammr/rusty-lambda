
use ast::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TermType <'a>{
    Int,
    Bool,
    Func {name: &'a str, t1: &'a Term<'a>},
}

/// This represents a binding between names and TermTypes.
struct TyEnv <'a>{
    map: HashMap <String, &'a TermType<'a>>
}

impl<'a> TyEnv<'a> {
    fn get_val(&self, name: &str) -> Result<TermType <'a>, String> {
            match self.map.get(name) {
                 Some(t) => Ok(**t),
                 _ => Err (" does not exist".to_string()),
            }
    }
}

// Main Type checking function.
// This evaluates a term to a TermType or throw an error.
fn type_check<'a>(term: Term<'a>, env: &'a TyEnv<'a>) -> Result<(TermType<'a>, &'a TyEnv<'a>), String> {
    match term{
        Term::Var(n) => Ok((env.get_val(n)?, env)),
        Term::Lambda{var_name: v, expr: e} => Ok((TermType::Func{name: v, t1: e}, env)),
        Term::NumConst (n) => Ok((TermType::Int, env)),
        Term::BoolConst(b) => Ok((TermType::Bool, env)),
        Term::MathOp {opr, t1, t2} => type_check_bin_math_op(type_check(*t1, env)?.0, type_check(*t2, env)?.0, env),
        Term::Equals {left_side: t1, right_side: t2} => type_check_bin_logic_op (type_check(*t1, env)?.0, type_check(*t2, env)?.0, env),
        Term::NotEquals {left_side: t1, right_side: t2} => type_check_bin_logic_op (type_check(*t1, env)?.0,type_check(*t2, env)?.0, env),
        Term::IfStmt {test: c, then_body: tb, else_body: eb} => type_check_if (type_check(*c, env)?.0, type_check(*tb, env)?.0, type_check(*eb, env)?.0, env),
        _ => Err("Type checker: Invalid term".to_string()),
   }
}

fn type_check_bin_math_op<'a>(t1: TermType, t2: TermType, env: &'a TyEnv<'a>) -> Result<(TermType<'a>, &'a TyEnv<'a>), String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok((TermType::Int, env)),
        (_,_) => Err("Mathematical binary operators require both parameters to be integers".to_string()),
    }
}

fn type_check_bin_logic_op<'a>(t1: TermType, t2: TermType, env: &'a TyEnv<'a>) -> Result<(TermType<'a>, &'a TyEnv<'a>), String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok((TermType::Bool, env)),
        (TermType::Bool, TermType::Bool) => Ok((TermType::Bool, env)),
        (_,_) => Err("Logical binary operators require both parameters to be integers or booleans".to_string()),
    }
}

/// Borrow checker gets thrown off from returning t1 in match_type so we need to annotate it with <'a>
fn type_check_if<'a>(c: TermType, tb: TermType<'a>, eb: TermType, env: &'a TyEnv<'a>) -> Result<(TermType<'a>, &'a TyEnv<'a>), String> {
    match c {
       TermType::Bool => match_type (tb, eb, env),
       _ => Err("Condition to if must be a boolean".to_string()),
    }
}

/// Borrow checker gets thrown off from returning t1 so we need to annotate it with <'a>
fn match_type<'a>(t1: TermType<'a>, t2: TermType, env: &'a TyEnv<'a>) -> Result<(TermType<'a>, &'a TyEnv<'a>), String> {
    if t1 == t2{
        Ok((t1,env))
    }
    else {
        Err("Types do not match".to_string()) // be more descriptive. TODO: write a pretty printer for types
    }
}
