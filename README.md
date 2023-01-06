# OverView
practice rust cqrs.

## Environment building

Install protobuf so that you can compile Proto files according to your environment.

[protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf)

## Execution method

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

First you need to set ``AWS_ACCESS_KEY_ID`` and ``AWS_SECRET_ACCESS_KEY``.

#### Mac/Linux
```shell
$ export AWS_ACCESS_KEY_ID={Your AWS_ACCESS_KEY_ID}
$ export AWS_SECRET_ACCESS_KEY={Your AWS_SECRET_ACCESS_KEY}
```

#### Windows
```shell
$ set AWS_ACCESS_KEY_ID={Your AWS_ACCESS_KEY_ID}
$ set AWS_SECRET_ACCESS_KEY={Your AWS_SECRET_ACCESS_KEY}
```

Once you are done, do the following.

#### Execute

```shell
$ cd path/to/rust-actor-model-practice
$ cargo run --package migrator
```
