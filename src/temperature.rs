use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Unit::Celsius => "°C",
            Unit::Fahrenheit => "°F",
            Unit::Kelvin => "K",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Temperature(i16, Unit);

impl Temperature {
    pub fn new(val: i16, unit: Unit) -> Temperature {
        Temperature(val, unit)
    }
    pub fn unit(&self) -> Unit {
        self.1
    }
    pub fn val(&self) -> i16 {
        self.0
    }
    pub fn as_unit(self, unit: Unit) -> Temperature {
        use Unit::*;
        match (self.val(), self.unit(), unit) {
            // Kelvin to Celsius
            (val, Kelvin, Celsius) => Temperature::new(val - 273.15 as i16, Celsius),
            // Kelvin to Fahrenheit
            (val, Kelvin, Fahrenheit) => Temperature::new(val * 9 / 5 - 459.67 as i16, Fahrenheit),
            // Celsius to Kelvin
            (val, Celsius, Kelvin) => Temperature::new(val + 273.15 as i16, Kelvin),
            // Celsius to Fahrenheit
            (val, Celsius, Fahrenheit) => Temperature::new(val * 9 / 5 + 32 as i16, Fahrenheit),
            // Fahrenheit to Kelvin
            (val, Fahrenheit, Kelvin) => Temperature::new((val + 459.67 as i16) * 5 / 9 as i16, Kelvin),
            // Fahrenheit to Celsius
            (val, Fahrenheit, Celsius) => Temperature::new((val - 32) * 5 / 9 as i16, Celsius),
            // Identity
            _ => self,
        }
    }
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Temperature) -> bool {
        self.val() == other.as_unit(self.unit()).val()
    }
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Temperature) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Temperature {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val().cmp(&other.as_unit(self.unit()).val())
    }
}