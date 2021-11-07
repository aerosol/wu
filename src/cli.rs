use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
pub struct Cli {
    pub op: String,
    #[structopt(parse(from_os_str))]
    pub layout_root: PathBuf,
    #[structopt(parse(from_os_str))]
    pub build_root: Option<PathBuf>,
    pub domain: Option<String>,
}

fn new(layout_root: std::path::PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(layout_root)?;
    Ok(())
}

fn parse_opts() -> Cli {
    Cli::from_args()
}

pub fn dispatch() {
    let args = parse_opts();
    let result = match args.op.as_str() {
        "new" => new(args.layout_root),
        "gen" => Ok(()),
        &_ => Err(Error::new(ErrorKind::Other, "Fail")),
    };

    if let Err(e) = result {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
