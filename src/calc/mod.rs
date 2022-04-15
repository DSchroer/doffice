mod operators;
mod engine;
mod reader;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::string::String;

use operators::{*};
use engine::CSVEngine;
use reader::CSVReader;

pub fn evaluate_csv_file(file: String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file);
    let reader = File::open(path)?;

    let mut out_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.with_extension("out.csv"))?;

    evaluate_csv(reader, &mut out_file)
}

fn evaluate_csv(reader: impl Read, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let csv = CSVReader::new(BufReader::new(reader).bytes());

    let mut engine = CSVEngine::new(csv);
    engine.register_operator(Sum{});
    engine.register_operator(Count{});
    engine.register_operator(Average{});

    for cell in engine {
        write!(writer, "{}", cell)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calc::evaluate_csv;

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
        evaluate_csv(input.as_bytes(), &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
}
