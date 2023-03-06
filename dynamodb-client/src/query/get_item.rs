use aws_sdk_dynamodb::model::AttributeValue;

#[derive(Debug)]
pub struct GetItemQuery {
    table_name: String,
    key: Key,
    consistent_read: bool,
}

#[derive(Debug)]
pub struct Key {
    name: String,
    value: AttributeValue,
}

impl GetItemQuery {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn consistent_read(&self) -> &bool {
        &self.consistent_read
    }

    pub fn new(table_name: impl Into<String>, key: Key, consistent_read: bool) -> Self {
        Self {
            table_name: table_name.into(),
            key,
            consistent_read,
        }
    }
}

impl Key {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    pub fn new(name: impl Into<String>, value: AttributeValue) -> Self {
        Self { name: name.into(), value }
    }
}
