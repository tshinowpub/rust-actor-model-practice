use std::path::PathBuf;
use anyhow::{anyhow, Context};
use aws_sdk_dynamodb::{Credentials, Endpoint, Region};
use aws_sdk_dynamodb::error::DescribeTableError;
use aws_sdk_dynamodb::error::DescribeTableErrorKind::ResourceNotFoundException;
use aws_sdk_dynamodb::model::{AttributeDefinition, AttributeValue, KeySchemaElement, ProvisionedThroughput, StreamSpecification};
use aws_sdk_dynamodb::output::{CreateTableOutput, DeleteTableOutput, GetItemOutput, ListTablesOutput, PutItemOutput};
use aws_sdk_dynamodb::types::SdkError::ServiceError;
use chrono::Utc;
use http::Uri;

use crate::query::create_table::CreateTableQuery;
use crate::query::delete_table::DeleteTableQuery;
use crate::query::get_item::GetItemQuery;
use crate::query::list_tables::ListTablesQuery;

#[derive(Debug, PartialEq)]
pub enum ExistsTableResultType {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
pub struct Client {
    client: aws_sdk_dynamodb::Client,
}

impl Client {
    pub fn new () -> Client {
        Self { client: Client::factory() }
    }

    pub async fn create_table(self, table_name: &str, query: &CreateTableQuery) -> anyhow::Result<CreateTableOutput> {
        println!("---Called create_table---");

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

        let stream_specification = StreamSpecification::builder()
            .stream_enabled(query.stream_specification().stream_enabled())
            .set_stream_view_type(query.stream_specification().stream_view_type())
            .build();

        let create_table_response = self.client
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

    pub async fn delete_table(self, query: &DeleteTableQuery) -> anyhow::Result<DeleteTableOutput> {
        let delete_table_response = self.client
            .delete_table()
            .table_name(query.table_name())
            .send()
            .await;

        Ok(delete_table_response.context(format!("Failed delete_table. Table name: {:?}", query.table_name()))?)
    }

    pub async fn get_item(self, query: &GetItemQuery) -> anyhow::Result<GetItemOutput> {
        let query_response = self.client
            .get_item()
            .table_name(query.table_name())
            .key(query.key().name(), query.key().value().clone())
            .consistent_read(*query.consistent_read())
            .send()
            .await;

        Ok(query_response.context(format!("Failed get_item. Table name: {}", query.table_name()))?)
    }

    pub async fn list_tables(self, _query: &ListTablesQuery) -> anyhow::Result<ListTablesOutput> {
        let list_tables_response = self.client
            .list_tables()
            .send()
            .await;

        Ok(list_tables_response
            .map_err(|error| anyhow!(format!("Failed list tables. Error: {}", error.to_string())))?)
    }

    pub async fn exists_table(self, table_name: &str) -> anyhow::Result<ExistsTableResultType> {
        let describe_table_response = self.client
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

    pub async fn add_migration_record(self, file: &PathBuf) -> anyhow::Result<PutItemOutput> {
        let file_name = AttributeValue::S(file
            .file_name()
            .context(format!("Cannot get filename from PathBuf. {:?}", file))?
            .to_string_lossy()
            .to_string()
        );
        let executed_at = AttributeValue::S(Utc::now().to_string());

        let request = self.client
            .put_item()
            .table_name("migrations")
            .item("FileName", file_name)
            .item("ExecutedAt", executed_at);

        let response = request.send().await.context("Failed put item.")?;

        Ok(response)
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
