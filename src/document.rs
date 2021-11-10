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

impl<'a> Document<'a> {
    pub fn render(&mut self) {
        let markdown_input = format!("{}", EmojiFormatter(&self.contents));

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&markdown_input, options);

        // Write to String buffer.
        let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
        html::push_html(&mut html_output, parser);

        self.rendered = Some(html_output);
    }
}

pub fn new(path: &Path) -> Document {
    let contents = fs::read_to_string(path).expect("File not found");
    Document {
        path,
        contents,
        rendered: None,
    }
}
