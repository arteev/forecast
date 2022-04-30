use std::{fs, vec};

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::Serialize;
use serde_json::Value;

use crate::config::yandex::ConfigYandex;
use crate::Error;
use crate::Error::InvalidRequest;
use crate::temperature::Temperature;
use crate::temperature::Unit::Celsius;
use crate::weather::provider::{WeatherGetter, WeatherQueryType};
use crate::weather::weather::{Condition, Forecast, ForecastPart, WeatherInfo};

const API_URL: &str = "https://api.weather.yandex.ru/v2/informers?";

pub struct Yandex {
    config: ConfigYandex,
}

impl Yandex {
    pub fn new(config: ConfigYandex) -> Self {
        Yandex { config }
    }
}

impl WeatherGetter for Yandex {
    fn get(&self, _: Vec<WeatherQueryType>) -> Result<WeatherInfo, Error> {
        let query_params = QueryParamsInformers {
            lon: self.config.lon.as_str(),
            lat: self.config.lat.as_str(),
        };

        let params = serde_qs::to_string(&query_params).
            expect("failed to format query params");

        let url = API_URL.to_owned() + &params;

        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert("X-Yandex-API-Key", self.config.api_key.parse().unwrap());
        headers.insert(USER_AGENT, self.config.user_agent.parse().unwrap());

        let response = client.get(url)
            .headers(headers)
            .send()?;
        if !response.status().is_success() {
            return Err(
                InvalidRequest {
                    code: response.status().as_u16(),
                    text: response.text()?.trim().to_string(),
                }
            );
        }
        let res: Value = response.json()?;

        parse(res).ok_or(Error::InvalidResponse)
    }
}

#[derive(Debug, Serialize)]
struct QueryParamsInformers<'a> {
    lat: &'a str,
    lon: &'a str,
}

fn parse(response: Value) -> Option<WeatherInfo> {
    let temperature = response["fact"]["temp"].as_i64()?;
    let temperature_like = response["fact"]["feels_like"].as_i64()?;


    Some(WeatherInfo {
        is_cached: false,
        temp: Temperature::new(temperature as i16, Celsius),
        feels_like: Some(Temperature::new(temperature_like as i16, Celsius)),
        humidity: response["fact"]["humidity"].as_u64(),
        icon: Some(response["fact"]["icon"].as_str()?.to_string()),
        condition: parse_condition(response["fact"]["condition"].as_str()),
        forecasts: parse_forecast(&response["forecast"]),
    })
}

fn parse_forecast(response: &Value) -> Option<Forecast> {
    let mut forecast = Forecast {
        parts: Vec::new(),
    };
    for part in response["parts"].as_array()? {
        let temperature = part["temp_avg"].as_i64()?;
        let temperature_like = part["feels_like"].as_i64()?;

        let forecast_part = ForecastPart {
            name: part["part_name"].as_str()?.to_string(),
            temp: Temperature::new(temperature as i16, Celsius),
            feels_like: Some(Temperature::new(temperature_like as i16, Celsius)),
            humidity: part["humidity"].as_u64(),
            condition: parse_condition(part["condition"].as_str()),
            icon: Some(part["icon"].as_str()?.to_string()),
        };

        forecast.parts.push(forecast_part);
    }
    Some(forecast)
}

fn parse_condition(s: Option<&str>) -> Option<Condition> {
    if let Some(condition) = s {
        return Some(match condition {
            "clear" => Condition::Clear,
            "partly-cloudy" => Condition::PartlyCloudy,
            "cloudy" => Condition::Cloudy,
            "overcast" => Condition::Overcast,
            "drizzle" => Condition::Drizzle,
            "light-rain" => Condition::LightRain,
            "rain" => Condition::Rain,
            "moderate-rain" => Condition::ModerateRain,
            "heavy-rain" => Condition::HeavyRain,
            "continuous-heavy-rain" => Condition::ContinuousHeavyRain,
            "showers" => Condition::Showers,
            "wet-snow" => Condition::WetSnow,
            "light-snow" => Condition::LightSnow,
            "snow" => Condition::Snow,
            "snow-showers" => Condition::SnowShowers,
            "hail" => Condition::Hail,
            "thunderstorm" => Condition::Thunderstorm,
            "thunderstorm-with-rain" => Condition::ThunderstormWithRain,
            "thunderstorm-with-hail" => Condition::ThunderstormWithHail,
            _ => return None,
        });
    }
    None
}