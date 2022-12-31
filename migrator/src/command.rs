use async_trait::async_trait;
use crate::lexers::option_lexer::Options;

#[async_trait]
pub trait Command {
    async fn execute(&self, arguments: &Vec<String>, options: &Options);

    fn command_name(self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct Output {
    exit_code: ExitCode,
    output: String
}

impl Output {
    pub fn new(exit_code: ExitCode, output: String) -> Self {
        Self{ exit_code, output }
    }
}

#[derive(Debug, Clone)]
pub enum ExitCode {
    SUCCEED = 0,
    FAILED = 1
}
