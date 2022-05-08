use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub config_file: Option<String>,

    #[clap(short, long)]
    pub no_cache: bool,

    #[clap(short, long)]
    pub prefer_cache_error: bool,
}

pub fn parse() -> Args {
    Args::parse()
}