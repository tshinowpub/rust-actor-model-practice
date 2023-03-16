use anyhow::Result;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::PutItemOutput;
use chrono::Utc;
use dynamodb_client::client::Client;
use dynamodb_client::query::put_item::{Items, PutItemQuery};
use std::collections::HashMap;
use tonic::transport::Uri;
use uuid::Uuid;

use crate::adapter::controllers::add_message_controller::message::MessageRequest;

#[derive(Debug, Default, Copy, Clone)]
pub struct AddMessageUsecase {}

impl AddMessageUsecase {
    pub async fn run(self, _request: MessageRequest) -> Result<PutItemOutput> {
        let mut items: Items = HashMap::new();

        items.insert("message_id".to_string(), AttributeValue::S(Uuid::new_v4().to_string()));
        items.insert(
            "account_id".to_string(),
            AttributeValue::S("111".to_string()),
        );
        items.insert(
            "channel_id".to_string(),
            AttributeValue::S("1".to_string()),
        );
        items.insert(
            "posted_at".to_string(),
            AttributeValue::S(Utc::now().to_string()),
        );
        items.insert(
            "message".to_string(),
            AttributeValue::S("テストテスト".to_string()),
        );
        items.insert(
            "message_type".to_string(),
            AttributeValue::S("post".to_string()),
        );

        let query = PutItemQuery::new("Messages".to_string(), items, None, None::<String>);

        Client::new(Uri::from_static("http://localhost:4566/")).put_item(query).await
    }
}
