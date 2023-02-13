use anyhow::{Result, Context, anyhow};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::error::CreateTableError;
use aws_sdk_dynamodb::error::DescribeTableError;
use aws_sdk_dynamodb::error::DescribeTableErrorKind::ResourceNotFoundException;
use aws_sdk_dynamodb::output::CreateTableOutput;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::types::SdkError::ServiceError;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, ProvisionedThroughput};
use std::{env, fs, result};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::io::Read;
use std::path::PathBuf;
use sqlx::MySqlPool;
use thiserror::__private::PathAsDisplay;

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

    fn read_migration_files(&self, current_path: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        let directories = fs::read_dir(&current_path).context(format!("Cannot resolve path. File: {} ", current_path.as_display()))?;

        let mut migration_files= vec![];
        for directory in directories {
            migration_files.push(directory.context(format!("Cannot resolve path."))?.path());
        }

        Ok(migration_files)
    }

    fn read_contents(self, path: &PathBuf) -> anyhow::Result<MigrationQuery> {
        let mut migration_contents = String::new();
        if fs::File::open(path).context("")?.read_to_string(&mut migration_contents).is_ok() {
            let query = self.to_migration_query(&migration_contents).context("Cannot parse json file.")?;

            return Ok(query)
        }

        Err(anyhow::anyhow!("Cannot read migration file."))
    }

    fn read_user_contents(self, path: &PathBuf) -> anyhow::Result<String> {
        let mut migration_contents = String::new();

        let mut target_file = fs::File::open(path).context("Cannot read migration file.")?;
        if target_file.read_to_string(&mut migration_contents).is_ok() {
            return Ok(migration_contents);
        }

        Err(anyhow!("Cannot load migration contents. File name: "))
    }

    fn to_migration_query(self, contents: &str) -> anyhow::Result<MigrationQuery> {
        let deserialized= serde_json::from_str(contents);

        Ok(deserialized?)
    }

    async fn create_table(self, table_name: &str, query: &MigrationQuery) -> anyhow::Result<CreateTableOutput> {
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

        Ok(create_table_response?)
    }

    async fn exists_table(self, table_name: &str) -> anyhow::Result<ExistsTableResultType> {
        let describe_table_response = self.create_client()
            .describe_table()
            .table_name(table_name)
            .send()
            .await;

        return match describe_table_response {
            Ok(_) => Ok(ExistsTableResultType::Found),
            Err(ServiceError { err: DescribeTableError { kind: ResourceNotFoundException(_) , .. }, raw: _ })  => Ok(ExistsTableResultType::NotFound),
            Err(error) => Err(anyhow!(error.to_string())),
        }
    }

    async fn create_migration_table_for_mysql(self) -> anyhow::Result<()> {
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

    async fn create_migration_table_for_dynamodb(self) -> anyhow::Result<()> {
        for migration_file in self.read_migration_files(self.migration_dir()?).context("")? {
            let query = self.read_contents(&migration_file).context("Cannot read migration file.")?;

            if ExistsTableResultType::NotFound == self.exists_table(query.table_name()).await? {
                self.create_table(query.table_name(), &query).await.context("Cannot create table. {}")?;
            }
        }

        Ok(())
    }

    fn migration_dir(self) -> anyhow::Result<PathBuf> {
        Ok(env::current_dir()
            .context("Cannot find current_dir.")?
            .join("src")
            .join(RESOURCE_FILE_DIR))
    }

    async fn migrate(self, migrate_type: &MigrateType, migration_dir: PathBuf) -> anyhow::Result<()> {
        let targets = self.read_migration_files(migration_dir)?;

        for migration_file in targets {
            if migrate_type.is_up() && !migration_file.to_str().unwrap().contains(".up.") {
                continue;
            }

            if migrate_type.is_down() && !migration_file.to_str().unwrap().contains(".down.") {
                continue;
            }

            println!("Run: {:?}", migration_file.to_str().unwrap());

            let output = self.execute_user_migration(self.read_user_contents(&migration_file)?);

            println!("{}", String::from_utf8(output.stdout).unwrap_or("".to_string()))
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn execute_user_migration(self, migration_data: String) -> std::process::Output {
        dbg!(std::env::consts::OS);

        let output = if cfg!(target_os = "windows") {
            let value = migration_data.replace("\n", "");

            dbg!(&value);

            std::process::Command::new("cmd")
                .args(["/C", value.as_str()])
                .output()
                .expect("failed to execute process on Windows.")
        } else {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(migration_data.as_str())
                .output()
                .expect("failed to execute process on Linux.")
        };

        output
    }

    fn create_client(self) -> Client { DynamodbClientFactory::factory() }

    pub async fn execute(self, command: &MigrateType, migrate_path: Option<&PathBuf>) -> Output {
        println!("Start migrate command...");

        let result= Settings::new();
        if let Err(error) = result {
            return Output::new(ExitCode::FAILED, format!("Cannot load config. Value ENV was not found. : {}", error.to_string()))
        }

        let config = result.unwrap();
        if config.borrow().migration().driver().is_mysql() {
            println!("MySQL selected...");

            let _result = self.create_migration_table_for_mysql().await;
        }

        if config.borrow().migration().driver().is_dynamodb() {
            println!("DynamoDB selected...");

            if let Err(message) = self.create_migration_table_for_dynamodb().await {
                return Output::new(ExitCode::FAILED, format!("Migration failed. : {}", message))
            }
        }

        let path = self.migrate_path_resolver()(migrate_path, PathBuf::from(DEFAULT_MIGRATION_FILE_PATH));
        if let Err(message) = self.migrate(command, path).await {
            return Output::new(ExitCode::FAILED, format!("Migration failed. : {}", message))
        }

        Output::new(ExitCode::SUCCEED, "Migrate succeed.".to_string())
    }

    fn migrate_path_resolver(self) -> fn(migrate_path: Option<&PathBuf>, default: PathBuf) -> PathBuf {
        (|migrate_path, default| (
            match migrate_path {
                Some(path) => path.to_path_buf(),
                _                   => default,
            }
        ))
    }
}

#[derive(Debug, PartialEq)]
enum ExistsTableResultType {
    Found,
    NotFound,
}
