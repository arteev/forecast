extern crate chrono;

use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::Error;
use crate::temperature::Temperature;
use crate::temperature::Unit::*;
use crate::weather::weather::WeatherInfo;

const TEMPLATE_DEBUG: &str = r#"
Weather template variables:

    cache: {{ cache }}

    created format="%H:%M": {{ created format="%H:%M "}}
    created: {{ created }}

    temperature_celsius: {{ temperature_celsius }}
    temperature_celsius_full: {{ temperature_celsius_full }}
    temperature_kelvin: {{ temperature_kelvin }}
    temperature_kelvin_full: {{ temperature_kelvin_full }}
    temperature_fahrenheit: {{ temperature_fahrenheit }}
    temperature_fahrenheit_full: {{ temperature_fahrenheit_full }}

    humidity: {{ humidity }}

    feel_temperature_celsius: {{ feel_temperature_celsius }}
    feel_temperature_celsius_full: {{ feel_temperature_celsius_full }}
    feel_temperature_kelvin: {{ feel_temperature_kelvin }}
    feel_temperature_kelvin_full: {{ feel_temperature_kelvin_full }}
    feel_temperature_fahrenheit: {{ feel_temperature_fahrenheit }}
    feel_temperature_fahrenheit_full: {{ feel_temperature_fahrenheit_full }}

     "#;

struct WeatherInfoTemplate {
    is_cached: bool,
    created_at: SystemTime,
    temp: Temperature,
    feels_like: Option<Temperature>,
    pub humidity: Option<u64>,
}


impl WeatherInfoTemplate {
    fn from(w: &WeatherInfo) -> Self {
        WeatherInfoTemplate {
            is_cached: w.is_cached,
            created_at: w.created_at,
            temp: w.temp,
            feels_like: w.feels_like,
            humidity: w.humidity,

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

        s.serialize_field("cache", &format!("{}", self.is_cached))?;

        s.serialize_field("date", &self.created_at)?;

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
        if let Some(humidity) = self.humidity {
            s.serialize_field("humidity", &humidity)?;
        }
        /*


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

    pub fn render(&self, w: &WeatherInfo, debug: bool) -> Result<String, Error> {
        let weather = WeatherInfoTemplate::from(w);

        let mut reg = Handlebars::new();
        //reg.set_strict_mode(true);
        reg.register_helper("created", Box::new(DateHelper));

        let template = if debug { TEMPLATE_DEBUG } else { self.template };

        let out = reg.render_template(template, &weather)?;

        Ok(out)
    }
}

#[derive(Clone, Copy)]
struct DateHelper;

impl HelperDef for DateHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>,
                            ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output) -> HelperResult {
        let fmt = match h.hash_get("format").map(|v| v.value()) {
            Some(v) => v.as_str().unwrap().to_string(),
            None => "%D %T".to_string(),
        };
        let obj = ctx.data().as_object().unwrap();
        let date = obj.get("date").unwrap().clone();
        let created_at: SystemTime = serde_json::from_value(date)?;
        let datetime: DateTime<Local> = created_at.into();
        let _ = out.write(&format!("{}", datetime.format(&fmt)));

        Ok(())
    }
}
