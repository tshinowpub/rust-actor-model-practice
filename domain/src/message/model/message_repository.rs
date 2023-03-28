use anyhow::Result;
use crate::message::model::message::Message;

pub trait MessageRepository {
    fn add(&self, message: &Message) -> Result<()>;
}
