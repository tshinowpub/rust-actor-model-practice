pub mod list;
pub mod migrate;
pub mod migrate_operation_type;
pub mod migrate_type;
pub mod reset;

#[derive(Debug, Clone)]
pub struct Output {
    exit_code: ExitCode,
    message: String,
}

impl Output {
    pub fn new(exit_code: ExitCode, message: String) -> Self {
        Self { exit_code, message }
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
    Succeed = 0,
    #[allow(dead_code)]
    Failed = 1,
}
