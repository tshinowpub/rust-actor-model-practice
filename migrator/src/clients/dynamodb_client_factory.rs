use aws_sdk_dynamodb::{Client, Credentials, Endpoint, Region};
use http::Uri;

#[derive(Debug)]
pub struct DynamodbClientFactory {}

impl DynamodbClientFactory {
    pub fn factory() -> Client {
        let endpoint = Endpoint::immutable(Uri::from_static("http://localhost:8000"));

        let dynamodb_local_config = aws_sdk_dynamodb::Config::builder()
            .region(Region::new("ap-northeast-1"))
            .endpoint_resolver(endpoint)
            .credentials_provider(Credentials::new("test", "test", None, None, "default"))
            .build();

        Client::from_conf(dynamodb_local_config)
    }
}
