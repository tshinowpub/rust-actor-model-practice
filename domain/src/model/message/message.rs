use crate::model::message::message_id::MessageId;

#[derive(Debug)]
pub struct Message {
    message_id: MessageId
}

impl Message {
    pub fn new(message_id: MessageId) -> Self {
        Self { message_id }
    }
}
