extern crate directories;

use std::fs;
use std::path::Path;
use std::time::Duration;

use directories::ProjectDirs;
use duration_string::DurationString;
use serde::Deserialize;

use crate::error::error::Error;

use super::args;
use super::yandex::ConfigYandex;

static DEFAULT_DISPLAY: &str = "{{ temperature_celsius }}";

#[derive(Debug, Deserialize, Clone)]
pub enum Provider {
    Yandex,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub provider: Provider,
    pub display: Option<String>,

    pub cache: Option<Cache>,

    pub ratelimit: Option<RateLimit>,

    //TODO: сделать динамически подключаемым либо парсить отдельно для провайдера
    pub yandex: Option<ConfigYandex>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cache {
    pub enabled: bool,
    pub expiration: Option<DurationString>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimit {
    pub enabled: bool,
    pub limit: u32,
    pub period: Period,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Period {
    Second,
    Minute,
    Hour,
    Day,
}


impl Config {
    pub fn new() -> Result<Self, Error> {
        let arguments = args::parse();
        let path = {
            if let Some(config_file) = arguments.config_file {
                Ok(config_file)
            } else if let Some(dirs) = ProjectDirs::from("", "", "forecast") {
                let dir = dirs.config_dir().join("config.toml");
                Ok(dir.as_path().to_str().unwrap().to_string())
            } else {
                Err(Error::MissingConfig)
            }
        }?;
        let content = fs::read_to_string(&path).ok().ok_or(Error::FailedReadConfig)?;
        let mut cfg: Config = toml::from_str(&content)?;
        if cfg.display.is_none() {
            cfg.display = Some(DEFAULT_DISPLAY.to_string())
        }
        cfg.check()?;
        Ok(cfg)
    }

    fn check(&self) -> Result<(), Error> {
        if let Some(rate) = &self.ratelimit {
            if rate.enabled {
                if rate.limit == 0 {
                    return Err(Error::InvalidConfigCheck(format!("wrong limit value {}", rate.limit)));
                }
            }
        }
        if let Some(cache) = &self.cache {
            if cache.enabled {
                if cache.expiration.is_none() {
                    return Err(Error::InvalidConfigCheck("expected cache.expiration".to_string()));
                }
            }
        }
        println!("{:?}\n", self);
        Ok(())
    }
}