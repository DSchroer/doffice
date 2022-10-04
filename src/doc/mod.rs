mod markdown_printer;
mod markdown_render;

use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;
use pulldown_cmark::{Event};
use crate::framework::{Loader};

pub use markdown_render::render_markdown;
pub use markdown_printer::MarkdownPrinter;

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



#[cfg(test)]
mod tests {
    use pulldown_cmark::html;
    use crate::doc::{Document};

    impl Document {
        pub fn new(source: &str) -> Self {
            Document { source: String::from(source), path: None }
        }
    }

    #[test]
    fn emits_html() {
        assert_eq!("<h1>hi</h1>\n", &render("# hi"));
    }

    #[test]
    fn highlights_cs() {
        assert!(&render("```cs\nvar t = 100;\n```").contains("var</span>"));
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
