use message::{MessageReply, MessageRequest};
use tonic::{Code, Request, Response, Status};

use crate::adapter::controllers::add_message_controller::message::message_server::Message;
use crate::usecase::add_message::AddMessageUsecase;

pub mod message {
    tonic::include_proto!("message");
}

pub struct AddMessage {
    usecase: AddMessageUsecase,
}

impl AddMessage {
    pub fn new(usecase: AddMessageUsecase) -> Self {
        Self { usecase }
    }
}

#[tonic::async_trait]
impl Message for AddMessage {
    async fn add_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageReply>, Status> {
        println!("Got a request from {:?}", &request.remote_addr());

        let message_request: MessageRequest = request.into_inner();

        let output = self
            .usecase
            .run(message_request.clone())
            .await
            .map_err(|error| {
                Status::new(
                    Code::Unavailable,
                    format!("Failed putItem. Error: {}", error.to_string()),
                )
            })?;

        dbg!(&output);

        let reply = message::MessageReply {
            message: format!("channel_id: {}", &message_request.channel_id),
        };

        dbg!(&reply);

        Ok(Response::new(reply))
    }
}
