use aws_sdk_dynamodb::error::ListTablesError;
use aws_sdk_dynamodb::types::SdkError;
use crate::clients::dynamodb_client_factory::DynamodbClientFactory;
use crate::command::{ExitCode, Output};

#[derive(Debug, Copy, Clone)]
pub struct Reset {}

impl Reset {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(self) -> Output {
        return match self.find_table_names().await {
            Ok(table_names) => {
                println!("Table names:");
                println!("--------------------------------------");
                for table_name in table_names {
                    println!("{}", table_name)
                }
                println!("--------------------------------------");

                Output::new(ExitCode::SUCCEED, "Succeeded".to_string())
            },
            Err(error) => Output::new(ExitCode::FAILED, format!("Reset failed. : {}", error)),
        }
    }

    async fn find_table_names(self) -> Result<Vec<String>, SdkError<ListTablesError>> {
        return match DynamodbClientFactory::factory().list_tables().send().await {
            Ok(output) => {
                //@todo When there are more than 100 tables.
                Ok(output.table_names().unwrap_or(&[]).to_vec())
            },
            Err(aaa) => Err(aaa),
        }
    }
}
