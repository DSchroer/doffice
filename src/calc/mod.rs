mod operators;
mod engine;
mod reader;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::string::String;
use handlebars::{Handlebars, TemplateError};

use operators::{*};
use engine::CSVEngine;
use reader::CSVReader;
use crate::framework::{Command, Runner, RunnerConfig};

const WATCHER: &str = "new EventSource('/watch').addEventListener('reload', () => window.location.reload());";

pub fn evaluate_csv_file(source: Source, formatter: Formatter, config: RunnerConfig) -> Result<(), Box<dyn Error>> {
    let runner = Runner::new(Calc{ source, formatter });
    runner.exec(config)
}

pub enum Source {
    FromFile(String),
    FromString(String)
}

pub enum Formatter {
    Raw(),
    Html{
        watch: bool,
        theme_file: Option<String>
    }
}

struct Calc {
    source: Source,
    formatter: Formatter
}

impl Command for Calc {
    fn execute(&self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {

        let mut engine =  match &self.source {
            Source::FromFile(path) => CSVEngine::new(CSVReader::new(BufReader::new(File::open(path)?).bytes())),
            Source::FromString(data) => CSVEngine::new(CSVReader::new(BufReader::new(data.as_bytes()).bytes())),
        };

        engine.register_operator(Sum{});
        engine.register_operator(Count{});
        engine.register_operator(Average{});

        let mut buffer = Vec::new();
        for cell in engine {
            write!(buffer, "{}", cell)?;
        }

        match &self.formatter {
            Formatter::Raw() => {
                writer.write_all(&buffer)?;
            }
            Formatter::Html { watch, theme_file } => {
                let table = String::from_utf8(buffer)?;

                let mut data = HashMap::new();
                data.insert("table", table.as_str());

                if *watch {
                    data.insert("watcher", WATCHER);
                }

                let mut theme = String::new();
                if let Some(theme_file) = theme_file {
                    let theme_path = Path::new(&theme_file);
                    if let Ok(mut file) = File::open(theme_path) {
                        file.read_to_string(&mut theme)?;
                        data.insert("theme", theme.as_str());
                    }
                }

                let handlebars = create_handlebars()?;
                writer.write_all(handlebars.render("SHEET", &data)?.as_bytes())?;
            }
        }

        Ok(())
    }
}

fn create_handlebars<'a>() -> Result<Handlebars<'a>, TemplateError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("SHEET", include_str!("res/template.hbs"))?;
    Ok(handlebars)
}

#[cfg(test)]
mod tests {

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
