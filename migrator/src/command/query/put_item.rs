use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PutItemQuery {
    #[serde(rename = "TableName")]
    table_name: String,
}
