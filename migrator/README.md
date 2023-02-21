# OverView
DynamoDB Migrator. 

Create a DynamoDB table based on the json file in the specified directory.

## Migration Filename Format

```shell
{version}_{title}.{command}.json
{version}_{title}.{command}.json
```

### command

DynamoDB commands.

Currently, supported commands are

- create_table
- delete_table

### Run

```shell
$ export ENV=develop

$ cd migrator/
$ cargo run migrate up
```
