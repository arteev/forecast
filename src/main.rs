/*
Inspired by:
    https://github.com/kamek-pf/polybar-forecast
 */

mod config;
mod error;
mod temperature;
mod weather;
mod yandex;
mod template;

use std::process;
use error::error::Error;
use config::config::Config;
use crate::weather::provider::{WeatherGetter, WeatherQueryType};
use crate::yandex::yandex::Yandex;
use template::template::Template;

fn weather() -> Result<String, Error> {
    let c = Config::new()?;
    let config = c.yandex.unwrap();
    let mut provider = Yandex::new(&config);
    let w = provider.get(vec![WeatherQueryType::All])?;
    println!("{:?}", w);

    let display = c.display.to_owned().unwrap();
    let tmpl = Template::new(display.as_str());

    tmpl.render(&w)
}

fn main() {
    match weather() {
        Ok(render_weather) => println!("{}", render_weather),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
