use ast::*;
use std::collections::HashMap;
use value::*;

/// This represents a binding between names and TermTypes.
pub struct EvalEnv(pub HashMap<String, Value>);

/// Error messages

static EVAL_MATH_ERROR: &'static str =
    "Invalid math operation. Both sides must evaluate to numbers";
static EVAL_BOOL_ERROR: &'static str =
    "Both terms in equality must be of the same type. Equality on functions are not supported";
static EVAL_INVALID_TERM: &'static str = "Invalid term";
static EVAL_IF_COND_REQUIRES_BOOL: &'static str = "test condition must be a boolean";

/// Main evaluation function. This part of the code assumes that the types are
/// correct. Although certain patterns would be impossible to reach after type
/// checking, they are included for completeness... and to satisfy the rust
/// compiler
pub fn eval(node: &Term, env: &EvalEnv) -> Result<Value, String> {
    match node {
        Term::Var(n) => Ok(env
            .0
            .get(n)
            .ok_or("Variable name missing in environment")?
            .clone()),
        Term::Lambda { var_name, expr } => Ok(Value::Func {
            name: var_name.clone(),
            func_term: expr.clone(),
        }),
        Term::Apply { var_term, function } => match eval(function, env)? {
            Value::Func { name, func_term } => {
                let var_val = eval(var_term, env)?;
                let mut env_prime = env.0.clone();
                env_prime.insert(name.clone(), var_val);
                eval(&func_term, &EvalEnv(env_prime))
            }
            _ => Err("terms need to be applied to function types".to_string()),
        },
        Term::NumConst(n) => Ok(Value::Num(*n)),
        Term::BoolConst(b) => Ok(Value::Bool(*b)),
        Term::MathOp { opr, t1, t2 } => eval_bin_math_op(opr, eval(t1, env)?, eval(t2, env)?),
        Term::Equals {
            left_side: t1,
            right_side: t2,
        } => eval_equals(eval(t1, env)?, eval(t2, env)?),
        Term::NotEquals {
            left_side: t1,
            right_side: t2,
        } => eval_not_equals(eval(t1, env)?, eval(t2, env)?),
        Term::IfStmt {
            test: c,
            then_body: tb,
            else_body: eb,
        } => eval_if(eval(c, env)?, tb, eb, env),
        Term::Assignm { var_name, expr } => Ok(Value::Assignm {
            name: var_name.clone(),
            val: Box::new(eval(expr, env)?),
        }),
        _ => Err(EVAL_INVALID_TERM.to_string()),
    }
}

fn eval_bin_math_op(opr: &BinMathOp, t1: Value, t2: Value) -> Result<Value, String> {
    match (opr, t1, t2) {
        (BinMathOp::Add, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 + v2)),
        (BinMathOp::Minus, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 - v2)),
        (BinMathOp::Multiply, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 * v2)),
        (BinMathOp::Divide, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 / v2)),
        _ => Err(EVAL_MATH_ERROR.to_string()),
    }
}

fn eval_equals(t1: Value, t2: Value) -> Result<Value, String> {
    match (t1, t2) {
        (Value::Num(num1), Value::Num(num2)) => Ok(Value::Bool(num1 == num2)),
        (Value::Bool(bool1), Value::Bool(bool2)) => Ok(Value::Bool(bool1 == bool2)),
        (_, _) => Err(EVAL_BOOL_ERROR.to_string()),
    }
}

fn eval_not_equals(t1: Value, t2: Value) -> Result<Value, String> {
    match (t1, t2) {
        (Value::Num(num1), Value::Num(num2)) => Ok(Value::Bool(num1 != num2)),
        (Value::Bool(bool1), Value::Bool(bool2)) => Ok(Value::Bool(bool1 != bool2)),
        (_, _) => Err(EVAL_BOOL_ERROR.to_string()),
    }
}

// Evaluates if/then/else statement.
// Note: at this point, type checking should have ensured that both branches of the condition
// have the same type.
fn eval_if(
    test: Value,
    then_body: &Term,
    else_body: &Term,
    env: &EvalEnv,
) -> Result<Value, String> {
    match test {
        Value::Bool(true) => eval(then_body, env),
        Value::Bool(false) => eval(else_body, env),
        _ => Err(EVAL_IF_COND_REQUIRES_BOOL.to_string()),
    }
}

#[test]
fn test_ev_const_vals() {
    let ast_num = Term::NumConst(1);
    let env = EvalEnv(HashMap::new());
    assert_eq!(Ok(Value::Num(1)), eval(&ast_num, &env));

    let ast_bool = Term::BoolConst(true);
    assert_eq!(Ok(Value::Bool(true)), eval(&ast_bool, &env));
}

