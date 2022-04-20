use std::error::Error;
use std::ops::{Deref};
use pulldown_cmark::{CodeBlockKind, Event, LinkType, Tag};
use crate::doc::Document;
use crate::Printer;

macro_rules! unknown {
    ($e: ident) => {
        {
            println!("{:?}", $e);
            todo!();
        }
    };
}

pub struct MarkdownPrinter;

impl MarkdownPrinter {
    pub fn new() -> Self {
        MarkdownPrinter
    }
}

impl Printer<Document> for MarkdownPrinter {
    fn print(&self, document: Document) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut output = String::new();
        let mut indented = false;
        let mut list_position = None;

        for event in document.elements()? {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Paragraph => {},
                        Tag::Heading(level, ..) => output += &format!("{} ", &"#".repeat(level as usize)),
                        Tag::BlockQuote => output += "> ",
                        Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => output += &format!("```{}\n", lang),
                        Tag::CodeBlock(CodeBlockKind::Indented) => indented = true,
                        Tag::List(position) => list_position = position,
                        Tag::Item => match list_position {
                            None => output += "- ",
                            Some(i) => {
                                output += &format!("{}. ", i);
                                list_position = Some(i + 1);
                            }
                        },
                        Tag::Image(..) => output += "![",
                        Tag::Link(line_type, ..) => {
                            match line_type {
                                LinkType::Inline => output += "[",
                                LinkType::Reference => output += "[",
                                LinkType::ReferenceUnknown => output += "[",
                                LinkType::Collapsed => output += "[",
                                LinkType::CollapsedUnknown => output += "[",
                                LinkType::Shortcut => output += "[",
                                LinkType::ShortcutUnknown => output += "[",
                                LinkType::Autolink => output += "<",
                                LinkType::Email => output += "<",
                            }
                        },
                        Tag::Emphasis => output += "*",
                        Tag::Strong => output += "__",
                        Tag::Strikethrough => output += "~~",

                        t => unknown!(t)
                    };
                },
                Event::End(tag) => {
                    match tag {
                        Tag::Paragraph => output += "\n",
                        Tag::Heading(..) => output += "\n",
                        Tag::BlockQuote => output += "\n",
                        Tag::CodeBlock(CodeBlockKind::Fenced(_)) => output += "```\n",
                        Tag::CodeBlock(CodeBlockKind::Indented) => {
                            indented = false;
                            output += "";
                        },
                        Tag::List(_) => list_position = None,
                        Tag::Item => output += "\n",
                        Tag::Image(_, url, _) => output += &format!("]({})", url),
                        Tag::Link(line_type, url, ..) => {
                            match line_type {
                                LinkType::Inline => output += &format!("]({})", url),
                                LinkType::Reference => output += &format!("][{}]", url),
                                LinkType::ReferenceUnknown => output += &format!("][{}]", url),
                                LinkType::Collapsed => output += &format!("][{}]", url),
                                LinkType::CollapsedUnknown => output += &format!("][{}]", url),
                                LinkType::Shortcut => output += "]",
                                LinkType::ShortcutUnknown => output += "]",
                                LinkType::Autolink => output += ">",
                                LinkType::Email => output += ">",
                            }
                        },

                        Tag::Emphasis => output += "*",
                        Tag::Strong => output += "__",
                        Tag::Strikethrough => output += "~~",

                        t => unknown!(t)
                    };
                }
                Event::Text(text) => {
                    if indented {
                        output += &format!("    {}", text.deref())
                    }else {
                        output += text.deref()
                    }
                },
                Event::Code(text) => output += &format!("`{}`", text),
                Event::Rule => output += "---\n",
                Event::SoftBreak => output += "\n",
                Event::HardBreak => output += "  \n",
                Event::Html(tag) => output += &format!("{}\n", tag),

                Event::FootnoteReference(..) => todo!(),
                Event::TaskListMarker(..) => todo!()
            }
        }

        Ok(output.into_bytes())
    }

    fn extension() -> &'static str {
        "md"
    }
}

#[cfg(test)]
mod tests {
    use pulldown_cmark::Event;
    use crate::doc::Document;
    use crate::doc::markdown_printer::MarkdownPrinter;
    use crate::Printer;

    #[test]
    fn copies_input() {
        let mut cases = Vec::new();
        cases.push("");
        cases.push("hi");
        cases.push("# hi");
        cases.push("## hi");
        cases.push("### hi");
        cases.push("#### hi");
        cases.push("##### hi");
        cases.push("> hi");
        cases.push("`hi`");
        cases.push("```test\nhi\n```");
        cases.push("    foo\n    bar");
        cases.push("- item");
        cases.push("2. item");
        cases.push("*hi*");
        cases.push("__hi__");
        cases.push("~~hi~~");
        cases.push("![foo](bar)");
        cases.push("[foo](bar)");
        cases.push("[foo][bar]");
        cases.push("<https://foo.com>");
        cases.push("<test@boo.com>");
        cases.push("---");
        cases.push("hi  \nho");
        cases.push("hi\nho");
        cases.push("<div>\n\ntest\n\n</div>\n");

        for case in cases {
            let printer = MarkdownPrinter;

            let buffer = printer.print(Document::new(case)).unwrap();

            let original_doc = Document::new(case);
            let new_doc = Document{ source: String::from_utf8(buffer).unwrap(), path: None };

            let original_elements: Vec<Event> = original_doc.elements().unwrap().collect();
            let new_elements: Vec<Event> = new_doc.elements().unwrap().collect();

            assert_eq!(original_elements, new_elements)
        }
    }
}