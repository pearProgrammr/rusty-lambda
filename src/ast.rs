#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Term <'a>{
    Var (&'static str),
    Lambda {var_name: &'static str, expr: &'a Term<'a>},
    Apply {var_term: &'a Term<'a>, function: &'a Term<'a>},

    /* Constants */
    NumConst (u64),
    BoolConst (bool),

    /* Operations */
    MathOp {opr: BinMathOp, t1: &'a Term<'a>, t2: &'a Term<'a>},
    IfStmt {test: &'a Term<'a>, then_body: &'a Term<'a>, else_body: &'a Term<'a>},
    Equals {left_side: &'a Term<'a>, right_side: &'a Term<'a>},
    NotEquals {left_side: &'a Term<'a>, right_side: &'a Term<'a>},

    Assignm {var_name: &'a str, expr: &'a Term<'a>},
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BinMathOp {
    Add,
    Minus,
    Multiply,
    Divide,
}
