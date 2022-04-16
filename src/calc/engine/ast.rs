use std::fmt::{Display, Formatter};
use crate::calc::engine::CellRef;

#[derive(Debug)]
pub enum Expression {
    Call(String, Vec<Expression>),
    Reference(CellRef),
    Range(CellRef, CellRef),
    Literal(Value)
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Text(String),
    Error(&'static str)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => if *v { write!(f, "TRUE") } else { write!(f, "FALSE") },
            Value::Number(n) => write!(f, "{:.2}", n),
            Value::Text(t) => write!(f, "{}", t),
            Value::Error(e) => write!(f, "#{}", e.to_uppercase()),
        }
    }
}