#[test]
fn test_mathops() {
    let add_expr = Term::MathOp {
        opr: BinMathOp::Add,
        t1: Box::new(Term::NumConst(4)),
        t2: Box::new(Term::NumConst(6)),
    };
    let sub_expr = Term::MathOp {
        opr: BinMathOp::Minus,
        t1: Box::new(Term::NumConst(6)),
        t2: Box::new(Term::NumConst(4)),
    };
    let mul_expr = Term::MathOp {
        opr: BinMathOp::Multiply,
        t1: Box::new(Term::NumConst(6)),
        t2: Box::new(Term::NumConst(4)),
    };
    let div_expr = Term::MathOp {
        opr: BinMathOp::Divide,
        t1: Box::new(Term::NumConst(6)),
        t2: Box::new(Term::NumConst(3)),
    };
    let incorrect_1 = Term::MathOp {
        opr: BinMathOp::Divide,
        t1: Box::new(Term::BoolConst(true)),
        t2: Box::new(Term::NumConst(3)),
    };
    let incorrect_2 = Term::MathOp {
        opr: BinMathOp::Divide,
        t1: Box::new(Term::NumConst(3)),
        t2: Box::new(Term::BoolConst(true)),
    };

    let env = EvalEnv(HashMap::new());
    assert_eq!(Ok(Value::Num(2)), eval(&sub_expr, &env));
    assert_eq!(Ok(Value::Num(10)), eval(&add_expr, &env));
    assert_eq!(Ok(Value::Num(24)), eval(&mul_expr, &env));
    assert_eq!(Ok(Value::Num(2)), eval(&div_expr, &env));
    assert_eq!(Err(EVAL_MATH_ERROR.to_string()), eval(&incorrect_1, &env));
    assert_eq!(Err(EVAL_MATH_ERROR.to_string()), eval(&incorrect_2, &env));
}

#[test]
fn test_boolops() {
    let eq_expr_1 = Term::Equals {
        left_side: Box::new(Term::NumConst(4)),
        right_side: Box::new(Term::NumConst(6)),
    };
    let eq_expr_2 = Term::Equals {
        left_side: Box::new(Term::NumConst(6)),
        right_side: Box::new(Term::NumConst(6)),
    };
    let eq_expr_3 = Term::NotEquals {
        left_side: Box::new(Term::BoolConst(false)),
        right_side: Box::new(Term::BoolConst(false)),
    };
    let eq_expr_4 = Term::NotEquals {
        left_side: Box::new(Term::BoolConst(true)),
        right_side: Box::new(Term::BoolConst(false)),
    };
    let eq_expr_5 = Term::NotEquals {
        left_side: Box::new(Term::NumConst(1)),
        right_side: Box::new(Term::BoolConst(false)),
    };
    let env = EvalEnv(HashMap::new());
    assert_eq!(Ok(Value::Bool(false)), eval(&eq_expr_1, &env));
    assert_eq!(Ok(Value::Bool(true)), eval(&eq_expr_2, &env));
    assert_eq!(Ok(Value::Bool(false)), eval(&eq_expr_3, &env));
    assert_eq!(Ok(Value::Bool(true)), eval(&eq_expr_4, &env));
    assert_eq!(Err(EVAL_BOOL_ERROR.to_string()), eval(&eq_expr_5, &env));
}

#[test]
fn test_if() {
    let if_1 = Term::IfStmt {
        test: Box::new(Term::BoolConst(true)),
        then_body: Box::new(Term::NumConst(6)),
        else_body: Box::new(Term::NumConst(7)),
    };
    let if_2 = Term::IfStmt {
        test: Box::new(Term::NumConst(1)),
        then_body: Box::new(Term::NumConst(6)),
        else_body: Box::new(Term::NumConst(7)),
    };
    let if_3 = Term::IfStmt {
        test: Box::new(Term::BoolConst(false)),
        then_body: Box::new(Term::NumConst(6)),
        else_body: Box::new(Term::NumConst(7)),
    };
    let env = EvalEnv(HashMap::new());
    assert_eq!(Ok(Value::Num(6)), eval(&if_1, &env));
    assert_eq!(
        Err(EVAL_IF_COND_REQUIRES_BOOL.to_string()),
        eval(&if_2, &env)
    );
    assert_eq!(Ok(Value::Num(7)), eval(&if_3, &env));
}
