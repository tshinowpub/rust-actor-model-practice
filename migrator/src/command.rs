use async_trait::async_trait;
use crate::lexers::option_lexer::Options;

#[async_trait]
pub trait Command {
    async fn execute(&self, arguments: &Vec<String>, options: &Options);

    fn command_name(self) -> &'static str;
}
