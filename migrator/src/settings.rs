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

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    log: Log,
    migration: Migration,
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_DEVELOP_FILE_PATH: &str = "./config/Development.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    pub fn new() -> Result<Settings, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .unwrap()
        ;

        let settings = config
            .clone()
            .try_deserialize::<Settings>()
            .unwrap();

        println!("{:?}", &settings);

        Ok(settings)
    }
}
