use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path};
use handlebars::{Handlebars, TemplateError};
use pulldown_cmark::{Parser, html, Event, Tag, CodeBlockKind, CowStr};
use crate::calc::evaluate_csv_file;
use crate::framework::{Command, Runner, RunnerConfig};
use crate::{Formatter, Source};

const WATCHER: &str = "new EventSource('/watch').addEventListener('reload', () => window.location.reload());";

pub fn process_markdown_file(file: String, theme_file: Option<String>, config: RunnerConfig) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file);
    let watch = matches!(config, RunnerConfig::Watch(..));
    let runner = Runner::new(Doc{ file: path, watch, theme_file });
    runner.exec(config)
}

struct Doc<'a>{
    file: &'a Path,
    theme_file: Option<String>,
    watch: bool
}

impl<'a> Command for Doc<'a> {
    fn execute(&self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(self.file)?;

        let mut text = String::new();
        file.read_to_string(&mut text)?;

        let html = render_markdown(&text);

        let mut data = HashMap::new();
        data.insert("document", html.as_str());

        if self.watch {
            data.insert("watcher", WATCHER);
        }

        let mut theme = String::new();
        if let Some(theme_file) = &self.theme_file {
            let theme_path = Path::new(&theme_file);
            if let Ok(mut file) = File::open(theme_path) {
                file.read_to_string(&mut theme)?;
                data.insert("theme", theme.as_str());
            }
        }

        let handlebars = create_handlebars()?;
        writer.write_all(handlebars.render("DOCUMENT", &data)?.as_bytes())?;
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
            evaluate_csv_file(Source::FromString(text), Formatter::Raw(), RunnerConfig::ToData(&mut computed)).unwrap();
            events[i+1] = Event::Text(CowStr::from(String::from_utf8(computed).unwrap()));
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output,  events.into_iter());

    return html_output;
}

fn create_handlebars<'a>() -> Result<Handlebars<'a>, TemplateError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("DOCUMENT", include_str!("res/template.hbs"))?;
    Ok(handlebars)
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
        assert_eq!("<pre><code class=\"language-csv\">NUM, NUM\n10,10.00\n</code></pre>\n", &render_markdown(r"
```csv
NUM, NUM
10,=A2
```
        ".trim()));
    }
}
