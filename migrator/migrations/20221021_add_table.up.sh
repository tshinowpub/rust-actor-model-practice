aws dynamodb create-table ^
    --endpoint http://127.0.0.1:8000 ^
    --table-name Music ^
    --attribute-definitions ^
        AttributeName=Artist,AttributeType=S ^
        AttributeName=SongTitle,AttributeType=S ^
    --key-schema ^
        AttributeName=Artist,KeyType=HASH ^
        AttributeName=SongTitle,KeyType=RANGE ^
    --provisioned-throughput ^
        ReadCapacityUnits=5,WriteCapacityUnits=5 ^
    --table-class STANDARD
