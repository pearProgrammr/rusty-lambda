use ast::*;
#[derive(PartialEq, Debug, Clone)]
/// This enum represents all possible values that a term can evaluate to.
pub enum Value {
    Num(u64),
    Bool(bool),
    Func { name: String, func_term: Box<Term> },
}
