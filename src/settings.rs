use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use crate::models::{Job, Meta, Person};

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("need to load configuration");
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub person: Person,
    pub job: Job,
    pub meta: Meta,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // This makes it so "WEBBY_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("webby").separator("__"))?;

        s.try_into()
    }
}
