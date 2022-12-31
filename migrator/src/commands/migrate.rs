use async_trait::async_trait;
use std::{env, fs};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use serde_json::Error;
use aws_sdk_dynamodb::Client;

use crate::command::{Command, ExitCode, Output};
use crate::clients::dynamodb_client;
use crate::clients::dynamodb_client::DynamodbClient;
use crate::commands::migration_query::MigrationQuery;
use crate::config::aws_config::AwsConfig;
use crate::lexers::option_lexer::Options;

const MIGRATE_PATH: &str = "migrations";
const RESOURCE_FILE_DIR: &str = "resource";

#[derive(Debug, Copy, Clone)]
pub struct Migrate {
}

impl Migrate {
    pub fn new(dynamodb_client: DynamodbClient) -> Self {
        Self {}
    }

    fn current_dir(self) -> PathBuf {
        let current_dir;
        match env::current_dir() {
            Ok(path) => current_dir = path,
            Err(error) => {
                println!("Failed to get current execute path: {}.", error);

                exit(1);
            },
        };

        current_dir
    }

    fn migration_dir(self) -> PathBuf {
        let current_dir = self.current_dir();

        current_dir
            .join("src")
            .join(RESOURCE_FILE_DIR)
    }

    fn read_migration_files(&self) -> Result<Vec<PathBuf>, &str> {
        let mut migration_files: Vec<PathBuf> = Vec::new();

        let result = fs::read_dir(self.migration_dir());
        match result {
            Ok(directory) => {
                for file in directory.into_iter() {
                    migration_files.push(file.expect("").path());
                }
            },
            _ => return Err("Cannot read migration files.")
        }

        Ok(migration_files)
    }

    fn read_contents(self, path: &PathBuf) -> Result<MigrationQuery, Error> {
        let mut migration_contents = String::new();

        let mut migration_file = self.read_file(path);
        migration_file
            .read_to_string(&mut migration_contents)
            .expect("Cannot read migration file.");

        self.to_migration_query(&migration_contents)
    }

    fn read_file(self, path: &PathBuf) -> File {
        let file = File::open(path).expect("File was not found.");

        file
    }

    fn to_migration_query(self, contents: &str) -> Result<MigrationQuery, Error> {
        let deserialized= serde_json::from_str(contents);

        deserialized
    }

    async fn create_table(self, query: &MigrationQuery) {
        println!("Called create_table!!!");

        let table_name = query.table_name();

        println!("TableName: {}", table_name);

        /*
        let attribute_definition = AttributeDefinition::builder()
            .attribute_name(&a_name)
                .attribute_type(ScalarAttributeType::S)
                .build();
*/

        let key_schemas = query.key_schemas();

        let map_key_schemas = key_schemas.to_vec();

        let vec_key_schemas = map_key_schemas.iter()
            .map(|key_schema| (
                KeySchemaElement::builder()
                    .attribute_name(key_schema.attribute_name()))
                    .key_type(key_schema.key_type())
                    .build()
            )
            .collect::<Vec<_>>();

        let input_provisioned_throughput = query.provisioned_throughput();

        let provisioned_throughput = ProvisionedThroughput::builder()
            .read_capacity_units(*input_provisioned_throughput.read_capacity_units())
            .write_capacity_units(*input_provisioned_throughput.write_capacity_units())
            .build();

        let shared_config = AwsConfig::aws_config().await;
        let client = Client::new(&shared_config);

        let create_table_response = client
            .create_table()
            .table_name(table_name)
            //.key_schema(key_schema_element)
            //.attribute_definitions(attribute_definition)
            .set_key_schema(Some(vec_key_schemas))
            .provisioned_throughput(provisioned_throughput)
            .send()
            .await;

        match create_table_response {
            Ok(output) => {
                dbg!("{}", output.table_description());
            },
            Err(error) => {
                dbg!("{}", error.to_string());
            }
        }

        println!("Table {} was created!!!", table_name)
    }
}

#[async_trait]
impl Command for Migrate {
    async fn execute(&self, arguments: &Vec<String>, options: &Options) -> Output {
        println!("Migrate!!!");
        println!("{}", MIGRATE_PATH);

        match self.read_migration_files() {
            Ok(target_files) => {
                for migration_file in target_files {
                    println!("{}", migration_file.to_str().unwrap());

                    let query = self.read_contents(&migration_file).unwrap();

                    self.create_table(&query).await;
                }
            },
            _ => {
                panic!("Cannot read migration files.");
            }
        }

        Output::new(ExitCode::SUCCEED, "".to_string())
    }

    fn command_name(self) -> &'static str {
        "migrate"
    }
}
