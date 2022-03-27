/*
Inspired by:
    https://github.com/kamek-pf/polybar-forecast
 */

mod config;
mod error;
mod temperature;

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

    let t = temperature::Temperature::new(15, temperature::Unit::Celsius);
    print!("{:?}\n", t);
}
