use crate::commands::list::List;
use crate::command::Command;
use crate::commands::migrate::Migrate;

#[derive(Default)]
pub struct Executor {}

impl Executor {
    pub fn execute(command_name: &String, arguments: Vec<String>, options: Vec<String>) {
        let result = Executor::resolve(command_name);

        match result {
            Ok(ref command) => command.execute(arguments, options),
            Err(_) => println!("Command {} was not found.", command_name),
        }
    }

    fn resolve(command_name: &String) -> Result<Box<dyn Command>, &str> {
        let migrate = Migrate::new();
        let list = List::new();

        let command: Box<dyn Command>;
        match command_name {
            _ if (command_name == migrate.command_name()) => command = Box::new(migrate),
            _ if (command_name == list.command_name())    => command = Box::new(list),
            _                                             => {
                return Err(stringify!("Cannot resolve command_name. Command name {}.", command_name))
            }
        }

        Ok(command)
    }
}
