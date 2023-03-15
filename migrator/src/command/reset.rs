use anyhow::{anyhow, Result};
use dynamodb_client::client::Client;
use dynamodb_client::query::delete_table::DeleteTableQuery;
use dynamodb_client::query::list_tables::ListTablesQuery;
use tokio_stream::StreamExt;

use crate::command::{ExitCode, Output};

#[derive(Debug, Clone)]
pub struct Reset {
    client: Client
}

impl Reset {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn execute(self) -> Result<Output> {
        let table_names = self
            .find_table_names()
            .await
            .map_err(|error| anyhow!(format!("Reset failed. : {}", error)))?;

        println!("Executing...");
        println!("--------------------------------------");
        let mut stream = tokio_stream::iter(table_names);
        while let Some(name) = stream.next().await {
            self.delete_table(&name).await.map_err(|error| {
                anyhow!(format!("Remove table failed. : {}", error))
            })?;

            println!("Table {} was deleted...", name);
        }
        println!("--------------------------------------");

        Ok(Output::new(ExitCode::Succeed, "Remove all table was succeeded."))
    }

    async fn find_table_names(&self) -> Result<Vec<String>> {
        let result = self
            .client
            .list_tables(&ListTablesQuery::default())
            .await;

        let table_names = result.map(|output|
            //@todo When there are more than 100 tables.
            output
                .table_names()
                .unwrap_or(&[])
                .to_vec()
        )?;

        Ok(table_names)
    }

    async fn delete_table(&self, table_name: &str) -> Result<()> {
        let query = DeleteTableQuery::new(table_name);

        self.client.delete_table(&query).await?;

        Ok(())
    }
}
