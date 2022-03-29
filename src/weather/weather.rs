use crate::temperature::Temperature;

#[derive(Debug)]
pub struct WeatherInfo {
    pub temp: Temperature,
    pub feels_like: Option<Temperature>,
    pub forecasts: Option<Forecast>,
}

#[derive(Debug)]
pub struct Forecast {
    pub parts:  Option<Vec<ForecastPart>>
}

#[derive(Debug)]
pub struct ForecastPart {
    pub temp:Temperature,
    pub feels_like: Option<Temperature>,
}