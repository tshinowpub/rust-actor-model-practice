use crate::lexers::option_lexer::Options;

pub trait Command {
    fn execute(&self, arguments: &Vec<String>, options: &Options);

    fn command_name(&self) -> &str;
}
