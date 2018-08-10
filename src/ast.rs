#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Term {
    Var(String),
    Lambda {
        var_name: String,
        expr: Box<Term>,
    },
    Apply {
        var_term: Box<Term>,
        function: Box<Term>,
    },

    /* Constants */
    NumConst(u64),
    BoolConst(bool),

    /* Operations */
    MathOp {
        opr: BinMathOp,
        t1: Box<Term>,
        t2: Box<Term>,
    },
    IfStmt {
        test: Box<Term>,
        then_body: Box<Term>,
        else_body: Box<Term>,
    },
    Equals {
        left_side: Box<Term>,
        right_side: Box<Term>,
    },
    NotEquals {
        left_side: Box<Term>,
        right_side: Box<Term>,
    },

    Assignm {
        var_name: String,
        expr: Box<Term>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BinMathOp {
    Add,
    Minus,
    Multiply,
    Divide,
}
