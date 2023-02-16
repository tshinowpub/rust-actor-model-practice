use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct DeleteTableQuery {
    #[serde(rename = "TableName")]
    table_name: String,
}
