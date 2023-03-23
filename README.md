# OverView

![Workflow Status](https://github.com/tshinowpub/rust-cqrs-practice/workflows/test/badge.svg)

practice rust cqrs.

- https://aws.amazon.com/jp/blogs/news/build-a-cqrs-event-store-with-amazon-dynamodb/

## Environment building

Install protobuf so that you can compile Proto files according to your environment.

[protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf)

## Usage

### migration for RDB
Migration uses [golang-migrate](https://github.com/golang-migrate/migrate).

```shell
$ docker run -v {{ migration dir }}:/migrations --network host migrate/migrate \
    -path=/migrations/ -database mysql://{user}:{password}@localhost:3306/rust up
    
## Widows + Git Bash example
// CLI
migrate -source file://$(pwd -W)/resources/rds-reader/migrations -database "mysql://rust:rust@tcp(localhost:3306)/rust" up
migrate -source file://$(pwd -W)/resources/rds-reader/migrations -database "mysql://rust:rust@tcp(localhost:3306)/rust" down

// Docker
docker run -v $(pwd)/migrations:/migrations --network host migrate/migrate \
  -path=/migrations/ -database "mysql://rust:rust@tcp(localhost:3306)/rust" up
docker run -v $(pwd)/migrations:/migrations --network host migrate/migrate \
  -path=/migrations/ -database "mysql://rust:rust@tcp(localhost:3306)/rust" down
```

### read-http-api
```shell
$ cd path/to/rust-actor-model-practice
$ cargo run --package read-http-api
```

### write-grpc
```shell
$ cd path/to/rust-actor-model-practice
$ cargo run --package write-grpc
```

### migrator

- Mac/Linux
```shell
$ export ENV=develop
```

- Windows
```shell
$ set ENV=develop
```

```shell
$ cd path/to/rust-actor-model-practice
$ cargo run --package migrator
```

Once you are done, do the following.
