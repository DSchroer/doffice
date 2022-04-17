use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::doc::render_markdown;
use handlebars::{Handlebars, RenderError, TemplateError};
use crate::framework::{Command, Runner, RunnerConfig};

const SLIDE: &str = "<section>{{{ slide }}}</section>";
const WATCHER: &str = "new EventSource('/watch').addEventListener('reload', () => window.location.reload());";

pub enum Theme {
    White,
    Black
}

pub fn slides_from_file(file: String, base_theme: Theme, theme_file: Option<String>, config: RunnerConfig) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file);
    let watch = matches!(config, RunnerConfig::Watch(..));
    let runner = Runner::new(Slides{ file: path, base_theme, theme_file, watch });
    runner.exec(config)
}

struct Slides<'a>{
    file: &'a Path,
    base_theme: Theme,
    theme_file: Option<String>,
    watch: bool
}

impl<'a> Command for Slides<'a> {
    fn execute(&self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(self.file)?;

        let mut text = String::new();
        file.read_to_string(&mut text)?;

        let handlebars = create_handlebars()?;

        let slides = render_slides(&text, &handlebars)?;

        let mut data = HashMap::new();
        data.insert("slides", slides.as_str());
        data.insert("style", include_str!("res/reveal.out.css"));
        data.insert("reveal", include_str!("res/reveal.out.js"));

        if self.watch {
            data.insert("watcher", WATCHER);
        }

        match self.base_theme {
            Theme::White => { data.insert("base_theme", include_str!("res/white.out.css")); },
            Theme::Black => { data.insert("base_theme", include_str!("res/black.out.css")); },
        }

        let mut theme = String::new();
        if let Some(theme_file) = &self.theme_file {
            let theme_path = Path::new(&theme_file);
            if let Ok(mut file) = File::open(theme_path) {
                file.read_to_string(&mut theme)?;
                data.insert("theme", theme.as_str());
            }
        }

        writer.write_all(handlebars.render("PRESENTATION", &data)?.as_bytes())?;
        Ok(())
    }
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