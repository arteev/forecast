extern crate unqlite;

use std::ops::Add;
use std::time::{Duration, SystemTime};

use unqlite::{KV, UnQLite};

use crate::{WeatherGetter, WeatherQueryType};
use crate::Error;
use crate::weather::weather::WeatherInfo;

const KEY: &str = "weather";

pub struct UnQLiteCache {
    next: Box<dyn WeatherGetter>,
    ttl: Duration,
    unqlite: UnQLite,
    prefer_cache: bool,
}

impl UnQLiteCache {
    pub fn new(next: Box<dyn WeatherGetter>, filename: &str, ttl: Duration, prefer_cache: bool) -> Self {
        let unqlite = UnQLite::create(filename);
        UnQLiteCache {
            next,
            ttl,
            unqlite,
            prefer_cache,
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

    fn ttl(&self, key: &str, d: Duration) -> Result<(), Error> {
        let key_ttl = format!("{}_ttl", key);
        let expiration = SystemTime::now()
            .add(d);
        let serialized = serde_json::to_string(&expiration)?;
        self.unqlite.kv_store(key_ttl, serialized).ok().ok_or(
            Error::InvalidCache("store TTL".to_string()))?;
        Ok(())
    }

    fn get_from_cache(&self) -> Result<WeatherInfo, Error> {
        let cached = self.unqlite.kv_fetch(KEY);
        match cached {
            Ok(data) => {
                let s = std::str::from_utf8(&data).unwrap();
                let mut weather: WeatherInfo = serde_json::from_str(s)?;
                weather.is_cached = true;
                Ok(weather)
            }
            Err(e) => Err(Error::InvalidCache(e.to_string())),
        }
    }
}

impl WeatherGetter for UnQLiteCache {
    fn get(&self, types: Vec<WeatherQueryType>) -> Result<WeatherInfo, Error> {
        if !self.is_expired(&KEY) {
            let w = self.get_from_cache();
            if w.is_ok() {
                return w;
            }
        }
        let response = self.next.get(types);
        if response.is_err() && self.prefer_cache {
            return self.get_from_cache();
        }

        let response = response?;

        let serialized = serde_json::to_string(&response).unwrap();

        self.unqlite.kv_store(KEY, serialized).ok().ok_or(
            Error::InvalidCache("store weather".to_string())
        )?;
        self.ttl(&KEY, self.ttl)?;
        Ok(response)
    }
}