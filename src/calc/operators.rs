use crate::calc::engine::{CellRef, Engine, Expression, Operator, Value};

pub struct Sum;
impl<T: Engine> Operator<T> for Sum {
    fn name(&self) -> &'static str {
        "SUM"
    }

    fn execute(&self, arguments: &Vec<Expression>, engine: &T) -> Value {
        let mut sum: f64 = 0.0;
        for arg in arguments {
            if let Expression::Range(a, b) = arg {
                for cell in CellRef::range(&a, &b) {
                    if let Value::Number(n) = engine.value_at(&cell) {
                        sum += n;
                    } else {
                        return Value::Error("ARG?")
                    }
                }
            } else if let Value::Number(n) = engine.eval(arg) {
                sum += n;
            } else {
                return Value::Error("ARG?")
            }
        }
        Value::Number(sum)
    }
}

pub struct Count;
impl<T: Engine> Operator<T> for Count {
    fn name(&self) -> &'static str {
        "COUNT"
    }

    fn execute(&self, arguments: &Vec<Expression>, engine: &T) -> Value {
        let mut count: u64 = 0;
        for arg in arguments {
            if let Expression::Range(a, b) = arg {
                for cell in CellRef::range(a, b) {
                    if let Value::Number(_) = engine.value_at(&cell) {
                        count += 1;
                    }
                }
            } else if let Value::Number(_) = engine.eval(arg) {
                count += 1;
            }
        }
        Value::Number(count as f64)
    }
}

pub struct Average;
impl<T: Engine> Operator<T> for Average {
    fn name(&self) -> &'static str {
        "AVERAGE"
    }

    fn execute(&self, arguments: &Vec<Expression>, engine: &T) -> Value {
        let sum = engine.call("SUM", arguments);
        let count = engine.call("COUNT", arguments);

       if let Value::Number(s) = sum {
            if let Value::Number(c) = count {
                return Value::Number(s / c)
            }
        }
        Value::Error("ARG?")
    }
}
