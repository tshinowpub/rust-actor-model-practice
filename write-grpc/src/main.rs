use tonic::{transport::Server, Request, Response, Status};

use crate::message::message_server::{Message, MessageServer};
use message::{MessageReply, MessageRequest};

pub mod message {
    tonic::include_proto!("message");
}

#[derive(Default)]
pub struct MyMessage {}

#[tonic::async_trait]
impl Message for MyMessage {
    async fn add_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = message::MessageReply {
            message: format!("Hello {}!", request.into_inner().value),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let message = MyMessage::default();

    println!("MessageServer listening on {}", addr);

    Server::builder()
        .add_service(MessageServer::new(message))
        .serve(addr)
        .await?;

    Ok(())
}
