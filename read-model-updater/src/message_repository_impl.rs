use domain::model::message::message::Message;
use domain::model::message::message_repository::MessageRepository;

pub struct MessageRepositoryImpl {
}

impl MessageRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl MessageRepository for MessageRepositoryImpl {
    fn add(&self, message: &Message) -> anyhow::Result<()> {
        println!("Message: {:?}.", message);

        Ok(())
    }
}
