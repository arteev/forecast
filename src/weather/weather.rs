use crate::temperature::Temperature;

#[derive(Debug)]
pub struct WeatherInfo {
    pub temp: Temperature,
    pub feels_like: Option<Temperature>,
    pub humidity: Option<u64>,
    pub icon: Option<String>,
    pub condition: Option<Condition>,
    pub forecasts: Option<Forecast>,
}

#[derive(Debug)]
pub struct Forecast {
    pub parts: Vec<ForecastPart>,
}

#[derive(Debug)]
pub struct ForecastPart {
    pub name: String,
    pub temp: Temperature,
    pub humidity: Option<u64>,
    pub icon: Option<String>,
    pub condition: Option<Condition>,
    pub feels_like: Option<Temperature>,
}

#[derive(Debug)]
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