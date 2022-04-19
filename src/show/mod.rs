use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;
use pulldown_cmark::Event;
use crate::doc::{render_markdown};
use crate::framework::{Loader};

pub struct Slides<'a>{
    file: &'a Path,
}

impl<'a> Slides<'a> {
    pub fn new(file: &'a Path) -> Self {
        Slides { file }
    }
}

pub struct Presentation {
    path: Option<PathBuf>,
    source: String,
}

pub struct Slide<'a> {
    path: Option<&'a Path>,
    md: &'a str
}

impl<'a> Slide<'a> {
    pub fn elements(&self) -> Result<IntoIter<Event>, Box<dyn Error>> {
        render_markdown(self.md, self.path)
    }
}

impl Presentation {
    pub fn new(source: &str) -> Self {
        Presentation{ source: String::from(source), path: None }
    }

    pub fn slides(&self) -> IntoIter<Slide> {
        let parts = self.source.split("<!-- slide -->");
        let slides: Vec<Slide> = parts.map(|s|Slide{md: s, path: match &self.path {
            None => None,
            Some(p) => Some(p)
        }}).collect();
        slides.into_iter()
    }
}

impl<'a> Loader for Slides<'a> {
    type Result = Presentation;

    fn load(&self) -> Result<Presentation, Box<dyn Error>> {
        let mut file = File::open(self.file)?;

        let mut text = String::new();
        file.read_to_string(&mut text)?;

        Ok(Presentation{ source: text, path: Some(self.file.to_path_buf()) })
    }
}
