mod operators;
mod engine;
mod reader;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::string::String;

use operators::{*};
use engine::CSVEngine;
use reader::CSVReader;
use crate::framework::{Command, Runner, RunnerConfig};

pub fn evaluate_csv_file(source: Source, config: RunnerConfig) -> Result<(), Box<dyn Error>> {
    let runner = Runner::new(Calc(source));
    runner.exec(config)
}

pub enum Source {
    FromFile(String),
    FromString(String)
}

struct Calc(Source);

impl Command for Calc {
    fn execute(&self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {

        let mut engine =  match &self.0 {
            Source::FromFile(path) => CSVEngine::new(CSVReader::new(BufReader::new(File::open(path)?).bytes())),
            Source::FromString(data) => CSVEngine::new(CSVReader::new(BufReader::new(data.as_bytes()).bytes())),
        };

        engine.register_operator(Sum{});
        engine.register_operator(Count{});
        engine.register_operator(Average{});

        for cell in engine {
            write!(writer, "{}", cell)?;
        }

        Ok(())
    }
}

// pub fn evaluate_csv_str(input: &str) -> Result<String, Box<dyn Error>> {
//     let mut buf = Vec::new();
//     evaluate_csv(input.as_bytes(), &mut buf)?;
//     Ok(String::from_utf8(buf)?)
// }

#[cfg(test)]
mod tests {
    // use crate::calc::evaluate_csv;

    #[test]
    fn reference() {
        assert_eq!("0,0", eval("0,=A1"));
    }

    #[test]
    fn operators() {
        assert_eq!("1,2,3,6", eval("1,2,3,=SUM(A1:C1)"));
        assert_eq!("1,2,3,3", eval("1,2,3,=COUNT(A1:C1)"));
        assert_eq!("1,2,3,2", eval("1,2,3,=AVERAGE(A1:C1)"));
    }

    fn eval(input: &str) -> String {
        let mut buf = Vec::new();
        // evaluate_csv(input.as_bytes(), &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
}
