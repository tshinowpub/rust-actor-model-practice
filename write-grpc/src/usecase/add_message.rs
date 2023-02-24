use std::collections::HashMap;
use anyhow::Result;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::PutItemOutput;
use tonic::Request;
use dynamodb_client::client::Client;
use dynamodb_client::query::put_item::{Items, PutItemQuery};

use crate::adapter::controllers::add_message_controller::message::MessageRequest;

#[derive(Debug, Default, Copy, Clone)]
pub struct AddMessageUsecase {}

impl AddMessageUsecase {
    pub async fn run(self, _request: Request<MessageRequest>) -> Result<PutItemOutput> {
        let mut items: Items = HashMap::new();

        items.insert("AccountId".to_string(), AttributeValue::S("111".to_string()));
        items.insert("Message".to_string(), AttributeValue::S("Test message".to_string()));

        let query = PutItemQuery::new("Messages".to_string(), items);

        Ok(
            Client::new()
                .put_item(query)
                .await?
        )
    }
}
