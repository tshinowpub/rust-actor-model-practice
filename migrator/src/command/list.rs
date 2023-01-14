use async_trait::async_trait;

use crate::command::{Command, ExitCode, Output};
use crate::lexer::option_lexer::Options;

#[derive(Debug, Copy, Clone)]
pub struct List {}

impl List {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for List {
    async fn execute(&self, arguments: &Vec<String>, options: &Options) -> Output {
        let message = "Usage:  migrator [Command] [Option] \n
        Options:
            list    Display command list.
            migrate Execute migration..
        ";

        Output::new(ExitCode::SUCCEED, message.to_string())
    }

    fn command_name(self) -> &'static str {
        "list"
    }
}
