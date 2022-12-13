use std::env;
use std::fs::{create_dir, create_dir_all, File, read};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::command::Command;
use crate::clients::dynamodb_client;
use crate::clients::dynamodb_client::DynamodbClient;
use crate::lexers::option_lexer::Options;

const MIGRATE_PATH: &str = "migrations";
const RESOURCE_FILE_DIR: &str = "resource";
const MANAGEMENT_TABLE_FILE_NAME: &str = "migrations.json";

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

    fn migration_table_contents(&self) -> String {
        let migration_file_path = &self.migration_file_path();

        dbg!(migration_file_path.clone());

        let mut migration_file = File::open(migration_file_path).expect("Migration file was not found.");

        let mut migration_contents = String::new();

        migration_file.read_to_string(&mut migration_contents).expect("Cannot read migration file.");

        dbg!(migration_contents.clone());

        migration_contents
    }
}

impl Command for Migrate {
    fn execute(&self, arguments: &Vec<String>, options: &Options) {
        println!("Migrate!!!");
        println!("{}", MIGRATE_PATH);

        let _ = &self.migration_table_contents();
    }

    fn command_name(&self) -> &str {
        "migrate"
    }
}
