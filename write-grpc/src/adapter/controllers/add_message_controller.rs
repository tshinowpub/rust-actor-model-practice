use aws_sdk_dynamodb::model::AttributeValue;
use tonic::{Code, Request, Response, Status};
use message::{MessageReply, MessageRequest};

use crate::adapter::controllers::add_message_controller::message::message_server::Message;
use crate::usecase::add_message::AddMessageUsecase;

pub mod message {
    tonic::include_proto!("message");
}

pub struct AddMessage {
    usecase: AddMessageUsecase
}

impl AddMessage {
    pub fn new(usecase: AddMessageUsecase) -> Self {
        Self {usecase}
    }
}

#[tonic::async_trait]
impl Message for AddMessage {
    async fn add_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageReply>, Status> {
        println!("Got a request from {:?}", &request.remote_addr());

        let output = self
            .usecase
            .run(request)
            .await
            .map_err(|error| Status::new(Code::Unavailable, format!("Failed putItem. Error: {}", error.to_string())))?;

        let account_id = output
            .attributes
            .ok_or(Status::new(Code::Unavailable, "Failed putItem."))?
            .get("AccountId")
            .cloned()
            .unwrap_or(AttributeValue::S("".to_string()));

        let reply = message::MessageReply {
            message: format!("AccountId: {:?}", account_id),
        };

        dbg!(&reply);

        Ok(Response::new(reply))
    }
}
