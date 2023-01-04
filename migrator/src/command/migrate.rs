use async_trait::async_trait;
use std::{env, fs, io, result};
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use std::process::exit;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use serde_json::Error;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::error::CreateTableError;
use aws_sdk_dynamodb::output::CreateTableOutput;
use aws_sdk_dynamodb::types::SdkError;

use crate::command::{Command, ExitCode, Output};
use crate::clients::dynamodb_client;
use crate::clients::dynamodb_client::DynamodbClient;
use crate::command::migration_query::MigrationQuery;
use crate::config::aws_config::AwsConfig;
use crate::lexer::option_lexer::Options;

const MIGRATE_PATH: &str = "migrations";
const RESOURCE_FILE_DIR: &str = "resource";

#[derive(Debug, Copy, Clone)]
pub struct Migrate {
}

impl Migrate {
    pub fn new(dynamodb_client: DynamodbClient) -> Self {
        Self {}
    }

    fn migration_dir(self) -> Result<PathBuf> {
        let current_dir = env::current_dir();

        match current_dir {
            Ok(path) => Ok(path.join("src").join(RESOURCE_FILE_DIR)),
            _ => current_dir,
        }
    }

    fn read_migration_files(&self, current_path: PathBuf) -> result::Result<Vec<PathBuf>, &str> {
        let mut migration_files: Vec<PathBuf> = Vec::new();

        let result = fs::read_dir(current_path);
        match result {
            Ok(directory) => {
                for file in directory.into_iter() {
                    migration_files.push(file.expect("").path());
                }
            },
            _ => return Err("aaaa"),
        }

        Ok(migration_files)
    }

    fn read_contents(self, path: &PathBuf) -> result::Result<MigrationQuery, &str> {
        let mut migration_contents = String::new();

        let mut migration_file = File::open(path);
        match migration_file {
            Ok(mut target_file) => {
                if target_file.read_to_string(&mut migration_contents).is_ok() {
                    return match self.to_migration_query(&migration_contents) {
                        Ok(query) => Ok(query),
                        _ => Err("Cannot parse json file.")
                    }
                }

                Err("Cannot load migration contents. File name: ")
            },
            _ => Err("Cannot read migration file.")
        }
    }

    fn to_migration_query(self, contents: &str) -> result::Result<MigrationQuery, Error> {
        let deserialized= serde_json::from_str(contents);

        deserialized
    }

    async fn create_table(self, query: &MigrationQuery) -> result::Result<CreateTableOutput, SdkError<CreateTableError>> {
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
        
        return match create_table_response {
            Ok(output) => {
                dbg!("{}", output.table_description());

                Ok(output)
            },
            Err(error) => {
                dbg!("{}", error.to_string());

                Err(error)
            }
        }
    }

    fn exists_migration_table(self) -> bool {
        false
    }

    async fn create_migration_table(self) -> result::Result<bool, String> {
        let migration_dir;
        match self.migration_dir() {
            Ok(target_dir) => migration_dir = target_dir,
            Err(error)       => return Err(format!("Failed to get current execute path: {}.", error)),
        }

        match self.read_migration_files(migration_dir) {
            Ok(target_files) => {
                for migration_file in target_files {
                    println!("{}", migration_file.to_str().unwrap());

                    let query = self.read_contents(&migration_file).unwrap();

                    let create_table_result = self.create_table(&query).await;

                    match create_table_result {
                        Ok(output)       => return Ok(true),
                        Err(error) => return Err(error.to_string()),
                    }
                }
            },
            _ => return Err("Cannot read migration files.".to_string()),
        }

        Ok(true)
    }
}

#[async_trait]
impl Command for Migrate {
    async fn execute(&self, arguments: &Vec<String>, options: &Options) -> Output {
        if self.exists_migration_table() == false {
            let result = self.create_migration_table().await;
            if let Err(message) = result {
                return Output::new(ExitCode::FAILED, message)
            }
        }

        Output::new(ExitCode::SUCCEED, "Migrate command succeed.".to_string())
    }

    fn command_name(self) -> &'static str {
        "migrate"
    }
}
