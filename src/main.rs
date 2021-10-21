#[macro_use]
extern crate include_dir;

use include_dir::Dir;
static PROJECT_DIR: Dir = include_dir!("./priv");

mod cli;

fn main() {
    let text_file = PROJECT_DIR.get_file("foo.html").unwrap();

    println!("{}", text_file.contents_utf8().unwrap());
    println!("Hello, world!");
    let args = cli::parse_opts();
    println!("{:?}", args);
    if (match args.op.as_str() {
        "new" => cli::new(args.layout_root),
        "gen" => Ok(()),
        &_ => Ok(()),
    })
    .is_ok()
    {
        std::process::exit(0);
    } else {
        std::process::exit(1)
    }
}
