use crate::Error;
use crate::weather::weather::WeatherInfo;

pub enum WeatherQueryType {
    All,
    Current,
    Forecast,
}

pub trait WeatherGetter {
    fn get(&self, query: Vec<WeatherQueryType>) -> Result<WeatherInfo, Error>;
}
