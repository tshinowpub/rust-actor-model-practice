use crate::command::Command;
use crate::clients::dynamodb_client;
use crate::clients::dynamodb_client::DynamodbClient;

const MIGRATE_PATH: &str = "./migrations";

pub struct Migrate {
    dynamodb_client: DynamodbClient
}

impl Migrate {
    pub fn new(dynamodb_client: DynamodbClient) -> Self {
        Self {
            dynamodb_client
        }
    }
}

impl Command for Migrate {
    fn execute(&self, arguments: &Vec<String>, options: &Vec<String>) {
        println!("Migrate!!!");


        println!("{}", MIGRATE_PATH);
    }

    fn command_name(&self) -> &str {
        "migrate"
    }
}
