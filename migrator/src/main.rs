use std::env;
use std::process::exit;
use crate::command::{ExitCode, Output};

mod command;
mod executor;
mod clients;

use crate::executor::Executor;

#[tokio::main]
async fn main() {
    let args_for_command :Vec<String> = env::args().skip(1).collect();
    let command_args :Vec<String> = env::args().skip(2).collect();

    let executor = Executor::default();

    let output: Output;
    match (args_for_command.first(), command_args) {
        (Some(command), args) => output = executor.execute(command, &args).await,
        _                                        => {
            let args: Vec<String> = Vec::new();

            output = executor.execute(&"list".to_string(), &args).await
        },
    }

    println!("{}", output.message());

    exit(*(output.exit_code()) as i32);
}
