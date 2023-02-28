use anyhow::{Context, Result};
use tonic::transport::Server;

pub mod adapter;
pub mod usecase;

use crate::adapter::controllers::add_message_controller::message::message_server::MessageServer;
use crate::usecase::add_message::AddMessageUsecase;
use adapter::controllers::add_message_controller::AddMessage;

/**
 * see https://qiita.com/ryuma017/items/1f31f5441ed5df80f1cc
 * https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/hello_tokio
 */
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051"
        .parse()
        .context("Failed start grpc server. Error: parse error.")?;

    let message = AddMessage::new(AddMessageUsecase::default());

    println!("MessageServer listening on {}", addr);

    Server::builder()
        .add_service(MessageServer::new(message))
        .serve(addr)
        .await
        .context("Failed start grpc server.")?;

    Ok(())
}
