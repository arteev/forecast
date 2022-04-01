use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use crate::config::yandex::ConfigYandex;
use crate::Error;
use crate::temperature::Temperature;
use crate::temperature::Unit::Celsius;
use crate::weather::provider::{WeatherGetter, WeatherQueryType};
use crate::weather::weather::WeatherInfo;
use serde_json::Value;
use serde::Serialize;
use crate::Error::{InvalidRequest};

const API_URL: &str = "https://api.weather.yandex.ru/v2/informers?";

pub struct Yandex<'a> {
    config: &'a ConfigYandex,
}

impl<'a> Yandex<'a> {
    pub fn new(config: &'a ConfigYandex) -> Yandex<'a> {
        Yandex { config }
    }
}

impl<'a> WeatherGetter for Yandex<'a> {
    fn get(&self, query: Vec<WeatherQueryType>) -> Result<WeatherInfo, Error> {
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
        headers.insert(USER_AGENT,self.config.user_agent.parse().unwrap());

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

        print!("{:?}", res);

        todo!("parse");


        Ok(WeatherInfo {
            temp: Temperature::new(18, Celsius),
            feels_like: None,
            forecasts: None,
        })
    }
}

#[derive(Debug, Serialize)]
struct QueryParamsInformers<'a> {
    lat: &'a str,
    lon: &'a str,
}