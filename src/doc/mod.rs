use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;
use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind, CowStr};
use crate::framework::{Loader, print_to_vec};
use crate::calc::{Calc, CsvPrinter};

pub struct Doc<'a>{
    file: &'a Path,
}

impl<'a> Doc<'a> {
    pub fn new(file: &'a Path) -> Self {
        Doc{ file }
    }
}

pub struct Document {
    path: Option<PathBuf>,
    source: String,
}

impl Document {
    pub fn new(source: &str) -> Self {
        Document { source: String::from(source), path: None }
    }

    pub fn elements(&self) -> Result<IntoIter<Event>, Box<dyn Error>> {
        render_markdown(&self.source, match &self.path {
            Some(p) => Some(&p),
            None => None
        })
    }
}

impl<'a> Loader for Doc<'a> {
    type Result = Document;

    fn load(&self) -> Result<Document, Box<dyn Error>> {
        let mut file = File::open(self.file)?;

        let mut text = String::new();
        file.read_to_string(&mut text)?;

        let directory = self.file.parent().unwrap().to_path_buf();
        Ok(Document{ source: text, path: Some(directory) })
    }
}

pub fn render_markdown<'a>(input: &'a str, root: Option<&Path>) -> Result<IntoIter<Event<'a>>, Box<dyn Error>> {
    let mut events: Vec<Event> = Parser::new(input).collect();

    for i in 0..events.len() {
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

        if let Some(root) = root {
            match &events[i] {
                Event::Start(Tag::Image(t, url, x)) => {
                    let mut buf = Vec::new();
                    let image_path = root.join(Path::new(&url.clone().into_string()));
                    let ext = image_path.clone();
                    File::open(image_path)?.read_to_end(&mut buf)?;

                    let data = base64::encode(buf);

                    let image = format!("data:image/{};base64,{}", ext.extension().unwrap().to_str().unwrap(), data);
                    events[i] = Event::Start(Tag::Image(t.clone(), CowStr::from(image), x.clone()))
                },
                _ => {}
            }
        }
    }

    Ok(events.into_iter())
}

#[cfg(test)]
mod tests {
    use pulldown_cmark::html;
    use crate::doc::{Document};

    #[test]
    fn emits_html() {
        assert_eq!("<h1>hi</h1>\n", &render("# hi"));
    }

    #[test]
    fn supports_utf8() {
        assert_eq!("<p>😊</p>\n", &render("😊"));
    }

    #[test]
    fn replaces_csv() {
        assert_eq!("<pre><code class=\"language-csv\">NUM, NUM\n10,10.00\n</code></pre>\n", &render(r"
```csv
NUM, NUM
10,=A2
```
        ".trim()));
    }

    fn render(input: &str) -> String {
        let doc = Document::new(input);
        let mut buf = Vec::new();
        html::write_html(&mut buf, doc.elements().unwrap()).unwrap();
        String::from_utf8(buf).unwrap()
    }
}
