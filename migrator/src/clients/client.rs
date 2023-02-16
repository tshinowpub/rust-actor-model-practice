use aws_sdk_dynamodb::model::{AttributeDefinition, KeySchemaElement, ProvisionedThroughput};
use aws_sdk_dynamodb::output::CreateTableOutput;
use crate::clients::dynamodb_client_factory::DynamodbClientFactory;

use crate::command::query::create_table::CreateTableQuery;

#[derive(Default, Debug, Clone)]
pub struct Client {
}

impl Client {
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

        let create_table_response = DynamodbClientFactory::factory()
            .create_table()
            .table_name(table_name)
            .set_attribute_definitions(Some(vec_attribute_definitions))
            .set_key_schema(Some(vec_key_schemas))
            .provisioned_throughput(provisioned_throughput)
            .send()
            .await;

        Ok(create_table_response?)
    }
}
