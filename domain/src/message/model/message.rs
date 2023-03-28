use crate::message::model::message_id::MessageId;

#[derive(Debug)]
pub struct Message {
    message_id: MessageId
}

impl Message {
    pub fn new(message_id: MessageId) -> Self {
        Self { message_id }
    }

    pub fn message_id(&self) -> &MessageId {
        &self.message_id
    }
}
