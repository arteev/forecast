extern crate unqlite;

use std::ops::Add;
use std::time::{Duration, SystemTime};

use unqlite::{Config, Cursor, KV, UnQLite};

use crate::{WeatherGetter, WeatherQueryType};
use crate::Error;
use crate::weather::weather::WeatherInfo;

pub struct Cache<T>(T);


pub struct UnQLiteCache {
    next: Box<dyn WeatherGetter>,
    ttl: Duration,
    unqlite: UnQLite,
}

impl UnQLiteCache {
    pub fn new(next: Box<dyn WeatherGetter>, filename: &str, ttl: Duration) -> Self {
        let unqlite = UnQLite::create(filename);
        UnQLiteCache {
            next,
            ttl,
            unqlite,
        }
    }

    fn is_expired(&self, key: &str) -> bool {
        let now = SystemTime::now();
        let key_ttl = format!("{}_ttl", key);
        let ttl_raw = self.unqlite.kv_fetch(key_ttl);
        match ttl_raw {
            Ok(data) => {
                let s = std::str::from_utf8(&data).unwrap();
                let expiration: SystemTime = serde_json::from_str(s).unwrap();

                expiration < now
            }
            Err(_) => false,
        }
    }

    fn ttl(&self, key: &str, d: Duration) {
        let key_ttl = format!("{}_ttl", key);
        let expiration = SystemTime::now()
            .add(d);
        let serialized = serde_json::to_string(&expiration).unwrap();
        self.unqlite.kv_store(key_ttl, serialized).unwrap();
    }
}

impl WeatherGetter for UnQLiteCache {
    fn get(&self, types: Vec<WeatherQueryType>) -> Result<WeatherInfo, Error> {
        const KEY: &str = "weather";
        if !self.is_expired(&KEY) {
            let cached = self.unqlite.kv_fetch(KEY);
            if let Ok(data) = cached {
                let s = std::str::from_utf8(&data).unwrap();
                let mut weather: WeatherInfo = serde_json::from_str(s)?;
                weather.is_cached = true;
                self.ttl(&KEY, self.ttl);
                return Ok(weather);
            }
        }

        let response = self.next.get(types)?;
        let serialized = serde_json::to_string(&response).unwrap();
        self.unqlite.kv_store(KEY, serialized).unwrap();
        Ok(response)
    }
}