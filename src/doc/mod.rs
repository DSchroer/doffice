use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path};
use pulldown_cmark::{Parser, html, Event, Tag, CodeBlockKind, CowStr};
use crate::calc::evaluate_csv_file;
use crate::framework::{Command, Runner, RunnerConfig};
use crate::Source;

pub fn process_markdown_file(file: String, config: RunnerConfig) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file);
    let runner = Runner::new(Doc(path));
    runner.exec(config)
}

struct Doc<'a>(&'a Path);

impl<'a> Command for Doc<'a> {
    fn execute(&self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(self.0)?;

        let mut text = String::new();
        file.read_to_string(&mut text)?;

        let html = render_markdown(&text);
        writer.write_all(html.as_bytes())?;
        Ok(())
    }
}

pub fn render_markdown(input: &str) -> String {
    let mut events: Vec<Event> = Parser::new(input).collect();

    for i in 0..events.len() {
        let csv = CowStr::from("csv");
        if matches!(&events[i], Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(x))) if x == &csv) {
            let text: String = match &events[i+1] {
                Event::Text(text) => text.clone().into_string(),
                _ => panic!("mal formatted code block")
            };

            let mut computed = Vec::new();
            evaluate_csv_file(Source::FromString(text), RunnerConfig::ToData(&mut computed)).unwrap();
            events[i+1] = Event::Text(CowStr::from(String::from_utf8(computed).unwrap()));
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output,  events.into_iter());

    return html_output;
}

#[cfg(test)]
mod tests {
    use crate::doc::render_markdown;

    #[test]
    fn emits_html() {
        assert_eq!("<h1>hi</h1>\n", &render_markdown("# hi"));
    }

    #[test]
    fn replaces_csv() {
        assert_eq!("<pre><code class=\"language-csv\">NUM, NUM\n10,10\n</code></pre>\n", &render_markdown(r"
```csv
NUM, NUM
10,=A2
```
        ".trim()));
    }
}
