pub trait Command {
    fn execute(&self);

    fn command_name(&self) -> &str;
}

pub struct Migrate {}
pub struct List {}

impl Migrate {
    pub fn new() -> Self {
        Self {}
    }
}

impl List {
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

impl Command for List {
    fn execute(&self) {
        println!("List!!!")
    }

    fn command_name(&self) -> &str {
        "list"
    }
}

