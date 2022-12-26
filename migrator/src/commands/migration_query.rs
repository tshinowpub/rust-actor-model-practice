use aws_sdk_dynamodb::model::KeyType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    read_capacity_units: i64,
    #[serde(rename = "WriteCapacityUnits")]
    write_capacity_units: i64
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KeySchema {
    #[serde(rename = "AttributeName")]
    attribute_name: String,
    #[serde(rename = "KeyType")]
    key_type: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MigrationQuery {
    #[serde(rename = "TableName")]
    table_name: String,
    #[serde(rename = "KeySchema")]
    key_schemas: Vec<KeySchema>,
    #[serde(rename = "ProvisionedThroughput")]
    provisioned_throughput: ProvisionedThroughput,
}

impl MigrationQuery {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn key_schemas(&self) -> &Vec<KeySchema> {
        &self.key_schemas
    }

    pub fn provisioned_throughput(&self) -> &ProvisionedThroughput {
        &self.provisioned_throughput
    }
}

impl KeySchema {
    pub fn attribute_name(&self) -> &str {
        &self.attribute_name
    }

    pub fn key_type(&self) -> KeyType {
        KeyType::Hash
    }
}

impl ProvisionedThroughput {
    pub fn read_capacity_units(&self) -> &i64 {
        &self.read_capacity_units
    }

    pub fn write_capacity_units(&self) -> &i64 {
        &self.write_capacity_units
    }
}
