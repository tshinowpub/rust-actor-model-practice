# OverView

### Run
Use postman.

- host: localhost:50051
- Message/AddMessage

```json
{
    "channel_id": "1",
    "account_id": "11111",
    "message": "hogesshoge"
}
```

## References
- [Amazon DynamoDB を使った CQRS イベントストアの構築](https://aws.amazon.com/jp/blogs/news/build-a-cqrs-event-store-with-amazon-dynamodb/)
- [DynamoDB local usage notes](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.UsageNotes.html)