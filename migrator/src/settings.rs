use anyhow::Result;
use config::{Config, File};
use serde::Deserialize;
use std::env::VarError;
use std::str::FromStr;
use dotenvy::dotenv;
use http::Uri;
use thiserror::Error;

#[derive(
    Clone,
    Debug,
    Deserialize,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
)]
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
    #[serde(rename = "ci")]
    #[strum(serialize = "ci")]
    Ci,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[allow(dead_code)]
    env: Environment,
    #[allow(dead_code)]
    log: Log,
    #[allow(dead_code)]
    dynamodb: DynamoDb
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    #[allow(dead_code)]
    level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DynamoDb {
    #[allow(dead_code)]
    scheme: String,
    #[allow(dead_code)]
    host: String,
    #[allow(dead_code)]
    port: u16,
}

impl DynamoDb {
    pub(crate) fn uri(&self) -> Result<Uri, http::Error> {
        Uri::builder()
            .scheme(self.scheme.as_str())
            .authority(format!("{}:{}", self.host, self.port))
            .path_and_query("/")
            .build()
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
        dotenv().ok();

        let string_env =
            std::env::var("ENV").map_err(EnvNotFoundError::EnvNameEmpty)?;

        let env: Environment = Environment::from_str(string_env.as_str())
            .map_err(EnvNotFoundError::CannotParseEnv)?;

        let config = Config::builder()
            .set_default("env", env.to_string())?
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(
                format!("{}{}.toml", CONFIG_FILE_PREFIX, env).as_str(),
            ))
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .expect("Cannot load config.");

        Ok(config.try_deserialize::<Settings>()?)
    }

    pub(crate) fn dynamodb(&self) -> &DynamoDb {
        &self.dynamodb
    }
}
