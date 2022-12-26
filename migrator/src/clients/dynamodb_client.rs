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

    pub fn create_table(&self) {
        println!("Method create_table was called!!!")
    }
}
