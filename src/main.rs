/*
Inspired by:
    https://github.com/kamek-pf/polybar-forecast
 */

use std::process;

use config::config::Config;
use error::error::Error;
use template::template::Template;

use crate::cache::unqlite::UnQLiteCache;
use crate::weather::provider::{WeatherGetter, WeatherQueryType};
use crate::yandex::yandex::Yandex;

mod config;
mod error;
mod temperature;
mod weather;
mod yandex;
mod template;
mod cache;

fn weather() -> Result<String, Error> {
    let c = Config::new()?;
    let config = c.yandex.unwrap().clone();
    let mut provider: Box<dyn WeatherGetter> = Box::new(Yandex::new(config));

    if let Some(cache) = c.cache {
        if cache.enabled {
            let user = std::env::var("USER").unwrap_or("user".to_string());
            let path = format!("/home/{}/cache.unqlite", user);
            provider = Box::new(UnQLiteCache::new(provider, &path,
                                                  cache.expiration.unwrap().into()));
        }
    }

    let w = provider.get(vec![WeatherQueryType::All])?;
    println!("{:?}", w);

    let display = c.display.to_owned();
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
