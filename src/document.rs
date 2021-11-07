use emojicons::EmojiFormatter;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Document<'a> {
    pub path: &'a Path,
    pub contents: String,
    pub rendered: Option<String>,
}

pub fn new(path: &Path) -> Document {
    let contents = fs::read_to_string(path).expect("File not found");
    Document {
        path,
        contents,
        rendered: None,
    }
}

pub fn render<'a>(document: &'a mut Document) -> &'a Document<'a> {
    let markdown_input = format!("{}", EmojiFormatter(&document.contents));

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    document.rendered = Some(html_output);
    document
}
