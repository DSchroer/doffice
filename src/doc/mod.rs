use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use pulldown_cmark::{Parser, html};

pub fn process_markdown_file(file: String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file);

    let mut out_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.with_extension("out.html"))?;

    let mut file = File::open(path)?;

    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let html = render_markdown(&text);
    out_file.write_all(html.as_bytes())?;

    Ok(())
}

pub fn render_markdown(input: &str) -> String {
    let parser = Parser::new(input);

    let mut html_output = String::new();
    html::push_html(&mut html_output,  parser);

    return html_output;
}

#[cfg(test)]
mod tests {
    use crate::doc::render_markdown;

    #[test]
    fn emits_html() {
        assert_eq!("<h1>hi</h1>\n", &render_markdown("# hi"));
    }
}
