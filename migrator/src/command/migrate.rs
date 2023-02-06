use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::error::CreateTableError;
use aws_sdk_dynamodb::error::DescribeTableError;
use aws_sdk_dynamodb::error::DescribeTableErrorKind::ResourceNotFoundException;
use aws_sdk_dynamodb::output::CreateTableOutput;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::types::SdkError::ServiceError;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, ProvisionedThroughput};
use std::{env, fs, result};
use std::process::Command as ProcessCommand;
use std::process::Output as ProcessOutput;
use std::io::{Read, Result};
use std::path::PathBuf;
use sqlx::MySqlPool;

use crate::command::{ExitCode, Output};
use crate::clients::dynamodb_client_factory::DynamodbClientFactory;
use crate::command::migrate_type::MigrateType;
use crate::command::migration_query::MigrationQuery;
use crate::settings::Settings;

const RESOURCE_FILE_DIR: &str = "resource";
const DEFAULT_MIGRATION_FILE_PATH: &str = "migrations";

#[derive(Debug, Copy, Clone)]
pub struct Migrate {
}

impl Migrate {
    pub fn new() -> Self {
        Self {}
    }

    fn migration_dir(self) -> Result<PathBuf> {
        let current_dir = env::current_dir();

        match current_dir {
            Ok(path) => Ok(path.join("src").join(RESOURCE_FILE_DIR)),
            _ => current_dir,
        }
    }

    fn read_migration_files(&self, current_path: PathBuf) -> result::Result<Vec<PathBuf>, String> {
        let mut migration_files: Vec<PathBuf> = Vec::new();

        let result = fs::read_dir(&current_path);
        match result {
            Ok(directory) => {
                for file in directory.into_iter() {
                    migration_files.push(file.expect("").path());
                }
            },
            _ => return Err(format!("Cannot read directory. path: {}", current_path.to_str().unwrap_or("cannot resolve path."))),
        }

        Ok(migration_files)
    }

    fn read_contents(self, path: &PathBuf) -> result::Result<MigrationQuery, &str> {
        let mut migration_contents = String::new();

        let migration_file = fs::File::open(path);
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

    fn read_user_contents(self, path: &PathBuf) -> result::Result<String, &str> {
        let mut migration_contents = String::new();

        let migration_file = fs::File::open(path);
        match migration_file {
            Ok(mut target_file) => {
                if target_file.read_to_string(&mut migration_contents).is_ok() {
                    return Ok(migration_contents);
                }

                Err("Cannot load migration contents. File name: ")
            },
            _ => Err("Cannot read migration file.")
        }
    }

    fn to_migration_query(self, contents: &str) -> result::Result<MigrationQuery, serde_json::Error> {
        let deserialized= serde_json::from_str(contents);

        deserialized
    }

    async fn create_table(self, table_name: &str, query: &MigrationQuery) -> result::Result<CreateTableOutput, SdkError<CreateTableError>> {
        println!("Called create_table!!!");

        println!("TableName: {}", table_name);

        let vec_attribute_definitions = query.attribute_definitions().to_vec().iter()
            .map(|attribute_definition| (
                AttributeDefinition::builder()
                    .attribute_name(attribute_definition.attribute_name()))
                    .attribute_type(attribute_definition.attribute_type())
                    .build()
            )
            .collect::<Vec<_>>();

        let vec_key_schemas = query.key_schemas().to_vec().iter()
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

        let create_table_response = self.create_client()
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

    async fn exists_table(self, table_name: &str) -> result::Result<bool, String> {
        let describe_table_response = self.create_client()
            .describe_table()
            .table_name(table_name)
            .send()
            .await;

        return match describe_table_response {
            Ok(_) => Ok(true),
            Err(ServiceError { err: DescribeTableError { kind: ResourceNotFoundException(_) , .. }, raw: _ })  => Ok(false),
            Err(error) => Err(error.to_string()),
        }
    }

    async fn create_migration_table_for_mysql(self) -> result::Result<(), sqlx::Error> {
        let pool = MySqlPool::connect("mysql://rust:rust@localhost/rust").await?;

        let rows = sqlx::query("SHOW TABLES;")
            .fetch_all(&pool)
            .await?;

        sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS migrations_dynamodb_status (id int, name text, created_at DATETIME)
            "#
        )
            .execute(&pool)
            .await?;

        dbg!(rows);

        Ok(())
    }

    async fn create_migration_table(self) -> result::Result<(), String> {
        let driver = "mysql";

        match driver {
            "mysql" => {

            },
            _ => {},
        }

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

                    match self.exists_table(query.table_name()).await {
                        Ok(true) => {},
                        Ok(false) => {
                            let create_table_result = self.create_table(query.table_name(),&query).await;

                            if let Err(error) = create_table_result { return Err(error.to_string()) }
                        },
                        Err(message) => return Err(message.to_string()),
                    }
                }
            },
            Err(message) => return Err(message),
        }

        Ok(())
    }

    async fn migrate(self, migrate_type: &MigrateType, migration_dir: PathBuf) -> result::Result<(), String> {
        match self.read_migration_files(migration_dir) {
            Ok(target_files) => {
                for migration_file in target_files {
                    if migrate_type.is_up() && !migration_file.to_str().unwrap().contains(".up.") {
                        continue;
                    }

                    if migrate_type.is_down() && !migration_file.to_str().unwrap().contains(".down.") {
                        continue;
                    }

                    println!("Run: {}", migration_file.to_str().unwrap());

                    let migration_data = self.read_user_contents(&migration_file).unwrap();

                    let output = self.execute_user_migration(migration_data);

                    println!("{}", String::from_utf8(output.stdout).unwrap_or("".to_string()))
                }
            },
            Err(message) => return Err(message),
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn execute_user_migration(self, migration_data: String) -> ProcessOutput {
        dbg!(std::env::consts::OS);

        let output = if cfg!(target_os = "windows") {
            let value = migration_data.replace("\n", "");

            dbg!(&value);

            ProcessCommand::new("cmd")
                .args(["/C", value.as_str()])
                .output()
                .expect("failed to execute process on Windows.")
        } else {
            ProcessCommand::new("sh")
                .arg("-c")
                .arg(migration_data.as_str())
                .output()
                .expect("failed to execute process on Linux.")
        };

        output
    }

    fn create_client(self) -> Client { DynamodbClientFactory::factory() }

    pub async fn execute(self, command: &MigrateType, migrate_path: &Option<PathBuf>) -> Output {
        let config = Settings::new();

        dbg!("Start execute!!!");

        let _result = self.create_migration_table_for_mysql().await;

        if let Err(message) = self.create_migration_table().await {
            return Output::new(ExitCode::FAILED, format!("Migration failed. : {}", message))
        }

        let user_migration_file_path = match migrate_path {
            Some(path) => path.to_path_buf(),
            None                => PathBuf::from(DEFAULT_MIGRATION_FILE_PATH)
        };

        if let Err(message) = self.migrate(command, user_migration_file_path).await {
            return Output::new(ExitCode::FAILED, format!("Migration failed. : {}", message))
        }

        Output::new(ExitCode::SUCCEED, "Migrate succeed.".to_string())
    }
}
