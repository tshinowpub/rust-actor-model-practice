use anyhow::anyhow;
use aws_sdk_dynamodb::{Credentials, Endpoint, Region};
use aws_sdk_dynamodb::error::DescribeTableError;
use aws_sdk_dynamodb::error::DescribeTableErrorKind::ResourceNotFoundException;
use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, ProvisionedThroughput};
use aws_sdk_dynamodb::output::CreateTableOutput;
use aws_sdk_dynamodb::types::SdkError::ServiceError;
use http::Uri;

use crate::clients::dynamodb_client_factory::DynamodbClientFactory;
use crate::command::query::create_table::CreateTableQuery;

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

        let create_table_response = self.client
            .create_table()
            .table_name(table_name)
            .set_attribute_definitions(Some(vec_attribute_definitions))
            .set_key_schema(Some(vec_key_schemas))
            .provisioned_throughput(provisioned_throughput)
            .send()
            .await;

        Ok(create_table_response?)
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

    pub fn factory() -> aws_sdk_dynamodb::Client {
        let endpoint = Endpoint::immutable(Uri::from_static("http://localhost:8000"));

        let dynamodb_local_config = aws_sdk_dynamodb::Config::builder()
            .region(Region::new("ap-northeast-1"))
            .endpoint_resolver(endpoint)
            .credentials_provider(Credentials::new("test", "test", None, None, "default"))
            .build();

        aws_sdk_dynamodb::Client::from_conf(dynamodb_local_config)
    }
}
