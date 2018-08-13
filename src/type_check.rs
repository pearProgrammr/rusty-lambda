use ast::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TermType {
    Int,
    Bool,
    Func { env: HashMap<String, TermType>, name: String, func_term: Box<Term> },
}

/// This represents a binding between names and TermTypes.
pub struct TyEnv(pub HashMap<String, TermType>);

// Main Type checking function.
// This evaluates a term to a TermType or throw an error.
pub fn type_check(term: &Term, env: &TyEnv) -> Result<TermType, String> {
    match term {
        Term::Var(n) => Ok(env
            .0
            .get(n)
            .ok_or("Variable name missing in environment")?
            .clone()),
        Term::Lambda { var_name, expr } => Ok(TermType::Func {
            env: env.0.clone(),
            name: var_name.clone(),
            func_term: expr.clone(),
        }),
        Term::Apply { var_term, function } => match type_check(function, env)? {
            TermType::Func { env: term_env, name, func_term } => {
                let var_type = type_check(var_term, env)?;
                let mut env_prime = env.0.clone();
                for (k,v) in term_env {
                    env_prime.insert(k, v);
                }
                env_prime.insert(name.clone(), var_type);
                type_check(&func_term, &TyEnv(env_prime))
            }
            _ => Err("terms need to be applied to function types".to_string()),
        },
        Term::NumConst(_) => Ok(TermType::Int),
        Term::BoolConst(_) => Ok(TermType::Bool),
        Term::MathOp { t1, t2, .. } => {
            type_check_bin_math_op(&type_check(t1, env)?, &type_check(t2, env)?)
        }
        Term::Equals {
            left_side: t1,
            right_side: t2,
        } => type_check_bin_logic_op(&type_check(t1, env)?, &type_check(t2, env)?),
        Term::NotEquals {
            left_side: t1,
            right_side: t2,
        } => type_check_bin_logic_op(&type_check(t1, env)?, &type_check(t2, env)?),
        Term::IfStmt {
            test: c,
            then_body: tb,
            else_body: eb,
        } => type_check_if(
            &type_check(c, env)?,
            &type_check(tb, env)?,
            &type_check(eb, env)?,
        ),
        Term::Assignm { expr, .. } => type_check(expr, env),
    }
}

fn type_check_bin_math_op(t1: &TermType, t2: &TermType) -> Result<TermType, String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok(TermType::Int),
        (_, _) => {
            Err("Mathematical binary operators require both parameters to be integers".to_string())
        }
    }
}

fn type_check_bin_logic_op(t1: &TermType, t2: &TermType) -> Result<TermType, String> {
    match (t1, t2) {
        (TermType::Int, TermType::Int) => Ok(TermType::Bool),
        (TermType::Bool, TermType::Bool) => Ok(TermType::Bool),
        (_, _) => Err(
            "Logical binary operators require both parameters to be integers or booleans"
                .to_string(),
        ),
    }
}

fn type_check_if(c: &TermType, tb: &TermType, eb: &TermType) -> Result<TermType, String> {
    match c {
        TermType::Bool => match_type(tb, eb),
        _ => Err("Condition to if must be a boolean".to_string()),
    }
}

fn match_type(t1: &TermType, t2: &TermType) -> Result<TermType, String> {
    if t1 == t2 {
        Ok(t1.clone())
    } else {
        Err("Types do not match".to_string()) // be more descriptive. TODO: write a pretty printer for types
    }
}

/* Tests */

#[test]
fn test_num_const() {
    let te = TyEnv(HashMap::new());
    let ast = Term::NumConst(4);
    assert_eq!(Ok(TermType::Int), type_check(&ast, &te));
}

#[test]
fn test_bool_const() {
    let te = TyEnv(HashMap::new());
    let ast = Term::BoolConst(false);
    assert_eq!(Ok(TermType::Bool), type_check(&ast, &te));
}

#[test]
fn test_bool_bin() {
    let te = TyEnv(HashMap::new());
    let ast = Term::Equals {
        left_side: Box::new(Term::BoolConst(false)),
        right_side: Box::new(Term::BoolConst(false)),
    };
    assert_eq!(Ok(TermType::Bool), type_check(&ast, &te));
}

#[test]
fn test_bool_bin_int() {
    let te = TyEnv(HashMap::new());
    let ast = Term::Equals {
        left_side: Box::new(Term::NumConst(5)),
        right_side: Box::new(Term::NumConst(6)),
    };
    assert_eq!(Ok(TermType::Bool), type_check(&ast, &te));
}

