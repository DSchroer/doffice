use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::path::{Path};
use std::vec::IntoIter;
use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind, CowStr};
use syntect::parsing::SyntaxSet;
use syntect::html::{ClassedHTMLGenerator, ClassStyle};
use syntect::util::LinesWithEndings;
use crate::framework::{print_to_vec};
use crate::calc::{Calc, CsvPrinter};

pub trait MdExtension {
    fn extend(&self, index: usize, events: &mut [Event]) -> Result<(), Box<dyn Error>>;
}

struct MarkdownEngine<'a> {
    events: Vec<Event<'a>>
}

impl<'a> MarkdownEngine<'a> {
    pub fn new(input: &'a str) -> Self {
        let events: Vec<Event> = Parser::new(input).collect();
        MarkdownEngine { events }
    }

    pub fn apply(&mut self, ext: impl MdExtension) -> Result<&mut MarkdownEngine<'a>, Box<dyn Error>> {
        for i in 0..self.events.len() {
            ext.extend(i, &mut self.events)?
        }
        Ok(self)
    }

    pub fn render(self) -> IntoIter<Event<'a>> {
        self.events.into_iter()
    }
}

pub fn render_markdown<'a>(input: &'a str, root: Option<&Path>) -> Result<IntoIter<Event<'a>>, Box<dyn Error>> {
    let mut r = MarkdownEngine::new(input);
    r.apply(SyntaxHighlight)?;
    r.apply(CSVCalc)?;
    if let Some(root) = root {
        r.apply(EmbedImages(root))?;
    }
    Ok(r.render())
}

struct SyntaxHighlight;
impl MdExtension for SyntaxHighlight {
    fn extend(&self, i: usize, events: &mut [Event]) -> Result<(), Box<dyn Error>> {
        if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(x))) = &events[i] {
            let lang = x.clone().into_string();
            let ps = SyntaxSet::load_defaults_newlines();

            let text: String = match &events[i+1] {
                Event::Text(text) => text.clone().into_string(),
                _ => panic!("mal formatted code block")
            };

            if let Some(syntax) = ps.find_syntax_by_extension(&lang) {
                events[i] = Event::Start(Tag::Paragraph);

                let mut rs_html_generator = ClassedHTMLGenerator::new_with_class_style(&syntax, &ps, ClassStyle::Spaced);
                for line in LinesWithEndings::from(&text) {
                    rs_html_generator.parse_html_for_line_which_includes_newline(line)?;
                }
                let highlighted = rs_html_generator.finalize();

                events[i+1] = Event::Html(CowStr::from(format!("<pre class=\"code\">{}</pre>", highlighted)));
                events[i+2] = Event::End(Tag::Paragraph);
            }
        }

        Ok(())
    }
}

struct CSVCalc;
impl MdExtension for CSVCalc {
    fn extend(&self, i: usize, events: &mut [Event]) -> Result<(), Box<dyn Error>> {
        let csv = CowStr::from("csv");
        if matches!(&events[i], Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(x))) if x == &csv) {
            let text: String = match &events[i+1] {
                Event::Text(text) => text.clone().into_string(),
                _ => panic!("mal formatted code block")
            };

            let csv = Calc::from_string(text);
            let printer = CsvPrinter::new();
            let computed = print_to_vec(csv, printer)?;

            events[i+1] = Event::Text(CowStr::from(String::from_utf8(computed)?));
        }
        Ok(())
    }
}

struct EmbedImages<'a>(&'a Path);
impl<'a> MdExtension for EmbedImages<'a> {
    fn extend(&self, i: usize, events: &mut [Event]) -> Result<(), Box<dyn Error>> {
        let root = self.0;
        match &events[i] {
            Event::Start(Tag::Image(t, url, x)) => {
                let mut buf = Vec::new();
                let image_path = root.join(Path::new(&url.clone().into_string()));
                let ext = image_path.clone();
                File::open(image_path)?.read_to_end(&mut buf)?;

                let extension = ext.extension().unwrap().to_str().unwrap();

                if extension == "svg" {
                    events[i] = Event::Html(CowStr::from(String::from_utf8(buf).unwrap()))
                }else{
                    let data = base64::encode(buf);
                    let image = format!("data:image/{};base64,{}", extension, data);
                    events[i] = Event::Start(Tag::Image(t.clone(), CowStr::from(image), x.clone()))
                }
            },
            _ => {}
        }
        Ok(())
    }
}