mod config;
mod error;

use std::process;
use error::error::Error;
use config::config::Config;

fn forecast() -> Result<(), Error> {
    let c = Config::new()?;
    Ok(())
}

fn main() {
    if let Some(err) = forecast().err() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
