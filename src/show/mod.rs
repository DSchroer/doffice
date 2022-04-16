use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use crate::doc::render_markdown;
use handlebars::{Handlebars, RenderError, TemplateError};

const SLIDE: &str = "<section>{{{ slide }}}</section>";

pub enum Theme {
    White,
    Black
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::White => f.write_str("white"),
            Theme::Black => f.write_str("black"),
        }
    }
}

impl FromStr for Theme {
    type Err = ThemeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "white" => Ok(Theme::White),
            "black" => Ok(Theme::Black),
            _ => Err(ThemeErr{})
        }
    }
}

#[derive(Debug)]
pub struct ThemeErr;
impl Display for ThemeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("failed to parse theme")
    }
}
impl Error for ThemeErr {}

pub fn slides_from_file(file: String, theme: Theme) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file);

    let mut out_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.with_extension("out.html"))?;

    let mut file = File::open(path)?;

    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let handlebars = create_handlebars()?;

    let slides = render_slides(&text, &handlebars)?;

    let mut data = HashMap::new();
    data.insert("slides", slides.as_str());
    data.insert("style", include_str!("res/reveal.out.css"));
    data.insert("reveal", include_str!("res/reveal.out.js"));

    match theme {
        Theme::White => { data.insert("theme", include_str!("res/white.out.css")); },
        Theme::Black => { data.insert("theme", include_str!("res/black.out.css")); },
    }

    out_file.write_all(handlebars.render("PRESENTATION", &data)?.as_bytes())?;
    Ok(())
}

fn render_slides(text: &str, handlebars: &Handlebars) -> Result<String, RenderError> {
    let mut rendered_slides = String::new();
    for slide in text.split("<!-- slide -->") {
        let mut data = HashMap::new();

        data.insert("slide", render_markdown(slide));

        rendered_slides += &handlebars.render("SLIDE", &data)?;
    }
    Ok(rendered_slides)
}

fn create_handlebars<'a>() -> Result<Handlebars<'a>, TemplateError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("SLIDE", SLIDE)?;
    handlebars.register_template_string("PRESENTATION", include_str!("res/template.hbs"))?;
    Ok(handlebars)
}