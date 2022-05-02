use std::collections::HashMap;
use std::fmt::format;
use std::time::SystemTime;

use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::temperature::{Temperature, Unit::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    #[serde(default)]
    pub is_cached: bool,

    #[serde(default = "default_created_at" )]
    pub created_at:  SystemTime,

    pub temp: Temperature,
    pub feels_like: Option<Temperature>,
    pub humidity: Option<u64>,
    pub icon: Option<String>,
    pub condition: Option<Condition>,
    pub forecasts: Option<Forecast>,
}

fn default_created_at() -> SystemTime {
    SystemTime::now()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub parts: Vec<ForecastPart>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastPart {
    pub name: String,
    pub temp: Temperature,
    pub humidity: Option<u64>,
    pub icon: Option<String>,
    pub condition: Option<Condition>,
    pub feels_like: Option<Temperature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Condition {
    // ясно.
    Clear,

    // малооблачно.
    PartlyCloudy,

    // облачно с прояснениями.
    Cloudy,

    // пасмурно.
    Overcast,

    // морось.
    Drizzle,

    // небольшой дождь.
    LightRain,

    // дождь.
    Rain,

    // умеренно сильный дождь.
    ModerateRain,

    // сильный дождь.
    HeavyRain,

    // длительный сильный дождь.
    ContinuousHeavyRain,

    // ливень.
    Showers,

    // дождь со снегом.
    WetSnow,

    // небольшой снег.
    LightSnow,

    // снег.
    Snow,

    // снегопад.
    SnowShowers,

    //  град.
    Hail,

    // гроза.
    Thunderstorm,

    //  дождь с грозой.
    ThunderstormWithRain,

    // гроза с градом.
    ThunderstormWithHail,
}