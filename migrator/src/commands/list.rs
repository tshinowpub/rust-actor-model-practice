use async_trait::async_trait;

use crate::command::Command;
use crate::lexers::option_lexer::Options;

pub struct List {}

impl List {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for List {
    async fn execute(&self, arguments: &Vec<String>, options: &Options) {
        println!("List!!!")
    }

    fn command_name(&self) -> &str {
        "list"
    }
}
