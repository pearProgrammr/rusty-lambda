enum Term{
    Var (String),
    Lambda {var_term: Box<Term>, function: Box<Term>},
    Apply {var_term: Box<Term>, function: Box<Term>},

    /* Constants */
    NumConst (u64),
    BoolConst (bool),

    /* Operations */
    MathOp {Opr: BinMathOp, T1: Box<Term>, T2: Box<Term>},
    IfStmt {test: Box<Term>, then_body: Box<Term>, else_body: Box<Term>},
    Equals {left_side: Box<Term>, right_side: Box<Term>},
    NotEquals {left_side: Box<Term>, right_side: Box<Term>},

    Assignm {var_name: String, Expr: Box<Term>},
}

enum BinMathOp{
    Add,
    Minus,
    Multiply,
    Divide,
}
