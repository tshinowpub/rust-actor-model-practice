use aws_sdk_dynamodb::model::{KeyType, ScalarAttributeType, StreamViewType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    read_capacity_units: i64,
    #[serde(rename = "WriteCapacityUnits")]
    write_capacity_units: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KeySchema {
    #[serde(rename = "AttributeName")]
    attribute_name: String,
    #[serde(rename = "KeyType")]
    key_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    attribute_name: String,
    #[serde(rename = "AttributeType")]
    attribute_type: String,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamSpecification {
    #[serde(rename = "StreamEnabled")]
    stream_enabled: Option<bool>,
    #[serde(rename = "StreamViewType")]
    stream_view_type: Option<String>,
}

impl StreamSpecification {
    pub fn stream_enabled(&self) -> bool {
        &self.stream_enabled == &Some(true)
    }

    pub fn stream_view_type(&self) -> Option<StreamViewType> {
        match &self.stream_view_type {
            _ if &self.stream_enabled == &None || &self.stream_enabled == &Some(false) => None,
            None => None,
            Some(stream_view_type) if stream_view_type.to_string() == "KeysOnly" => {
                Some(StreamViewType::KeysOnly)
            }
            Some(stream_view_type) if stream_view_type.to_string() == "NewAndOldImages" => {
                Some(StreamViewType::NewAndOldImages)
            }
            Some(stream_view_type) if stream_view_type.to_string() == "NewImage" => {
                Some(StreamViewType::NewImage)
            }
            Some(stream_view_type) if stream_view_type.to_string() == "OldImage" => {
                Some(StreamViewType::OldImage)
            }
            Some(stream_view_type) => Some(StreamViewType::Unknown(stream_view_type.to_string())),
        }
    }
}
