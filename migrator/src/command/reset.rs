use crate::command::{ExitCode, Output};

#[derive(Debug, Copy, Clone)]
pub struct Reset {}

impl Reset {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(self) -> Output {
        Output::new(ExitCode::SUCCEED, "".to_string())
    }
}
