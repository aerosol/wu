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

pub fn new(layout_root: std::path::PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(layout_root)?;
    Ok(())
}

pub fn parse_opts() -> Cli {
    Cli::from_args()
}
