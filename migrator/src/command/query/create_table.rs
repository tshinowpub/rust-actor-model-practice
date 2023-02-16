use serde::{Serialize, Deserialize};
use crate::command::query::dynamodb_query::{AttributeDefinition, KeySchema, ProvisionedThroughput};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateTableQuery {
    #[serde(rename = "TableName")]
    table_name: String,
    #[serde(rename = "AttributeDefinitions")]
    attribute_definitions: Vec<AttributeDefinition>,
    #[serde(rename = "KeySchema")]
    key_schemas: Vec<KeySchema>,
    #[serde(rename = "ProvisionedThroughput")]
    provisioned_throughput: ProvisionedThroughput,
}

impl CreateTableQuery {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn key_schemas(&self) -> &Vec<KeySchema> {
        &self.key_schemas
    }

    pub fn attribute_definitions(&self) -> &Vec<AttributeDefinition> {
        &self.attribute_definitions
    }

    pub fn provisioned_throughput(&self) -> &ProvisionedThroughput {
        &self.provisioned_throughput
    }
}
