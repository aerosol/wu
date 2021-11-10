extern crate include_dir;
use std::path::PathBuf;

use include_dir::{include_dir, Dir};

static PROJECT_DIR: Dir = include_dir!("./priv");

mod cli;
mod document;

fn main() {
    let text_file = PROJECT_DIR.get_file("foo.html").unwrap();
    println!("{}", text_file.contents_utf8().unwrap());

    println!("Hello, world!");
    let path = PathBuf::from("foo.md");
    let mut doc = document::new(&path);
    println!("{:?}", doc);

    doc.render();
    println!("{:?}", doc);

    cli::dispatch()
}
