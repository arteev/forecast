use std::collections::HashMap;

use handlebars::Handlebars;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::Error;
use crate::temperature::Temperature;
use crate::temperature::Unit::*;
use crate::weather::weather::WeatherInfo;

struct WeatherInfoTemplate {
    //"{{ temperature_celsius }}/{{ temperature_celsius_full }}/{{ feel_temperature_celsius_full }} h:{{ humidity }}% forecast:{{ forecast_0_humidity}}"
    temp: Temperature,
    feels_like: Option<Temperature>,
}


impl WeatherInfoTemplate {
    fn from(w: &WeatherInfo) -> Self {
        WeatherInfoTemplate {
            temp: w.temp,
            feels_like: w.feels_like,
        }
    }
}

impl Serialize for WeatherInfoTemplate {
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

             if let Some(feel) = self.feels_like {
                 let t_c = feel.as_unit(unit);
                 let name_field = format!("feel_temperature_{}", name_units.get(&unit).unwrap());
                 let name_field_full = format!("feel_temperature_{}_full", name_units.get(&unit).unwrap());
                 s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                 s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;
             }
        }
        /*
           if let Some(humidity) = self.humidity {
                   s.serialize_field("humidity", &humidity)?;
               }

               if let Some(forecasts) = &self.forecasts {
                   for (i, part) in forecasts.parts.iter().enumerate() {
                       for unit in vec![Celsius, Kelvin, Fahrenheit] {
                           let t_c = part.temp.as_unit(unit);
                           let name_field = format!("forecast_{}_temperature_{}", i, name_units.get(&unit).unwrap());
                           let name_field_full = format!("forecast_{}_temperature_{}_full", i, name_units.get(&unit).unwrap());
                           s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                           s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;

                           let name_field = format!("forecast_{}_temperature_{}", part.name, name_units.get(&unit).unwrap());
                           let name_field_full = format!("forecast_{}_temperature_{}_full", part.name, name_units.get(&unit).unwrap());
                           s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                           s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;

                           if let Some(feel) = part.feels_like {
                               let t_c = feel.as_unit(unit);
                               let name_field = format!("feel_forecast_{}_temperature_{}", i, name_units.get(&unit).unwrap());
                               let name_field_full = format!("feel_forecast_{}_temperature_{}_full", i, name_units.get(&unit).unwrap());
                               s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                               s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;

                               let name_field = format!("feel_forecast_{}_temperature_{}", part.name, name_units.get(&unit).unwrap());
                               let name_field_full = format!("feel_forecast_{}_temperature_{}_full", part.name, name_units.get(&unit).unwrap());
                               s.serialize_field(string_to_static_str(name_field), &t_c.val())?;
                               s.serialize_field(string_to_static_str(name_field_full), &format!("{}", t_c))?;
                           }

                           if let Some(humidity) = part.humidity {
                               let name_field = format!("forecast_{}_humidity", i);
                               s.serialize_field(string_to_static_str(name_field), &humidity)?;
                               let name_field = format!("forecast_{}_humidity", part.name);
                               s.serialize_field(string_to_static_str(name_field), &humidity)?;
                           }
                       }


                   }

               }
          */
        s.end()
    }
}


fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub struct Template<'a> {
    template: &'a str,
}

impl<'a> Template<'a> {
    pub fn new(template: &'a str) -> Self {
        Template {
            template,
        }
    }

    pub fn render(&self, w: &WeatherInfo) -> Result<String, Error> {
        let weather = WeatherInfoTemplate::from(w);

        let mut reg = Handlebars::new();
        //reg.set_strict_mode(true);

        let out = reg.render_template(self.template, &weather)?;
        Ok(out)
    }
}