use crate::command::{ExitCode, Output};

#[derive(Debug, Copy, Clone)]
pub struct List {}

impl List {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(self) -> Output {
        let message = "Usage:  migrator [Command] [Option] \n
        Options:
            list    Display command list.
            migrate Execute migration..
        ";

        Output::new(ExitCode::Succeed, message.to_string())
    }
}
