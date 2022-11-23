use tonic::{Request, Response, Status};

use crate::adapter::controllers::add_message_controller::message::message_server::Message;
use message::{MessageReply, MessageRequest};

pub mod message {
    tonic::include_proto!("message");
}

#[derive(Default)]
pub struct AddMessage {}

#[tonic::async_trait]
impl Message for AddMessage {
    async fn add_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = message::MessageReply {
            message: format!("Hello {}!", request.into_inner().message),
        };
        Ok(Response::new(reply))
    }
}
