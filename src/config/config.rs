extern crate directories;

use std::fs;

use directories::ProjectDirs;
use duration_string::DurationString;
use serde::Deserialize;

use crate::error::error::Error;

use super::args;
use super::yandex::ConfigYandex;

static DEFAULT_DISPLAY: &str = "{{ temperature_celsius_full }}";

#[derive(Debug, Deserialize, Clone)]
pub enum Provider {
    Yandex,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub provider: Provider,

    #[serde(default = "default_display")]
    pub display: String,

    pub cache: Option<Cache>,

    //TODO: сделать динамически подключаемым либо парсить отдельно для провайдера
    pub yandex: Option<ConfigYandex>,

    #[serde(default)]
    pub prefer_cache_error: bool,

    #[serde(default)]
    pub debug: bool,
}

fn default_display() -> String {
    DEFAULT_DISPLAY.to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cache {
    pub enabled: bool,
    pub expiration: Option<DurationString>,
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

        if cfg.cache.is_some() && arguments.no_cache {
            cfg.cache = None
        }
        cfg.prefer_cache_error = arguments.prefer_cache_error;
        cfg.debug = arguments.debug;

        cfg.check()?;
        Ok(cfg)
    }

    fn check(&self) -> Result<(), Error> {
        if let Some(cache) = &self.cache {
            if cache.enabled {
                if cache.expiration.is_none() {
                    return Err(Error::InvalidConfigCheck("expected cache.expiration".to_string()));
                }
            }
        }
        Ok(())
    }
}