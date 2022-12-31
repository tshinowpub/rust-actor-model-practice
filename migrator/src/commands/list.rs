use async_trait::async_trait;

use crate::command::{Command, ExitCode, Output};
use crate::lexers::option_lexer::Options;

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
        Output::new(ExitCode::SUCCEED, "List !!!".to_string())
    }

    fn command_name(self) -> &'static str {
        "list"
    }
}
