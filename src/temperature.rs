use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize)]
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

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_converter() {
        use Unit::*;

        assert_eq!(
            Temperature(0, Celsius).as_unit(Fahrenheit),
            Temperature(32, Fahrenheit)
        );

        assert_eq!(
            Temperature(-50, Celsius).as_unit(Kelvin),
            Temperature(223.15 as i16, Kelvin)
        );

        assert_eq!(
            Temperature(32, Fahrenheit).as_unit(Celsius),
            Temperature(0, Celsius)
        );

        assert_eq!(
            Temperature(-459.67 as i16, Fahrenheit).as_unit(Kelvin),
            Temperature(0, Kelvin)
        );

        assert_eq!(
            Temperature(223.15 as i16, Kelvin).as_unit(Celsius),
            Temperature(-50, Celsius)
        );

        assert_eq!(
            Temperature(0, Kelvin).as_unit(Fahrenheit),
            Temperature(-459.67 as i16, Fahrenheit)
        );

        assert_ne!(
            Temperature(5, Celsius),
            Temperature(4, Celsius)
        )
    }

    #[test]
    fn unit_cmp() {
        use Unit::*;
        assert!(Temperature(0, Celsius) <= Temperature(0, Celsius));
        assert!(!(Temperature(0, Celsius) < Temperature(0, Celsius)));

        assert!(Temperature(1, Celsius) > Temperature(30, Fahrenheit));
        assert!(Temperature(100, Kelvin) < Temperature(25, Celsius));
    }

    #[test]
    fn unit_display() {
        use Unit::*;
        assert_eq!(format!("10{}", Celsius), "10°C");
        assert_eq!(format!("1{}", Fahrenheit), "1°F");
        assert_eq!(format!("100{}", Kelvin), "100K");
    }

    #[test]
    fn temp_display() {
        use Unit::*;
        assert_eq!(format!("{}", Temperature::new(10, Celsius)), "10°C");
        assert_eq!(format!("{}", Temperature::new(247, Fahrenheit)), "247°F");
        assert_eq!(format!("{}", Temperature::new(90, Kelvin)), "90K");
    }
}
