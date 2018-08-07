/// This enum represents all possible values that a term can evaluate to.
pub enum Value {
    Num (u64),
    Bool (bool),
    Func {t1: Box<Value>, t2: Box<Value>},
}
