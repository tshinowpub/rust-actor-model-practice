use anyhow::{anyhow, Context};
use aws_sdk_dynamodb::error::DescribeTableError;
use aws_sdk_dynamodb::error::DescribeTableErrorKind::ResourceNotFoundException;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, ProvisionedThroughput, StreamSpecification,
};
use aws_sdk_dynamodb::output::{
    CreateTableOutput, DeleteTableOutput, GetItemOutput, ListTablesOutput,
    PutItemOutput,
};
use aws_sdk_dynamodb::types::SdkError::ServiceError;
use aws_sdk_dynamodb::{Credentials, Endpoint, Region};
use http::Uri;

use crate::query::create_table::CreateTableQuery;
use crate::query::delete_table::DeleteTableQuery;
use crate::query::get_item::GetItemQuery;
use crate::query::list_tables::ListTablesQuery;
use crate::query::put_item::PutItemQuery;

#[derive(Debug, PartialEq)]
pub enum ExistsTableResultType {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
pub struct Client {
    client: aws_sdk_dynamodb::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Client {
        Self {
            client: Client::factory(),
        }
    }

    pub async fn create_table(
        &self,
        table_name: &str,
        query: &CreateTableQuery,
    ) -> anyhow::Result<CreateTableOutput> {
        println!("---Called create_table---");

        println!("TableName: {}", table_name);

        let vec_attribute_definitions = query
            .attribute_definitions()
            .to_vec()
            .iter()
            .map(|attribute_definition| {
                (AttributeDefinition::builder()
                    .attribute_name(attribute_definition.attribute_name()))
                .attribute_type(attribute_definition.attribute_type())
                .build()
            })
            .collect::<Vec<_>>();

        let vec_key_schemas = query
            .key_schemas()
            .to_vec()
            .iter()
            .map(|key_schema| {
                (KeySchemaElement::builder().attribute_name(key_schema.attribute_name()))
                    .key_type(key_schema.key_type())
                    .build()
            })
            .collect::<Vec<_>>();

        let input_provisioned_throughput = query.provisioned_throughput();

        let provisioned_throughput = ProvisionedThroughput::builder()
            .read_capacity_units(*input_provisioned_throughput.read_capacity_units())
            .write_capacity_units(*input_provisioned_throughput.write_capacity_units())
            .build();

        let stream_specification = StreamSpecification::builder()
            .stream_enabled(query.stream_specification().stream_enabled())
            .set_stream_view_type(query.stream_specification().stream_view_type())
            .build();

        let create_table_response = self
            .client
            .create_table()
            .table_name(table_name)
            .set_attribute_definitions(Some(vec_attribute_definitions))
            .set_key_schema(Some(vec_key_schemas))
            .provisioned_throughput(provisioned_throughput)
            .stream_specification(stream_specification)
            .send()
            .await;

        Ok(create_table_response?)
    }

    pub async fn delete_table(&self, query: &DeleteTableQuery) -> anyhow::Result<DeleteTableOutput> {
        let delete_table_response = self
            .client
            .delete_table()
            .table_name(query.table_name())
            .send()
            .await;

        delete_table_response.context(format!(
            "Failed delete_table. Table name: {:?}",
            query.table_name()
        ))
    }

    pub async fn get_item(&self, query: &GetItemQuery) -> anyhow::Result<GetItemOutput> {
        let query_response = self
            .client
            .get_item()
            .table_name(query.table_name())
            .key(query.key().name(), query.key().value().clone())
            .consistent_read(*query.consistent_read())
            .send()
            .await;

        query_response.context(format!(
            "Failed get_item. Table name: {}",
            query.table_name()
        ))
    }

    pub async fn put_item(&self, query: PutItemQuery) -> anyhow::Result<PutItemOutput> {
        let put_item_response = self
            .client
            .put_item()
            .table_name(query.table_name())
            .set_item(Some(query.items()))
            .return_values(query.return_values())
            .condition_expression("attribute_not_exists(message_id)")
            .send()
            .await;

        Ok(put_item_response?)
    }

    pub async fn list_tables(&self, _query: &ListTablesQuery) -> anyhow::Result<ListTablesOutput> {
        self.client
            .list_tables()
            .send()
            .await
            .map_err(|error| {
                anyhow!(format!("Failed list tables. Error: {}", error))
            })
    }

    pub async fn exists_table(&self, table_name: &str) -> anyhow::Result<ExistsTableResultType> {
        let describe_table_response = self
            .client
            .describe_table()
            .table_name(table_name)
            .send()
            .await;

        match describe_table_response {
            Ok(_) => Ok(ExistsTableResultType::Found),
            Err(ServiceError {
                err:
                    DescribeTableError {
                        kind: ResourceNotFoundException(_),
                        ..
                    },
                raw: _,
            }) => Ok(ExistsTableResultType::NotFound),
            Err(error) => Err(anyhow!(error.to_string())),
        }
    }

    fn factory() -> aws_sdk_dynamodb::Client {
        let endpoint = Endpoint::immutable(Uri::from_static("http://localhost:8000"));

        let dynamodb_local_config = aws_sdk_dynamodb::Config::builder()
            .region(Region::new("ap-northeast-1"))
            .endpoint_resolver(endpoint)
            .credentials_provider(Credentials::new("test", "test", None, None, "default"))
            .build();

        aws_sdk_dynamodb::Client::from_conf(dynamodb_local_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{Client, ExistsTableResultType};

    #[tokio::test]
    async fn test_exists_table() {
        let exists_table_result_type = Client::new()
            .exists_table("migrations")
            .await
            .unwrap();

        assert_eq!(ExistsTableResultType::Found, exists_table_result_type)
    }

    #[tokio::test]
    async fn test_exists_table_not_found() {
        let exists_table_result_type = Client::new()
            .exists_table("test")
            .await
            .unwrap();

        assert_eq!(ExistsTableResultType::NotFound, exists_table_result_type)
    }
}
