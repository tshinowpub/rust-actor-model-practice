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
    --event-source arn:aws:dynamodb:ap-northeast-1:000000000000:table/Messages/stream/2023-03-11T23:36:41.626  \
    --batch-size 1 \
    --starting-position TRIM_HORIZON
```

## References
- [Custom AWS Lambda runtimes](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.KCLAdapter.html)
- [Using Kinesis Data Streams to capture changes to DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/kds.html)
- [Using the DynamoDB Streams Kinesis adapter to process stream records](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.KCLAdapter.html)
- [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)