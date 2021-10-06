use std::str::FromStr;

use anyhow::Result;
use serde::Deserialize;
use config::{Environment, File};
use tracing::Level;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub daemon: Daemon,
    pub discord: Discord,
}

impl Config {
    pub(crate) fn load_config() -> Result<Self> {
        let mut config = config::Config::new();

        config.merge(File::with_name("hey_listen.yaml").required(false))?;
        config.merge(Environment::with_prefix("HEY_LISTEN"))?;

        Ok(config.try_into()?)
    }
}

impl Validity for Config {
    fn is_valid(&self) -> Result<()> {
        self.daemon.is_valid()?;
        self.discord.is_valid()?;

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Daemon{
    pub log_level: String,
}

impl Validity for Daemon {
    fn is_valid(&self) -> Result<()> {
        Level::from_str(&self.log_level)?;
        
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Discord {
    pub token: String,
}

impl Validity for Discord {
    fn is_valid(&self) -> Result<()> {
        Ok(())
    }
}

trait Validity {
    fn is_valid(&self) -> Result<()>;
}