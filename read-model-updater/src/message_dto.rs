use std::num::ParseIntError;
use anyhow::{anyhow, Context, Result};
use aws_lambda_events::dynamodb::attributes::AttributeValue;
use aws_lambda_events::dynamodb::EventRecord;
use chrono::{DateTime, Utc};

pub struct MessageDto {
    message_write_id: String,
    account_id: u32,
    channel_id: u32,
    message: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>
}

impl MessageDto {
    pub fn from_event(event: &EventRecord) -> Result<MessageDto> {
        let message_write_id = event.change.new_image
            .get("message_id")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => Ok(value.to_string()),
                _ => Err(anyhow!("MessageWriteId attribute value {:?} was not supported.", attribute_value))
            })
            .context(format!("{:?} was not message_id.", event.change.new_image.get("message_id")))??;

        let account_id = event.change.new_image
            .get("account_id")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => value.parse::<u32>().context(""),
                _ => Err(anyhow!("AccountId attribute value {:?} was not supported.", attribute_value))
            })
            .context(format!("{:?} was not account_id.", event.change.new_image.get("account_id")))??;

        let channel_id = event.change.new_image
            .get("channel_id")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => value.parse::<u32>().context(""),
                _ => Err(anyhow!("ChannelId attribute value {:?} was not supported.", attribute_value))
            })
            .context(format!("{:?} was not channel_id.", event.change.new_image.get("channel_id")))??;

        let message = event.change.new_image
            .get("message")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => Ok(value.to_string()),
                _ => Err(anyhow!("Message attribute value {:?} was not supported.", attribute_value))
            })
            .context(format!("{:?} was not message.", event.change.new_image.get("message")))??;

        Ok(Self {
            message_write_id,
            account_id,
            channel_id,
            message,
            created_at: None,
            updated_at: None,
            deleted_at: None
        })
    }

    pub(crate) fn message_write_id(&self) -> &String {
        &self.message_write_id
    }

    pub(crate) fn account_id(&self) -> &u32 {
        &self.account_id
    }

    pub(crate) fn channel_id(&self) -> &u32 {
        &self.channel_id
    }

    pub(crate) fn message(&self) -> &String {
        &self.message
    }
}
