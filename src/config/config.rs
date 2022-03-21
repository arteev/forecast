extern crate directories;

use std::fs;
use std::path::Path;
use serde::Deserialize;
use directories::{ProjectDirs};

use crate::error::error::Error;
use super::args;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub provider: String,
    pub display: String,
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
        let cfg: Config = toml::from_str(&content)?;
        cfg.check()?;
        Ok(cfg)
    }

    fn check(&self) -> Result<(), Error> {
        println!("{:?}\n", self);
        Ok(())
    }
}