use anyhow::Result;
use serde::Deserialize;
use config::{Environment, File};


#[derive(Deserialize)]
pub(crate) struct Config {
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

#[derive(Deserialize)]
pub(crate) struct Discord {
    pub token: String,
}
