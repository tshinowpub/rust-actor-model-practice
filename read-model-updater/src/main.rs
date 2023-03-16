use aws_lambda_events::dynamodb::EventRecord;
use aws_lambda_events::event::dynamodb::Event;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

#[derive(Serialize)]
struct Body {
    message: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request
    let resp = Response {
        statusCode: 200,
        body: "Hello World!".to_string(),
    };

    dbg!("{}", &event);

    let records = event
        .payload
        .records
        .iter()
        .for_each(|event_records| push_to_read_model(event_records));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

fn push_to_read_model(record: &EventRecord) {
    match record.event_name.as_str() {
        "INSERT" => {
            println!("Called Lambda event: {:?}.", record.event_name)
        },
        _ => {
            println!("Called Lambda event: {:?}.", record.event_name)
        },
    };
}

#[cfg(test)]
mod tests {
    use std::process::exit;
    use aws_lambda_events::dynamodb::Event;
    use aws_lambda_events::serde_json;
    use lambda_runtime::LambdaEvent;
    use crate::push_to_read_model;

    #[test]
    fn test_function_push_to_read_model() {
        let data = include_bytes!("../tests/fixtures/example-dynamodb-event.json");
        let mut event: Event = serde_json::from_slice(data).expect("Cannot parse json.");

        let event_record = event.records.pop().unwrap();

        push_to_read_model(&event_record);

        exit(0);
    }
}
