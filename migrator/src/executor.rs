use aws_sdk_dynamodb as dynamodb;
use crate::clients::dynamodb_client::DynamodbClient;

use crate::commands::list::List;
use crate::command::Command;
use crate::commands::migrate::Migrate;
use crate::lexers::option_lexer::Options;

#[derive(Default)]
pub struct Executor {}

impl Executor {
    pub async fn execute(command_name: &String, arguments: &Vec<String>, options: &Options) {
        let result = Executor::resolve(command_name);

        match result.await {
            Ok(ref command) => command.execute(arguments, options),
            Err(_) => println!("Command {} was not found.", command_name),
        }
    }

    async fn resolve(command_name: &String) -> Result<Box<dyn Command>, &str> {
        let config = aws_config::load_from_env().await;

        let migrate = Migrate::new(DynamodbClient::new(dynamodb::Client::new(&config)));
        let list = List::new();

        let command: Box<dyn Command>;
        match command_name {
            _ if (command_name == migrate.command_name()) => command = Box::new(migrate),
            _ if (command_name == list.command_name())    => command = Box::new(list),
            _                                             => {
                return Err(stringify!("Cannot resolve command_name. Command name {}.", command_name))
            }
        }

        Ok(command)
    }
}
