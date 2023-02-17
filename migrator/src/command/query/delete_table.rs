use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct DeleteTableQuery {
    #[serde(rename = "TableName")]
    table_name: String,
}

impl DeleteTableQuery {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}
