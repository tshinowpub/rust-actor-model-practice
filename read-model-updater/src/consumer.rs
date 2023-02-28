use anyhow::{anyhow, Result};

#[derive(Debug, Default)]
pub struct Consumer {}

impl Consumer {
    pub fn consume(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::consumer::Consumer;

    #[test]
    fn test_consumer() -> anyhow::Result<()> {
        let consumer = Consumer::default();

        consumer.consume()
    }
}
