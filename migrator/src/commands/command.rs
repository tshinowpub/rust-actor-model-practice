pub trait Command {
    fn execute(&self);

    fn command_name(&self) -> &str;
}
