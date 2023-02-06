use std::str::FromStr;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Migration {
    driver: MigrationType,
}

#[derive(Clone, Debug, Deserialize)]
pub enum MigrationType {
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "dynamodb")]
    Dynamodb,
}

#[derive(Clone, Debug, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseEnvironmentError;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    log: Log,
    migration: Migration,
    env: Environment,
}

const CONFIG_FILE_PATH: &str = "./config/default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    pub fn new() -> Result<Settings, ConfigError> {
        let env: Environment = Environment::from_str(std::env::var("ENV").unwrap().as_str()).unwrap();

        let config = Config::builder()
            .set_default("env", format!("{}", env))?
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(format!("{}{}.toml", CONFIG_FILE_PREFIX, env).as_str()))
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .unwrap();

        let settings = config
            .clone()
            .try_deserialize::<Settings>()
            .unwrap();

        println!("{:?}", &settings);

        Ok(settings)
    }
}
