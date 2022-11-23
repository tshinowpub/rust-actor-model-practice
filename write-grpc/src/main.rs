use tonic::transport::Server;

mod adapter;

use crate::adapter::controllers::add_message_controller::message::message_server::MessageServer;
use adapter::controllers::add_message_controller::AddMessage;

/**
 * see https://qiita.com/ryuma017/items/1f31f5441ed5df80f1cc
 * https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/hello_tokio
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let message = AddMessage::default();

    println!("MessageServer listening on {}", addr);

    Server::builder()
        .add_service(MessageServer::new(message))
        .serve(addr)
        .await?;

    Ok(())
}
