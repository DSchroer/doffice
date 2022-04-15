mod cell;
mod eval;
mod expression_parser;
mod ast;

pub use cell::{*};
pub use eval::{*};
pub use ast::{*};

pub trait Engine {
    fn value_at(&self, cell: &CellRef) -> Value;
    fn eval(&self, expr: &Expression) -> Value;
    fn call(&self, name: &str, arguments: &Vec<Expression>) -> Value;
}

pub trait Operator<T: Engine> {
    fn name(&self) -> &'static str;
    fn execute(&self, arguments: &Vec<Expression>, engine: &T) -> Value;
}
