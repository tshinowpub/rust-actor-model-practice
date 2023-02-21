use std::env::VarError;
use anyhow::{Result, Context, anyhow};
use std::str::FromStr;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Migration {
    driver: Driver,
}

impl Migration {
    pub fn driver(&self) -> &Driver {
        &self.driver
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Driver {
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "dynamodb")]
    Dynamodb,
}

impl Driver {
    pub fn is_mysql(&self) -> bool {
        *self == Driver::Mysql
    }

    pub fn is_dynamodb(&self) -> bool {
        *self == Driver::Dynamodb
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display, strum_macros::IntoStaticStr)]
pub enum Environment {
    #[serde(rename = "develop")]
    #[strum(serialize = "develop")]
    Develop,
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    migration: Migration,
    env: Environment,
}

impl Settings {
    pub fn migration(&self) -> &Migration {
        &self.migration
    }
}

const CONFIG_FILE_PATH: &str = "./config/default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Error, Debug)]
pub enum EnvNotFoundError {
    #[error("Environment variable was not set. Env was empty. Error: {0}.")]
    EnvNameEmpty(#[from] VarError),
    #[error("Cannot parse environment. Env value not supported. Error: {0:?}. Supported only [develop, test, prod].")]
    CannotParseEnv(#[from] strum::ParseError),
}

impl Settings {
    pub fn new() -> Result<Settings> {
        let string_env = std::env::var("ENV")
            .map_err(|error| EnvNotFoundError::EnvNameEmpty(error))?;

        let env: Environment = Environment::from_str(string_env.as_str())
            .map_err(|error| EnvNotFoundError::CannotParseEnv(error))?;

        let config = Config::builder()
            .set_default("env", env.to_string())?
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(format!("{}{}.toml", CONFIG_FILE_PREFIX, env.to_string()).as_str()))
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .unwrap();

        Ok(config.try_deserialize::<Settings>()?)
    }
}
