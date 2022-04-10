use std::collections::HashMap;
use std::fmt::format;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::temperature::{Temperature, Unit::*};

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


impl Serialize for WeatherInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut name_units = HashMap::new();
        name_units.insert(Celsius, "celsius");
        name_units.insert(Kelvin, "kelvin");
        name_units.insert(Fahrenheit, "fahrenheit");


        let mut s = serializer.serialize_struct("WeatherInfo", 30)?;
        for unit in vec![Celsius, Kelvin, Fahrenheit] {
            let t_c = self.temp.as_unit(unit);
            let name_field = format!("temperature_{}", name_units.get(&unit).unwrap());
            let name_field_full = format!("temperature_{}_full", name_units.get(&unit).unwrap());
            s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
            s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;

            if let Some(feel)=self.feels_like {
                let t_c = feel.as_unit(unit);
                let name_field = format!("feel_temperature_{}", name_units.get(&unit).unwrap());
                let name_field_full = format!("feel_temperature_{}_full", name_units.get(&unit).unwrap());
                s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;
            }
        }

        if let Some(forecasts) = &self.forecasts {
            for (i,part) in forecasts.parts.iter().enumerate() {
                for unit in vec![Celsius, Kelvin, Fahrenheit] {
                    let t_c = part.temp.as_unit(unit);
                    let name_field = format!("forecast_{}_temperature_{}",i, name_units.get(&unit).unwrap());
                    let name_field_full = format!("forecast_{}_temperature_{}_full",i, name_units.get(&unit).unwrap());
                    s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                    s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;

                    let name_field = format!("forecast_{}_temperature_{}",part.name, name_units.get(&unit).unwrap());
                    let name_field_full = format!("forecast_{}_temperature_{}_full",part.name, name_units.get(&unit).unwrap());
                    s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                    s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;

                    if let Some(feel)=self.feels_like {
                        let t_c = feel.as_unit(unit);
                        let name_field = format!("feel_forecast_{}_temperature_{}",i, name_units.get(&unit).unwrap());
                        let name_field_full = format!("feel_forecast_{}_temperature_{}_full",i, name_units.get(&unit).unwrap());
                        s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                        s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;

                        let name_field = format!("feel_forecast_{}_temperature_{}",part.name, name_units.get(&unit).unwrap());
                        let name_field_full = format!("feel_forecast_{}_temperature_{}_full",part.name, name_units.get(&unit).unwrap());
                        s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                        s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;
                    }
                }
            }
        }

        s.end()
    }
}


fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}