#[test]
fn test_int_bin_int() {
    let te = TyEnv(HashMap::new());
    let ast = Term::MathOp {
        opr: BinMathOp::Add,
        t1: Box::new(Term::NumConst(5)),
        t2: Box::new(Term::NumConst(6)),
    };
    assert_eq!(Ok(TermType::Int), type_check(&ast, &te));
}

#[test]
fn test_int_bin_int_nested() {
    let te = TyEnv(HashMap::new());
    let ast = Term::MathOp {
        opr: BinMathOp::Minus,
        t1: Box::new(Term::NumConst(5)),
        t2: Box::new(Term::MathOp {
            opr: BinMathOp::Multiply,
            t1: Box::new(Term::MathOp {
                opr: BinMathOp::Divide,
                t1: Box::new(Term::NumConst(24)),
                t2: Box::new(Term::NumConst(8)),
            }),
            t2: Box::new(Term::NumConst(8)),
        }),
    };
    assert_eq!(Ok(TermType::Int), type_check(&ast, &te));
}

#[test]
fn test_bool_bin_nested() {
    let te = TyEnv(HashMap::new());
    let ast = Term::NotEquals {
        left_side: Box::new(Term::BoolConst(false)),
        right_side: Box::new(Term::Equals {
            left_side: Box::new(Term::NumConst(4)),
            right_side: Box::new(Term::NumConst(1000)),
        }),
    };
    assert_eq!(Ok(TermType::Bool), type_check(&ast, &te));
}

#[test]
fn test_var_does_not_exist() {
    let te = TyEnv(HashMap::new());
    let ast = Term::Var("v1".to_string());
    assert_eq!(
        Err("Variable name missing in environment".to_string()),
        type_check(&ast, &te)
    );
}

#[test]
fn if_test_1() {
    let te = TyEnv(HashMap::new());
    let ast = Term::IfStmt {
        test: Box::new(Term::BoolConst(true)),
        then_body: Box::new(Term::NumConst(1)),
        else_body: Box::new(Term::NumConst(8)),
    };
    assert_eq!(Ok(TermType::Int), type_check(&ast, &te));
}

#[test]
fn if_test_2() {
    let te = TyEnv(HashMap::new());
    let ast = Term::IfStmt {
        test: Box::new(Term::NumConst(9)),
        then_body: Box::new(Term::NumConst(1)),
        else_body: Box::new(Term::NumConst(8)),
    };
    assert_eq!(
        Err("Condition to if must be a boolean".to_string()),
        type_check(&ast, &te)
    );
}

#[test]
fn test_var_exists() {
    let mut map = HashMap::new();
    map.insert("v1".to_string(), TermType::Int);
    let te = TyEnv(map);
    let ast = Term::Var("v1".to_string());
    assert_eq!(Ok(TermType::Int), type_check(&ast, &te));
}

#[test]
fn test_simple_lambda_1() {
    let math_func = Term::MathOp {
        opr: BinMathOp::Add,
        t1: Box::new(Term::Var("v1".to_string())),
        t2: Box::new(Term::NumConst(6)),
    };
    let math_func2 = math_func.clone();

    let te = TyEnv(HashMap::new());
    let ast = Term::Lambda {
        var_name: "v1".to_string(),
        expr: Box::new(math_func),
    };

    assert_eq!(
        Ok(TermType::Func {
            env: HashMap::new(),
            name: "v1".to_string(),
            func_term: Box::new(math_func2),
        }),
        type_check(&ast, &te)
    );
}

#[test]
fn test_apply_1() {
    let math_expr = Term::MathOp {
        opr: BinMathOp::Add,
        t1: Box::new(Term::Var("v1".to_string())),
        t2: Box::new(Term::NumConst(6)),
    };
    let math_func = Term::Lambda {
        var_name: "v1".to_string(),
        expr: Box::new(math_expr),
    };

    let te = TyEnv(HashMap::new());
    let ast = Term::Apply {
        var_term: Box::new(Term::NumConst(1)),
        function: Box::new(math_func),
    };

    assert_eq!(Ok(TermType::Int), type_check(&ast, &te));
}

#[test]
/// Overrides existing v1
fn test_apply_2() {
    let math_expr = Term::MathOp {
        opr: BinMathOp::Add,
        t1: Box::new(Term::Var("v1".to_string())),
        t2: Box::new(Term::NumConst(6)),
    };
    let math_func = Term::Lambda {
        var_name: "v1".to_string(),
        expr: Box::new(math_expr),
    };

    let mut map = HashMap::new();
    map.insert("v1".to_string(), TermType::Bool);
    let te = TyEnv(map);

    let ast = Term::Apply {
        var_term: Box::new(Term::NumConst(1)),
        function: Box::new(math_func),
    };

    assert_eq!(Ok(TermType::Int), type_check(&ast, &te));
}
