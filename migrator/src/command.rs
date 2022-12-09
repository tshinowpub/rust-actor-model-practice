pub trait Command {
    fn execute(&self, arguments: &Vec<String>, options: &Vec<String>);

    fn command_name(&self) -> &str;
}
