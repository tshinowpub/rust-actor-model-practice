#[derive(Debug)]
pub struct AccountId {
    value: u32
}

impl AccountId {
    pub fn new(value: impl Into<u32>) -> Self {
        Self { value: value.into() }
    }

    pub fn value(&self) -> &u32 {
        &self.value
    }
}
