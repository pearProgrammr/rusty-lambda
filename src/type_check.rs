use ast::*;
use value::*;

#[derive(PartialEq, Eq)]
enum TermType {
    Int,
    Bool,
    Func {t1: Box<TermType>, t2:Box<TermType>},
}

/// Main Type checking function.
/// This evaluates a term to a TermType or throw an error.
fn type_check(node: Term) -> Result<TermType, String> {
    match node {
        Term::NumConst (n) => Ok(TermType::Int),
        Term::BoolConst(b) => Ok(TermType::Bool),
        Term::MathOp {Opr:opr, T1: t1, T2: t2} => type_check_bin_math_op(type_check(*t1).unwrap(), type_check(*t2).unwrap()),
        Term::Equals {left_side: t1, right_side: t2} => type_check_bin_logic_op (type_check(*t1).unwrap(), type_check(*t2).unwrap()),
        Term::NotEquals {left_side: t1, right_side: t2} => type_check_bin_logic_op (type_check(*t1).unwrap(),type_check(*t2).unwrap()),
        Term::IfStmt {test: c, then_body: tb, else_body: eb} => type_check_if (type_check(*c).unwrap(), type_check(*tb).unwrap(), type_check(*eb).unwrap()),
        _ => Err("Type checker: Invalid term".to_string()),
   }
}

fn type_check_bin_math_op(t1: TermType, t2: TermType) -> Result<TermType, String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok(TermType::Int),
        (_,_) => Err("Mathematical binary operators require both parameters to be integers".to_string()),
    }
}

fn type_check_bin_logic_op(t1: TermType, t2: TermType) -> Result<TermType, String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok(TermType::Bool),
        (TermType::Bool, TermType::Bool) => Ok(TermType::Bool),
        (_,_) => Err("Logical binary operators require both parameters to be integers or booleans".to_string()),
    }
}

fn type_check_if(c: TermType, tb: TermType, eb: TermType) -> Result<TermType, String> {
    match c {
       TermType::Bool => match_type (tb, eb),
       _ => Err("Condition to if must be a boolean".to_string()),
    }
}

fn match_type (t1: TermType, t2: TermType) -> Result<TermType, String> {
    if t1 == t2{
        Ok(t1)
    }
    else {
        Err("Types do not match".to_string()) // be more descriptive. TODO: write a pretty printer for types
    }
}
