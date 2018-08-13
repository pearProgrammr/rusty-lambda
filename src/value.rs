use ast::*;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
/// This enum represents all possible values that a term can evaluate to.
pub enum Value {
    Num(u64),
    Bool(bool),
    Closure { env: HashMap<String, Value>, name: String, func_term: Box<Term> },
    Assignm { name: String, val: Box<Value> },
}
