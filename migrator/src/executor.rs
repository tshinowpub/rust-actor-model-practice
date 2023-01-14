use aws_config::SdkConfig;
use aws_sdk_dynamodb::{Credentials, Region};

use crate::command::list::List;
use crate::command::{Command, ExitCode, Output};
use crate::command::migrate::Migrate;
use crate::lexer::option_lexer::Options;

#[derive(Default)]
pub struct Executor {}

/**
 * https://github.com/awslabs/aws-sdk-rust/issues/425#issuecomment-1020265854
 */
impl Executor {
    pub async fn execute(self, command_name: &String, arguments: &Vec<String>, options: &Options) -> Output {
        let default_provider = Credentials::new("test", "test", None, None, "local");

        let result= self.find_by_command_name(command_name);

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

    fn find_by_command_name<'a>(self, command_name: &'a String) -> Result<Box<dyn Command>, &'a str> {
        let migrate = Migrate::new();
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
