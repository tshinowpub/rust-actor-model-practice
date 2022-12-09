use crate::commands::command::Command;

pub struct List {}

impl List {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for List {
    fn execute(&self) {
        println!("List!!!")
    }

    fn command_name(&self) -> &str {
        "list"
    }
}
