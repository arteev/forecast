use std::collections::HashMap;
use handlebars::Handlebars;
use crate::Error;
use crate::weather::weather::WeatherInfo;

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
        let mut reg = Handlebars::new();
        //reg.set_strict_mode(true);

        let out = reg.render_template(self.template, w)?;
        Ok(out)
    }
}