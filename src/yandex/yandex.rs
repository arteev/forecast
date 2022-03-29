use crate::config::yandex::ConfigYandex;
use crate::Error;
use crate::temperature::Temperature;
use crate::temperature::Unit::Celsius;
use crate::weather::provider::{WeatherGetter, WeatherQueryType};
use crate::weather::weather::WeatherInfo;

pub struct Yandex<'a> {
    config: &'a ConfigYandex,
}

// impl<'a> OpenWeatherMap<'a> {
// pub fn new(config: &'a Configuration) -> OpenWeatherMap<'a> {
//     OpenWeatherMap { config }
// }

impl<'a> Yandex<'a> {
    pub fn new(config: &'a ConfigYandex) -> Yandex<'a> {
        Yandex { config }
    }
}

impl<'a> WeatherGetter for Yandex<'a> {
    fn get(&self, query: Vec<WeatherQueryType>) -> Result<WeatherInfo, Error> {
        Ok(WeatherInfo {
            temp: Temperature::new(18, Celsius),
            feels_like: None,
            forecasts: None,
        })
    }
}