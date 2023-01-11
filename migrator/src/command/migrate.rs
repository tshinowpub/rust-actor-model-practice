use async_trait::async_trait;
use std::{env, fs, result};
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, ProvisionedThroughput};
use serde_json::Error;
use aws_sdk_dynamodb::{Client, Credentials, Endpoint, Region};
use aws_sdk_dynamodb::error::CreateTableError;
use aws_sdk_dynamodb::output::CreateTableOutput;
use aws_sdk_dynamodb::types::SdkError;
use http::Uri;

use crate::command::{Command, ExitCode, Output};
use crate::clients::dynamodb_client;
use crate::clients::dynamodb_client::DynamodbClient;
use crate::command::migration_query::MigrationQuery;
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

    async fn create_table(self, table_name: &str, query: &MigrationQuery) -> result::Result<CreateTableOutput, SdkError<CreateTableError>> {
        println!("Called create_table!!!");

        println!("TableName: {}", table_name);

        let attribute_definitions = query.attribute_definitions();
        let mapped_attribute_definitions = attribute_definitions.to_vec();

        let vec_attribute_definitions = mapped_attribute_definitions.iter()
            .map(|attribute_definition| (
                AttributeDefinition::builder()
                    .attribute_name(attribute_definition.attribute_name()))
                    .attribute_type(attribute_definition.attribute_type())
                    .build()
            )
            .collect::<Vec<_>>();

        let key_schemas = query.key_schemas();

        let mapped_key_schemas = key_schemas.to_vec();

        let vec_key_schemas = mapped_key_schemas.iter()
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

        let endpoint = Endpoint::immutable(Uri::from_static("http://localhost:8000"));
        let dynamodb_local_config = aws_sdk_dynamodb::Config::builder()
            .region(Region::new("ap-northeast-1"))
            .endpoint_resolver(endpoint)
            .credentials_provider(Credentials::new("test", "test", None, None, "default"))
            .build();

        let client = Client::from_conf(dynamodb_local_config);

        let create_table_response = client
            .create_table()
            .table_name(table_name)
            .set_attribute_definitions(Some(vec_attribute_definitions))
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

    async fn exists_table(self, table_name: &str) -> bool {
        let client = self.create_client();

        let describe_table_response = client
            .describe_table()
            .table_name(table_name)
            .send()
            .await;

        false
    }

    async fn create_migration_table(self) -> result::Result<(), String> {
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

                    if self.exists_table(query.table_name()).await == true { return Ok(()) }

                    let create_table_result = self.create_table(query.table_name(),&query).await;

                    if let Err(error) = create_table_result { return Err(error.to_string()) }
                }
            },
            _ => return Err("Cannot read migration files.".to_string()),
        }

        Ok(())
    }

    fn create_client(self) -> Client {
        let endpoint = Endpoint::immutable(Uri::from_static("http://localhost:8000"));
        let dynamodb_local_config = aws_sdk_dynamodb::Config::builder()
            .region(Region::new("ap-northeast-1"))
            .endpoint_resolver(endpoint)
            .credentials_provider(Credentials::new("test", "test", None, None, "default"))
            .build();

        Client::from_conf(dynamodb_local_config)
    }
}

#[async_trait]
impl Command for Migrate {
    async fn execute(&self, arguments: &Vec<String>, options: &Options) -> Output {
        let result = self.create_migration_table().await;
        if let Err(message) = result {
            return Output::new(ExitCode::FAILED, message)
        }

        Output::new(ExitCode::SUCCEED, "Migrate command succeed.".to_string())
    }

    fn command_name(self) -> &'static str {
        "migrate"
    }
}
