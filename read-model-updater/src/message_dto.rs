use anyhow::{anyhow, Context, Result};
use aws_lambda_events::dynamodb::attributes::AttributeValue;
use aws_lambda_events::dynamodb::EventRecord;
use chrono::{DateTime, NaiveDateTime, Utc};

pub struct MessageDto {
    message_write_id: String,
    account_id: u32,
    channel_id: u32,
    message: String,
    #[allow(dead_code)]
    created_at: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    updated_at: Option<DateTime<Utc>>,
    #[allow(dead_code)]
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
            .context(format!("MessageWriteId was not found. Actual value: {:?}", event.change.new_image.get("message_id")))??;

        let account_id = event.change.new_image
            .get("account_id")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => value.parse::<u32>().context(""),
                _ => Err(anyhow!("AccountId attribute value {:?} was not supported.", attribute_value))
            })
            .context(format!("AccountId was not found. Actual value: {:?}", event.change.new_image.get("account_id")))??;

        let channel_id = event.change.new_image
            .get("channel_id")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => value.parse::<u32>().context(""),
                _ => Err(anyhow!("ChannelId attribute value {:?} was not supported.", attribute_value))
            })
            .context(format!("ChannelId was not found. Actual value: {:?}", event.change.new_image.get("channel_id")))??;

        let message = event.change.new_image
            .get("message")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => Ok(value.to_string()),
                _ => Err(anyhow!("Message attribute value {:?} was not supported.", attribute_value))
            })
            .context(format!("Message was not found. Actual value: {:?}", event.change.new_image.get("message")))??;

        let posted_at = event.change.new_image
            .get("posted_at")
            .map(|attribute_value| match attribute_value {
                AttributeValue::String(value) => {
                    let datetime = NaiveDateTime::parse_from_str(value.as_str(), "%Y-%m-%d %H:%M:%S%.9f %Z")
                        .map(|native_datetime| DateTime::<Utc>::from_utc(native_datetime, Utc))?;

                    Ok(datetime)
                },
                _ => Err(anyhow!("PostedAt attribute value {:?} was not supported.", attribute_value)),
            })
            .context(format!("Posted_at was not found. Actual value: {:?}", event.change.new_image.get("posted_at")))??;

        Ok(Self {
            message_write_id,
            account_id,
            channel_id,
            message,
            created_at: Some(posted_at),
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

    pub(crate) fn created_at(&self) -> &Option<DateTime<Utc>> {
        &self.created_at
    }
}
