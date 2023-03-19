#[derive(Debug)]
pub struct MessageId {
    value: String
}

impl MessageId {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}
