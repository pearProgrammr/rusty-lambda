use ast::*;
use value::*;

/// Main evaluation function. This part of the code assumes that the types are
/// correct. Although certain patterns would be impossible to reach after type
/// checking, they are included for completeness... and to satisfy the rust
/// compiler
fn eval(node: Term) -> Result<Value, String> {
    match node {
        Term::NumConst(n) => Ok(Value::Num(n)),
        Term::BoolConst(b) => Ok(Value::Bool(b)),
        Term::MathOp { opr, t1, t2 } => {
            eval_bin_math_op(opr, eval(*t1).unwrap(), eval(*t2).unwrap())
        }
        Term::Equals {
            left_side: t1,
            right_side: t2,
        } => eval_equals(eval(*t1).unwrap(), eval(*t2).unwrap()),
        Term::NotEquals {
            left_side: t1,
            right_side: t2,
        } => eval_not_equals(eval(*t1).unwrap(), eval(*t2).unwrap()),
        Term::IfStmt {
            test: c,
            then_body: tb,
            else_body: eb,
        } => eval_if(eval(*c).unwrap(), *tb, *eb),
        _ => Err("Invalid term".to_string()),
    }
}

fn eval_bin_math_op(opr: BinMathOp, t1: Value, t2: Value) -> Result<Value, String> {
    match (opr, t1, t2) {
        (BinMathOp::Add, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 + v2)),
        (BinMathOp::Minus, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 - v2)),
        (BinMathOp::Multiply, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 * v2)),
        (BinMathOp::Divide, Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(v1 / v2)),
        _ => Err("Invalid math operation".to_string()),
    }
}

fn eval_equals(t1: Value, t2: Value) -> Result<Value, String> {
    match (t1, t2) {
        (Value::Num(num1), Value::Num(num2)) => Ok(Value::Bool (num1 == num2)),
        (Value::Bool(bool1), Value::Bool(bool2)) => Ok(Value::Bool (bool1==bool2)),
        (_,_) => Err("Both terms in equality must be of the same type. Equality on functions are not supported.".to_string()),
    }
}

fn eval_not_equals(t1: Value, t2: Value) -> Result<Value, String> {
    match (t1, t2) {
        (Value::Num(num1), Value::Num(num2)) => Ok(Value::Bool (num1 != num2)),
        (Value::Bool(bool1), Value::Bool(bool2)) => Ok(Value::Bool (bool1 != bool2)),
        (_,_) => Err("Both terms in non-equality must be of the same type. Equality on functions are not supported.".to_string()),
    }
}

/// Evaluates if/then/else statement.
/// Note: at this point, type checking should have ensured that both branches of the condition
/// have the same type.
fn eval_if(test: Value, then_body: Term, else_body: Term) -> Result<Value, String> {
    match test {
        Value::Bool(true) => eval(then_body),
        Value::Bool(false) => eval(else_body),
        _ => Err("test condition must be a boolean".to_string()),
    }
}
