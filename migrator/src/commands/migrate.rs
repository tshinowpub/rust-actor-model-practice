use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use serde_json::Error;
use aws_sdk_dynamodb::Client;

use crate::command::Command;
use crate::clients::dynamodb_client;
use crate::clients::dynamodb_client::DynamodbClient;
use crate::commands::migration_query::MigrationQuery;
use crate::config::aws_config::AwsConfig;
use crate::lexers::option_lexer::Options;

const MIGRATE_PATH: &str = "migrations";
const RESOURCE_FILE_DIR: &str = "resource";
const MANAGEMENT_TABLE_FILE_NAME: &str = "migrations.json";

#[derive(Debug)]
pub struct Migrate {
    dynamodb_client: DynamodbClient
}

impl Migrate {
    pub fn new(dynamodb_client: DynamodbClient) -> Self {
        Self {
            dynamodb_client
        }
    }

    fn current_dir(&self) -> PathBuf {
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

    fn migration_file_path(&self) -> PathBuf {
        let current_dir = &self.current_dir();

        current_dir
            .join("src")
            .join(RESOURCE_FILE_DIR)
            .join(MANAGEMENT_TABLE_FILE_NAME)
    }

    async fn migration_table_contents(&self) {
        let migration_file_path = &self.migration_file_path();

        dbg!(migration_file_path.clone());

        let mut migration_file = File::open(migration_file_path).expect("Migration file was not found.");

        let mut migration_contents = String::new();

        migration_file.read_to_string(&mut migration_contents).expect("Cannot read migration file.");

        dbg!(migration_contents.clone());

        let query = &self.parse(&migration_contents).expect("Cannot parse migration.json.");

        println!("Before create_table !!!");

        let _ = &self.create_table(query);

        println!("After create_table !!!");
    }

    fn parse(&self, contents: &str) -> Result<MigrationQuery, Error> {
        let deserialized: Result<MigrationQuery, Error> = serde_json::from_str(contents);

        deserialized
    }

    async fn create_table(&self, query: &MigrationQuery) {
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

        let _ = &self.dynamodb_client.create_table();

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
    async fn execute(&self, arguments: &Vec<String>, options: &Options) {
        println!("Migrate!!!");
        println!("{}", MIGRATE_PATH);

        let _ = &self.migration_table_contents().await;
    }

    fn command_name(&self) -> &str {
        "migrate"
    }
}
