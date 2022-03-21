use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub config_file: Option<String>,
}

pub fn parse() -> Args {
    Args::parse()
}