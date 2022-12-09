use crate::command::Command;

pub struct List {}

impl List {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for List {
    fn execute(&self, arguments: &Vec<String>, options: &Vec<String>) {
        println!("List!!!")
    }

    fn command_name(&self) -> &str {
        "list"
    }
}
