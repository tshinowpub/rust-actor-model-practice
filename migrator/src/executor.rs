use crate::commands::migrator_command::{List, Migrate};
use crate::commands::migrator_command::Command;

#[derive(Default)]
pub struct Executor {}

impl Executor {
    pub fn execute(command_name: &String) {
        let command = Executor::resolve(command_name);

        command.execute();
    }

    fn resolve(command_name: &String) -> Box<dyn Command> {
        let migrate = Migrate::new();

        let command: Box<dyn Command>;
        match command_name {
            _ if (command_name == migrate.command_name()) => command = Box::new(migrate),
            _                                             => command = Box::new(List::new())
        }

        command
    }
}
