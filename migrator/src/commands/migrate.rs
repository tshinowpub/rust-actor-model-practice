use crate::command::Command;

pub struct Migrate {}

impl Migrate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Migrate {
    fn execute(&self, arguments: Vec<String>, options: Vec<String>) {
        println!("Migrate!!!")
    }

    fn command_name(&self) -> &str {
        "migrate"
    }
}
