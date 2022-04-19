mod operators;
mod engine;
mod reader;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Read};
use std::slice::Iter;
use std::string::String;

use operators::{*};
use engine::CSVEngine;
use reader::CSVReader;
use crate::calc::engine::Cell;
use crate::framework::{Loader, Printer};

pub enum Source {
    FromFile(String),
    FromString(String)
}

pub struct Calc {
    source: Source,
}

impl Calc {
    pub fn from_file(path: String) -> Self {
        Calc{ source: Source::FromFile(path) }
    }

    pub fn from_string(source: String) -> Self {
        Calc{ source: Source::FromString(source) }
    }
}

pub struct Table {
    cells: Vec<Cell>
}

impl Table {
    pub fn new(cells: Vec<Cell>) -> Self {
        Table{ cells }
    }

    pub fn cells(&self) -> Iter<'_, Cell> {
        self.cells.iter()
    }
}

impl Loader for Calc {
    type Result = Table;

    fn load(&self) -> Result<Table, Box<dyn Error>> {
        let mut engine =  match &self.source {
            Source::FromFile(path) => CSVEngine::new(CSVReader::new(BufReader::new(File::open(path)?).bytes())),
            Source::FromString(data) => CSVEngine::new(CSVReader::new(BufReader::new(data.as_bytes()).bytes())),
        };

        engine.register_operator(Sum{});
        engine.register_operator(Count{});
        engine.register_operator(Average{});

        Ok(Table {
            cells: engine.collect()
        })
    }
}

pub struct CsvPrinter;

impl CsvPrinter {
    pub fn new() -> Self {
        CsvPrinter
    }
}

impl Printer<Table> for CsvPrinter {
    fn print(&self, table: Table) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        for cell in table.cells() {
            write!(&mut buffer, "{}", cell)?;
        }
        Ok(buffer)
    }

    fn extension() -> &'static str {
        "csv"
    }
}

#[cfg(test)]
mod tests {
    use crate::calc::{Calc, CsvPrinter};
    use crate::framework::{Loader, Printer};

    #[test]
    fn reference() {
        assert_eq!("0,0.00", eval("0,=A1"));
    }

    #[test]
    fn operators() {
        assert_eq!("1,2,3,6.00", eval("1,2,3,=SUM(A1:C1)"));
        assert_eq!("1,2,3,3.00", eval("1,2,3,=COUNT(A1:C1)"));
        assert_eq!("1,2,3,2.00", eval("1,2,3,=AVERAGE(A1:C1)"));
    }

    fn eval(input: &str) -> String {
        let table = Calc::from_string(input.to_string()).load().unwrap();
        let printer = CsvPrinter::new();
        String::from_utf8(printer.print(table).unwrap()).unwrap()
    }
}
