use anyhow::Result;
use crate::model::message::message::Message;

pub trait MessageRepository {
    fn add(&self, message: &Message) -> Result<()>;
}
