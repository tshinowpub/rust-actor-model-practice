use aws_sdk_dynamodb::error::{DeleteTableError, ListTablesError};
use aws_sdk_dynamodb::types::SdkError;
use tokio_stream::StreamExt;
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
                println!("Executing...");
                println!("--------------------------------------");
                let mut stream = tokio_stream::iter(table_names);
                while let Some(name) = stream.next().await {
                    let result = self.remove_table(&name).await;
                    match result {
                        Ok(_) => println!("Table {} was deleted...", name),
                        Err(error) => return Output::new(ExitCode::FAILED, format!("Remove table failed. {}", error.to_string())),
                    }
                }
                println!("--------------------------------------");

                Output::new(ExitCode::SUCCEED, "Remove table was succeeded.".to_string())
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
            Err(error) => Err(error),
        }
    }

    async fn remove_table(self, table_name: &str) -> Result<(), SdkError<DeleteTableError>> {
        return match DynamodbClientFactory::factory().delete_table().table_name(table_name).send().await {
            Ok(_output) => Ok(()),
            Err(error) => Err(error),
        }
    }
}
