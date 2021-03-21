use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, fmt};

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

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // This makes it so "WEBBY_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("webby").separator("__"))?;

        s.try_into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum Env {
    Development,
    Testing,
    Production,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Development => write!(f, "Development"),
            Env::Testing => write!(f, "Testing"),
            Env::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for Env {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => Env::Testing,
            "Production" => Env::Production,
            _ => Env::Development,
        }
    }
}
