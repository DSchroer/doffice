use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use handlebars::{Handlebars, TemplateError};
use pulldown_cmark::html;
use crate::calc::Table;
use crate::doc::Document;
use crate::framework::Printer;
use crate::show::Presentation;

const SLIDE: &str = "<section>{{{ slide }}}</section>";
const WATCHER: &str = "new EventSource('/watch').addEventListener('reload', () => window.location.reload());";

pub struct HtmlPrinter {
    watch: bool,
    theme_file: Option<String>
}

impl HtmlPrinter {
    pub fn new(watch: bool, theme_file: Option<String>) -> Self {
        HtmlPrinter{ watch, theme_file }
    }
}

impl Printer<Table> for HtmlPrinter {
    fn print(&self, value: Table) -> Result<Vec<u8>, Box<dyn Error>> {

        let mut buffer = Vec::new();
        for cell in value.cells() {
            write!(buffer, "{}", cell)?;
        }

        let table = String::from_utf8(buffer)?;

        let mut data = HashMap::new();
        data.insert("table", table.as_str());

        if self.watch {
            data.insert("watcher", WATCHER);
        }

        let mut theme = String::new();
        if let Some(theme_file) = &self.theme_file {
            let theme_path = Path::new(theme_file);
            if let Ok(mut file) = File::open(theme_path) {
                file.read_to_string(&mut theme)?;
                data.insert("theme", theme.as_str());
            }
        }

        let handlebars = create_handlebars()?;
        Ok(handlebars.render("SHEET", &data)?.as_bytes().to_vec())
    }
}

impl Printer<Document> for HtmlPrinter {
    fn print(&self, document: Document) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut data = HashMap::new();

        let mut buffer = Vec::new();
        html::write_html(&mut buffer, document.elements()?)?;

        let document_str = String::from_utf8(buffer)?;
        data.insert("document", document_str.as_str());

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
        Ok(handlebars.render("DOCUMENT", &data)?.as_bytes().to_vec())
    }
}

impl Printer<Presentation> for HtmlPrinter {
    fn print(&self, presentation: Presentation) -> Result<Vec<u8>, Box<dyn Error>> {
        let handlebars = create_handlebars()?;

        let mut data = HashMap::new();

        let mut slides = String::new();
        for slide in presentation.slides() {
            let mut slide_data = HashMap::new();

            let mut buffer = Vec::new();
            html::write_html(&mut buffer, slide.elements()?)?;
            slide_data.insert("slide", String::from_utf8(buffer)?);
            let rendered_slide = handlebars.render("SLIDE", &slide_data)?;

            writeln!(slides, "{}", rendered_slide)?;
        }

        data.insert("slides", slides.as_str());
        data.insert("style", include_str!("res/reveal.out.css"));
        data.insert("reveal", include_str!("res/reveal.out.js"));
        data.insert("base_theme", include_str!("res/white.out.css"));

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

        Ok(handlebars.render("SLIDES", &data)?.as_bytes().to_vec())
    }
}

fn create_handlebars<'a>() -> Result<Handlebars<'a>, TemplateError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("SHEET", include_str!("res/table.hbs"))?;
    handlebars.register_template_string("DOCUMENT", include_str!("res/document.hbs"))?;
    handlebars.register_template_string("SLIDES", include_str!("res/slides.hbs"))?;
    handlebars.register_template_string("SLIDE", SLIDE)?;
    Ok(handlebars)
}

#[cfg(test)]
mod tests {
    use crate::calc::Table;
    use crate::doc::Document;
    use crate::show::Presentation;
    use crate::framework::Printer;
    use crate::{HtmlPrinter};

    #[test]
    fn can_render_table() {
        let render = HtmlPrinter::new(false, None);
        render.print(Table::new(Vec::new())).unwrap();
    }

    #[test]
    fn can_render_document() {
        let render = HtmlPrinter::new(false, None);
        render.print(Document::new("")).unwrap();
    }

    #[test]
    fn can_render_slides() {
        let render = HtmlPrinter::new(false, None);
        render.print(Presentation::new("")).unwrap();
    }
}
