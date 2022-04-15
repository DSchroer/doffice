use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use crate::calc::engine::{Cell, CellRef, Engine, Operator};
use crate::calc::engine::{Expression, Value};
use crate::calc::engine::expression_parser::parse;

pub struct CSVEngine {
    heap: BinaryHeap<Reverse<CellRef>>,
    cells: HashMap<CellRef, Cell>,
    operators: HashMap<&'static str, Box<dyn Operator<CSVEngine>>>
}

impl CSVEngine {
    pub fn new<T: Iterator<Item=Cell>>(reader: T) -> Self {
        let mut engine = CSVEngine {
            cells: HashMap::new(),
            heap: BinaryHeap::new(),
            operators: HashMap::new()
        };
        for cell in reader {
            engine.add_cell(cell);
        }
        engine
    }

    pub fn register_operator(&mut self, operation: impl Operator<CSVEngine> + 'static){
        self.operators.insert(operation.name(), Box::new(operation));
    }

    fn compute(&self, mut cell: Cell) -> Cell {
        if cell.content.starts_with("=") {
            let parsed = parse(&cell.content[1..]);
            cell.content = format!("{}", self.eval(&parsed));
        }
        cell
    }

    fn value_of(&self, cell: &Cell) -> Value {
        if cell.content.starts_with("=") {
            let parsed = parse(&cell.content[1..]);
            return self.eval(&parsed);
        }
        if let Ok(float) = cell.content.trim().parse::<f32>() {
            return Value::Number(float as f64);
        }
        if cell.content.to_uppercase() == "TRUE" {
            return Value::Bool(true);
        }
        if cell.content.to_uppercase() == "FALSE" {
            return Value::Bool(false);
        }
        Value::Text(String::from(&cell.content))
    }

    fn add_cell(&mut self, cell: Cell) {
        self.heap.push(Reverse(cell.position));
        self.cells.insert(cell.position, cell);
    }
}

impl Engine for CSVEngine {
    fn value_at(&self, position: &CellRef) -> Value {
        if let Some(cell) = self.cells.get(position) {
            self.value_of(cell)
        } else {
            Value::Number(0.0)
        }
    }

    fn eval(&self, expr: &Expression) -> Value {
        match expr {
            Expression::Call(name, args) => self.call(name, args),
            Expression::Reference(r) => self.value_at(&r),
            Expression::Range(_, _) => Value::Error("REF!"),
            Expression::Literal(v) => v.clone(),
        }
    }

    fn call(&self, name: &str, arguments: &Vec<Expression>) -> Value {
        if let Some(operation) = self.operators.get(name) {
            operation.execute(arguments, self)
        } else {
            Value::Error("NAME?")
        }
    }
}

impl Iterator for CSVEngine {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.heap.pop() {
            None => None,
            Some(Reverse(cell)) => {
                let cell = self.cells.get(&cell).unwrap();
                let result = self.compute(cell.clone());
                Some(result)
            }
        }
    }
}
