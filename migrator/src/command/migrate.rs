use anyhow::{Context, anyhow};
use aws_sdk_dynamodb::error::DescribeTableError;
use aws_sdk_dynamodb::error::DescribeTableErrorKind::ResourceNotFoundException;
use aws_sdk_dynamodb::output::CreateTableOutput;
use aws_sdk_dynamodb::types::SdkError::ServiceError;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, ProvisionedThroughput};
use std::{env, fs};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::path::PathBuf;
use aws_sdk_dynamodb::operation::CreateTable;
use serde::Deserialize;
use sqlx::MySqlPool;
use sqlx::query::Query;
use thiserror::__private::PathAsDisplay;

use crate::command::{ExitCode, Output};
use crate::clients::dynamodb_client_factory::DynamodbClientFactory;
use crate::clients::client::Client;
use crate::command::migrate_operation_type::MigrateOperationType;
use crate::command::migrate_type::MigrateType;
use crate::command::query::create_table::CreateTableQuery;
use crate::command::query::delete_table::DeleteTableQuery;
use crate::command::query::put_item::PutItemQuery;
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

    async fn exists_table(self, table_name: &str) -> anyhow::Result<ExistsTableResultType> {
        let describe_table_response = DynamodbClientFactory::factory()
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
            let data = std::fs::File::open(&migration_file).context("Cannot read migration file.")?;

            let query = self.from_json_file::<CreateTableQuery>(data)?;

            if ExistsTableResultType::NotFound == self.exists_table(query.table_name()).await? {
                Client::default().create_table(query.table_name(), &query).await.context("Cannot create table. {}")?;
            }
        }

        Ok(())
    }

    pub async fn execute(self, command: &MigrateType, migrate_path: Option<&PathBuf>) -> Output {
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
                return Output::new(ExitCode::FAILED, format!("Failed create migration table. : {}", message))
            }
        }

        let path = self.migrate_path_resolver()(migrate_path, PathBuf::from(DEFAULT_MIGRATION_FILE_PATH));
        if let Err(message) = self.migrate(path).await {
            return Output::new(ExitCode::FAILED, format!("Migration failed. : {}", message))
        }

        Output::new(ExitCode::SUCCEED, "Migrate succeed.".to_string())
    }

    async fn migrate(self, target_path: PathBuf) -> anyhow::Result<()> {
        let files = self.read_migration_files(target_path)?;

        for file in files {
            let operation_type = MigrateOperationType::resolve(&file)?;

            let data = std::fs::File::open(&file)?;

            match operation_type {
                MigrateOperationType::CreateTable => {
                    let query = self.from_json_file::<CreateTableQuery>(data)?;

                    let output= Client::default().create_table(query.table_name(), &query).await.context("Cannot create table. {}")?;

                    dbg!(output);
                },
                MigrateOperationType::DeleteTable => {
                    let query = self.from_json_file::<DeleteTableQuery>(data);
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn from_json_file<T: for<'a> Deserialize<'a>>(self, file: std::fs::File) -> anyhow::Result<T> {
        let aaa: T = serde_json::from_reader(file).context("Cannot parse json file.")?;

        return Ok(aaa)
    }

    fn migration_dir(self) -> anyhow::Result<PathBuf> {
        Ok(env::current_dir()
            .context("Cannot find current_dir.")?
            .join("src")
            .join(RESOURCE_FILE_DIR))
    }

    fn migrate_path_resolver(self) -> fn(migrate_path: Option<&PathBuf>, default: PathBuf) -> PathBuf {
        |migrate_path, default|
            match migrate_path {
                Some(path) => path.to_path_buf(),
                _ => default,
            }
    }

    fn add_migration_record(self, file: &PathBuf) {
        // DynamoDB に実行したファイルの名前を登録する
    }
}

#[derive(Debug, PartialEq)]
enum ExistsTableResultType {
    Found,
    NotFound,
}
