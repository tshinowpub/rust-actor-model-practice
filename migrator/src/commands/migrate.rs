use crate::commands::command::Command;

pub struct Migrate {}

impl Migrate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Migrate {
    fn execute(&self) {
        println!("Migrate!!!")
    }

    fn command_name(&self) -> &str {
        "migrate"
    }
}
