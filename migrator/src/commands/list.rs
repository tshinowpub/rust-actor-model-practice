use crate::command::Command;
use crate::lexers::option_lexer::Options;

pub struct List {}

impl List {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for List {
    fn execute(&self, arguments: &Vec<String>, options: &Options) {
        println!("List!!!")
    }

    fn command_name(&self) -> &str {
        "list"
    }
}
