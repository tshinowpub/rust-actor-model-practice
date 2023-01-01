use async_trait::async_trait;
use crate::lexer::option_lexer::Options;

pub mod migrate;
pub mod migration_query;
pub mod list;

#[async_trait]
pub trait Command {
    async fn execute(&self, arguments: &Vec<String>, options: &Options) -> Output;

    fn command_name(self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct Output {
    exit_code: ExitCode,
    message: String
}

impl Output {
    pub fn new(exit_code: ExitCode, message: String) -> Self {
        Self{ exit_code, message }
    }

    pub fn exit_code(&self) -> &ExitCode {
        &self.exit_code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ExitCode {
    SUCCEED = 0,
    FAILED = 1
}
