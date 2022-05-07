use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub config_file: Option<String>,

    #[clap(short, long)]
    pub no_cache: bool,
}

pub fn parse() -> Args {
    Args::parse()
}