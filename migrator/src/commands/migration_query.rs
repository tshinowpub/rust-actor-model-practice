use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    read_capacity_units: u16,
    #[serde(rename = "WriteCapacityUnits")]
    write_capacity_units: u16
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
    key_schema: Vec<KeySchema>,
    #[serde(rename = "ProvisionedThroughput")]
    provisioned_throughput: ProvisionedThroughput,
}

impl MigrationQuery {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}
