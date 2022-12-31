use aws_config::SdkConfig;
use aws_sdk_dynamodb as dynamodb;
use crate::clients::dynamodb_client::DynamodbClient;

use crate::commands::list::List;
use crate::command::{Command, ExitCode, Output};
use crate::commands::migrate::Migrate;
use crate::config::aws_config::AwsConfig;
use crate::lexers::option_lexer::Options;

#[derive(Default)]
pub struct Executor {}

impl Executor {
    pub async fn execute(self, command_name: &String, arguments: &Vec<String>, options: &Options) -> Output {
        let config =  AwsConfig::aws_config().await;

        let result= self.find_by_command_name(command_name, &config);

        let output: Output;
        match result {
            Ok(command) => {
                output = command.execute(arguments, options).await;
            },
            Err(_) => {
                output = Output::new(ExitCode::FAILED, format!("Command {} was not found.", command_name))
            },
        }

        output
    }

    fn find_by_command_name<'a>(self, command_name: &'a String, config: &'a SdkConfig) -> Result<Box<dyn Command>, &'a str> {
        let migrate = Migrate::new(DynamodbClient::new(dynamodb::Client::new(config)));
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
