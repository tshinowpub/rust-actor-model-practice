use aws_sdk_dynamodb::model::{KeyType, ScalarAttributeType};
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    attribute_name: String,
    #[serde(rename = "AttributeType")]
    attribute_type: String
}

impl KeySchema {
    pub fn attribute_name(&self) -> &str {
        &self.attribute_name
    }

    pub fn key_type(&self) -> KeyType {
        match &self.key_type {
            _ if &self.key_type.to_string() == "HASH" => KeyType::Hash,
            _ if &self.key_type.to_string() == "RANGE" => KeyType::Range,
            _ => {
                let name = &self.key_type.to_string();

                KeyType::Unknown(name.clone())
            }
        }
    }
}


impl AttributeDefinition {
    pub fn attribute_name(&self) -> &str {
        &self.attribute_name
    }

    pub fn attribute_type(&self) -> ScalarAttributeType {
        match &self.attribute_type {
            _ if &self.attribute_type.to_string() == "S" => ScalarAttributeType::S,
            _ if &self.attribute_type.to_string() == "N" => ScalarAttributeType::N,
            _ if &self.attribute_type.to_string() == "B" => ScalarAttributeType::B,
            _ => {
                let name = &self.attribute_type.to_string();

                ScalarAttributeType::Unknown(name.clone())
            }
        }
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
