# read-model-updater

## Why using AWS Lambda ?

The Kinesis data stream records might appear in a different sequence than the item changes occurred.

See [here(Using Kinesis Data Streams to capture changes to DynamoDB)
](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/kds.html).

This code created by [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime).

## Build

```shell
cargo lambda build --release --x86-64 --output-format zip
```

## Usage

```shell
aws iam create-role --role-name ReadModelUpdater \
    --endpoint-url http://localhost:4566 \
    --path "/service-role/" \
    --assume-role-policy-document file://read-model-updater/trust-relationship.json
    
aws iam put-role-policy --role-name ReadModelUpdater \
    --endpoint-url http://localhost:4566 \
    --policy-name ReadModelUpdaterPolicy \
    --policy-document file://read-model-updater/role-policy.json
    
aws lambda create-function \
    --endpoint-url http://localhost:4566 \
    --region ap-northeast-1 \
    --function-name ReadModelUpdater \
    --zip-file fileb://target/lambda/read-model-updater/bootstrap.zip \
    --role "arn:aws:iam::000000000000:roleC:/Program Files/Git/service-role/ReadModelUpdater" \
    --handler ReadModelUpdater.handler \
    --timeout 5 \
    --runtime provided.al2
    
aws lambda create-event-source-mapping \
    --endpoint-url http://localhost:4566 \
    --region ap-northeast-1 \
    --function-name ReadModelUpdater \
    --event-source arn:aws:dynamodb:ap-northeast-1:000000000000:table/Messages/stream/2023-03-15T23:52:27.487  \
    --batch-size 1 \
    --starting-position TRIM_HORIZON
```

## DynamoDB Stream Event Sample

```shell
 [read-model-updater\src\main.rs:39] "{}" = "{}"
 [read-model-updater\src\main.rs:39] event = LambdaEvent {
     payload: Event {
         records: [
             EventRecord {
                 aws_region: "ap-northeast-1",
                 change: StreamRecord {
                     approximate_creation_date_time: 2023-03-15T23:57:36Z,
                     keys: {
                         "message_id": String(
                             "0b4eb05a-6d46-4326-96d6-bcaa4bc2035f",
                         ),
                     },
                     new_image: {
                         "channel_id": String(
                             "1",
                         ),
                         "message_id": String(
                             "0b4eb05a-6d46-4326-96d6-bcaa4bc2035f",
                         ),
                         "message_type": String(
                             "post",
                         ),
                         "posted_at": String(
                             "2023-03-15 23:57:35.353449500 UTC",
                         ),
                         "message": String(
                             "テストテスト",
                         ),
                         "account_id": String(
                             "111",
                         ),
                     },
                     old_image: {},
                     sequence_number: Some(
                         "49638917533186157979722089949123779376369619237901172738",
                     ),
                     size_bytes: 261,
                     stream_view_type: Some(
                         NewAndOldImages,
                     ),
                 },
                 event_id: "c633d67d",
                 event_name: "INSERT",
                 event_source: Some(
                     "aws:dynamodb",
                 ),
                 event_version: Some(
                     "1.0",
                 ),
                 event_source_arn: Some(
                     "arn:aws:dynamodb:ap-northeast-1:000000000000:table/Messages/stream/2023-03-15T23:52:27.487",
                 ),
                 user_identity: None,
                 record_format: None,
                 table_name: None,
             },
         ],
     },
     context: Context {
         request_id: "00aa8d0d-3d54-477f-9604-ebb291401f0a",
         deadline: 1678924664282,
         invoked_function_arn: "arn:aws:lambda:ap-northeast-1:000000000000:function:ReadModelUpdater",
         xray_trace_id: Some(
             "Root=1-53cfd31b-192638fa13e39d2c2bcea001;Parent=365fb4b15f2e3987;Sampled=0",
         ),
         client_context: None,
         identity: None,
         env_config: Config {
             function_name: "ReadModelUpdater",
             memory: 128,
             version: "$LATEST",
             log_stream: "2023/03/15/[$LATEST]f9ff7fbe85e9c8a5ff3177b25d5096f0",
             log_group: "/aws/lambda/ReadModelUpdater",
         },
     },
 }
```

## References
- [Custom AWS Lambda runtimes](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.KCLAdapter.html)
- [Using Kinesis Data Streams to capture changes to DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/kds.html)
- [Using the DynamoDB Streams Kinesis adapter to process stream records](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.KCLAdapter.html)
- [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)