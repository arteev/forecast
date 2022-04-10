/*
Inspired by:
    https://github.com/kamek-pf/polybar-forecast
 */

mod config;
mod error;
mod temperature;
mod weather;
mod yandex;

use std::process;
use error::error::Error;
use config::config::Config;
use crate::weather::provider::{WeatherGetter, WeatherQueryType};
use crate::yandex::yandex::Yandex;

fn forecast() -> Result<(), Error> {
    let c = Config::new()?;
    let config = c.yandex.unwrap();
    let mut provider = Yandex::new(&config);
    let w = provider.get(vec![WeatherQueryType::All])?;
    println!("{:?}", w);
    Ok(())
}

fn main() {
    if let Some(err) = forecast().err() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
