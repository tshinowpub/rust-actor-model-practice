use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct DeleteTableQuery {
    #[serde(rename = "TableName")]
    table_name: String,
}

impl DeleteTableQuery {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn new(table_name: String) -> Self {
        Self { table_name }
    }
}
