use std::env;
use std::process::exit;
use crate::command::{ExitCode, Output};

mod command;
mod executor;
mod extractor;
mod clients;
mod lexer;

use crate::executor::Executor;
use crate::extractor::argument_extractor::ArgumentExtractor;
use crate::extractor::option_extractor::OptionExtractor;
use crate::lexer::option_lexer::OptionLexer;

#[tokio::main]
async fn main() {
    let execute_path: String;
    match env::current_exe() {
        Ok(exe_path) =>
            execute_path = exe_path.display().to_string(),
        Err(e) => {
            println!("Failed to get current execute path: {e}.");

            exit(1);
        },
    };

    let argument_extractor = ArgumentExtractor::default();
    let option_extractor = OptionExtractor::default();
    let lexer = OptionLexer::default();

    let arguments = argument_extractor.extract(env::args().collect(), &execute_path);
    let option_arguments = option_extractor.extract(env::args().collect(), &execute_path);

    let options = lexer.parse(&option_arguments);

    let executor = Executor::default();

    let output: Output;
    match arguments.split_first() {
        Some((command, args)) if !args.is_empty() => output = executor.execute(command, &args.to_vec(), &options).await,
        Some((command, _)) => output = executor.execute(command, &Vec::new(), &options).await,
        _ => output = Output::new(ExitCode::FAILED, "Use --help.".to_string())
    }

    println!("{}", output.message());

    exit(*(output.exit_code()) as i32);
}
