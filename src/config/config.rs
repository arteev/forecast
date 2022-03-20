use std::fs;
use std::path::Path;
use serde::Deserialize;
use crate::error::error::Error;

extern crate directories;

use directories::{ProjectDirs};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub provider: String,
    pub display: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        //todo: передать конфиг из параметров командной строки
        let path = {
            if let Some(dirs) = ProjectDirs::from("", "", "forecast") {
                let mut dir = dirs.config_dir().join("config.toml");
                Ok(dir.as_path().to_str().unwrap().to_string())
            } else {
                Err(Error::MissingConfig)
            }
        }?;
        let content = fs::read_to_string(&path).ok().ok_or(Error::FailedReadConfig)?;
        let cfg: Config = toml::from_str(&content)?;
        cfg.check()?;
        Ok(cfg)
    }

    fn check(&self) -> Result<(), Error> {
        println!("{:?}\n", self);
        Ok(())
    }
}