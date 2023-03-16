use anyhow::{anyhow, Context};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::PutItemOutput;
use chrono::Utc;
use dynamodb_client::client::{Client, ExistsTableResultType};
use dynamodb_client::query::create_table::CreateTableQuery;
use dynamodb_client::query::delete_table::DeleteTableQuery;
use dynamodb_client::query::get_item::{GetItemQuery, Key};
use dynamodb_client::query::put_item::{Items, PutItemQuery};
use std::fmt::Debug;
use std::path::PathBuf;
use std::{env, fs};
use thiserror::__private::PathAsDisplay;

use crate::command::migrate_operation_type::MigrateOperationType;
use crate::command::migrate_type::MigrateType;
use crate::command::{ExitCode, Output};
use crate::parser::Parser;

const RESOURCE_FILE_DIR: &str = "resource";
const DEFAULT_MIGRATION_FILE_PATH: &str = "migrations";

#[derive(Debug, Clone)]
pub struct Migrate {
    client: Client
}

impl Migrate {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn execute(
        &self,
        command: &MigrateType,
        migrate_path: Option<&PathBuf>,
    ) -> anyhow::Result<Output> {
        match command.to_owned() {
            MigrateType::Up => {
                self.create_migration_table_for_dynamodb()
                    .await
                    .map_err(|error| {
                        anyhow!(format!(
                    "Failed create default migration table. Error: {}",
                    error
                ))
                    })?;

                let path =
                    self.migrate_path_resolver()(migrate_path, PathBuf::from(DEFAULT_MIGRATION_FILE_PATH));

                self.migrate(path).await.map_err(|error| {
                    anyhow!(format!("Failed user migration data. Error: {}", error))
                })?;

                Ok(Output::new(ExitCode::Succeed, "All migrate succeed."))
            },
            MigrateType::Down => Ok(Output::new(ExitCode::Succeed, "Migrate down succeed."))
        }
    }

    fn read_migration_files(&self, current_path: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        let directories = fs::read_dir(&current_path).context(format!(
            "Cannot resolve path. File: {} ",
            current_path.as_display()
        ))?;

        let mut migration_files = vec![];
        for directory in directories {
            migration_files.push(directory.context("Cannot resolve path.")?.path());
        }

        Ok(migration_files)
    }

    async fn create_migration_table_for_dynamodb(&self) -> anyhow::Result<()> {
        for migration_file in self
            .read_migration_files(self.migration_dir()?)
            .context("")?
        {
            let data =
                std::fs::File::open(&migration_file).context("Cannot read migration file.")?;

            let query = Parser::from_json_file::<CreateTableQuery>(&data)?;

            if ExistsTableResultType::NotFound
                == self.client
                    .exists_table(query.table_name())
                    .await
                    .context("Cannot check exists table.")?
            {
                self.client
                    .create_table(query.table_name(), &query)
                    .await
                    .context("Cannot create table.")?;
            }
        }

        Ok(())
    }

    async fn migrate(&self, target_path: PathBuf) -> anyhow::Result<()> {
        let files = self
            .read_migration_files(target_path)
            .context("Cannot read migration file.")?;

        for file in files {
            let operation_type = MigrateOperationType::resolve(&file)?;

            let file_name = file
                .file_name()
                .context(format!("Cannot get filename from PathBuf. {:?}", file))?
                .to_string_lossy()
                .to_string();

            let query = GetItemQuery::new(
                "migrations".to_string(),
                Key::new("FileName".to_string(), AttributeValue::S(file_name.clone())),
                true,
            );

            println!("Running file name {}", &file_name);

            match (self.client.get_item(&query).await?.item(), operation_type) {
                (Some(_), _) => {
                    println!(
                        "File name {} was already executed. This file was skipped.",
                        file_name
                    )
                }
                (None, MigrateOperationType::CreateTable) => {
                    let data = std::fs::File::open(&file).context(format!(
                        "Cannot open migration file. FileName: {}",
                        file_name
                    ))?;

                    let query = Parser::from_json_file::<CreateTableQuery>(&data)?;

                    self.client
                        .create_table(query.table_name(), &query)
                        .await?;
                    self.add_migration_record(&file).await?;
                }
                (None, MigrateOperationType::DeleteTable) => {
                    let data = std::fs::File::open(&file).context(format!(
                        "Cannot open migration file. FileName: {}",
                        file_name
                    ))?;

                    let query = Parser::from_json_file::<DeleteTableQuery>(&data)?;

                    self.client
                        .delete_table(&query)
                        .await
                        .context("Cannot delete table. {}")?;
                    self.add_migration_record(&file).await?;
                }
                (_, _) => {
                    println!("File name {} was skipped. Unsupported command.", file_name)
                }
            }
        }

        Ok(())
    }

    fn migration_dir(&self) -> anyhow::Result<PathBuf> {
        let migration_dir = env::current_dir()
            .context("Cannot find current_dir.")?
            .join("src")
            .join(RESOURCE_FILE_DIR);

        Ok(migration_dir)
    }

    fn migrate_path_resolver(
        &self,
    ) -> fn(migrate_path: Option<&PathBuf>, default: PathBuf) -> PathBuf {
        |migrate_path, default| match migrate_path {
            Some(path) => path.to_path_buf(),
            _ => default,
        }
    }

    async fn add_migration_record(&self, file: &PathBuf) -> anyhow::Result<PutItemOutput> {
        let file_name = AttributeValue::S(
            file.file_name()
                .context(format!("Cannot get filename from PathBuf. {:?}", file))?
                .to_string_lossy()
                .to_string(),
        );

        let mut items = Items::new();

        items.insert("FileName".to_string(), file_name);
        items.insert(
            "ExecutedAt".to_string(),
            AttributeValue::S(Utc::now().to_string()),
        );

        let query = PutItemQuery::new("migrations".to_string(), items, None, None::<String>);

        let response = self.client
            .put_item(query)
            .await
            .context("Failed put item.")?;

        Ok(response)
    }
}
