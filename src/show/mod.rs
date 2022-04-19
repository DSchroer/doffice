use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;
use pulldown_cmark::Event;
use regex::{Regex};
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
    layout: Option<&'a str>,
    path: Option<&'a Path>,
    md: &'a str
}

impl<'a> Slide<'a> {
    pub fn layout(&self) -> Option<&str> {
        self.layout
    }

    pub fn elements(&self) -> Result<IntoIter<Event>, Box<dyn Error>> {
        render_markdown(self.md, self.path)
    }
}

impl Presentation {
    pub fn new(source: &str) -> Self {
        Presentation{ source: String::from(source), path: None }
    }

    pub fn slides(&self) -> Result<IntoIter<Slide>, Box<dyn Error>> {
        let pattern = Regex::new(r"<!--\s*slide(?:\[([-\w]+)?\])?\s*-->")?;
        let path: Option<&Path> = match &self.path {
            None => None,
            Some(p) => Some(p)
        };

        let mut slides = Vec::new();
        let mut last = 0;
        let mut layout = None;

        for (index, matched) in self.source.match_indices(&pattern) {
            let slide_text = &self.source[last..index];

            if last != index {
                slides.push(Slide{ md: slide_text, path, layout });
                last = index + matched.len();
            }

            layout = if let Some(c) = pattern.captures(matched) {
                match c.get(1) {
                    None => None,
                    Some(m) => Some(&self.source[index+m.range().start..index+m.range().end])
                }
            }else{
                None
            };
        }

        if last < self.source.len() {
            let slide_text = &self.source[last..];
            slides.push(Slide{ md: slide_text, path, layout });
        }

        Ok(slides.into_iter())
    }
}

impl<'a> Loader for Slides<'a> {
    type Result = Presentation;

    fn load(&self) -> Result<Presentation, Box<dyn Error>> {
        let mut file = File::open(self.file)?;

        let mut text = String::new();
        file.read_to_string(&mut text)?;

        let directory = self.file.parent().unwrap().to_path_buf();
        Ok(Presentation{ source: text, path: Some(directory) })
    }
}

#[cfg(test)]
mod tests {
    use crate::show::{Presentation};

    #[test]
    fn renders_single_slide() {
        let pres = Presentation::new("test");
        assert_eq!(1, pres.slides().unwrap().len())
    }

    #[test]
    fn splits_slides() {
        let pres = Presentation::new("a<!--slide-->b");
        assert_eq!(2, pres.slides().unwrap().len())
    }

    #[test]
    fn skips_empty_first_slide() {
        let pres = Presentation::new("<!--slide-->b");
        assert_eq!(1, pres.slides().unwrap().len())
    }

    #[test]
    fn loads_layout_from_slide() {
        let pres = Presentation::new("a<!--slide[test]-->b");
        assert_eq!(Some("test"), pres.slides().unwrap().nth(1).unwrap().layout)
    }

    #[test]
    fn ignores_empty_layout() {
        let pres = Presentation::new("<!--slide[]-->b");
        assert_eq!(None, pres.slides().unwrap().nth(0).unwrap().layout)
    }
}
