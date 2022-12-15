use aws_sdk_dynamodb::Client;

#[derive(Debug)]
pub struct DynamodbClient {
    client: Client
}

impl DynamodbClient {
    pub fn new(client: Client) -> Self {
        Self {
            client
        }
    }
}
