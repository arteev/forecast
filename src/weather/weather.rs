use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::temperature::Temperature;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    #[serde(default)]
    pub is_cached: bool,

    #[serde(default = "default_created_at")]
    pub created_at: SystemTime,

    pub temp: Temperature,
    pub feels_like: Option<Temperature>,
    pub humidity: Option<u64>,
    pub icon: Option<String>,
    pub condition: Option<Condition>,
    pub forecasts: Option<Forecast>,
    pub daytime: Option<Daytime>,
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
    pub daytime: Option<Daytime>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl Condition {
    pub fn name(&self) -> String {
        match *self {
            Condition::Clear => "clear".to_string(),
            Condition::PartlyCloudy => "partly cloudy".to_string(),
            Condition::Cloudy => "cloudy".to_string(),
            Condition::Overcast => "overcast".to_string(),
            Condition::Drizzle => "drizzle".to_string(),
            Condition::LightRain => "light rain".to_string(),
            Condition::Rain => "rain".to_string(),
            Condition::ModerateRain => "moderate rain".to_string(),
            Condition::HeavyRain => "heavy rain".to_string(),
            Condition::ContinuousHeavyRain => "continuous heavy rain".to_string(),
            Condition::Showers => "showers".to_string(),
            Condition::WetSnow => "wet snow".to_string(),
            Condition::LightSnow => "light snow".to_string(),
            Condition::Snow => "snow".to_string(),
            Condition::SnowShowers => "snow showers".to_string(),
            Condition::Hail => "hail".to_string(),
            Condition::Thunderstorm => "thunderstorm".to_string(),
            Condition::ThunderstormWithRain => "thunderstorm with rain".to_string(),
            Condition::ThunderstormWithHail => "thunderstorm with hail".to_string(),
        }
    }

    pub fn icon(&self, is_night: Daytime) -> char {
        match self {
            Condition::Clear => if is_night == Daytime::Night { '' } else { '' },
            Condition::PartlyCloudy => if is_night == Daytime::Night { '' } else { '' },
            Condition::Cloudy => '',
            Condition::Overcast => '',
            Condition::Drizzle => ' ',
            Condition::LightRain => if is_night == Daytime::Night { '' } else { '' },
            Condition::Rain => if is_night == Daytime::Night { '' } else { '' },
            Condition::ModerateRain => if is_night == Daytime::Night { '' } else { '' },
            Condition::HeavyRain => if is_night == Daytime::Night { '' } else { '' },
            Condition::ContinuousHeavyRain => if is_night == Daytime::Night { '' } else { '' },
            Condition::Showers => if is_night == Daytime::Night { '' } else { '' },
            Condition::WetSnow => if is_night == Daytime::Night { '' } else { '' },
            Condition::LightSnow => if is_night == Daytime::Night { '' } else { '' },
            Condition::Snow => if is_night == Daytime::Night { '' } else { '' },
            Condition::SnowShowers => if is_night == Daytime::Night { '' } else { '' },
            Condition::Hail => ' ',
            Condition::Thunderstorm => if is_night == Daytime::Night { '' } else { '' },
            Condition::ThunderstormWithRain => if is_night == Daytime::Night { '' } else { '' },
            Condition::ThunderstormWithHail => if is_night == Daytime::Night { '' } else { '' },
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Daytime {
    Day,
    Night,
}
