use std::env;
use std::process::exit;

mod commands;
mod executor;
mod extractor;
mod clients;
mod lexers;

pub mod command;

use crate::executor::Executor;
use crate::extractor::argument_extractor::ArgumentExtractor;
use crate::extractor::option_extractor::OptionExtractor;
use crate::lexers::option_lexer::OptionLexer;

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

    match arguments.split_first() {
        Some((command, args)) if !args.is_empty() => {
            Executor::execute(command, &args.to_vec(), &options).await;
        },
        Some((command, _)) => {
            Executor::execute(command, &Vec::new(), &options).await;
        },
        _ => {
            println!("Use --help.");

            exit(1);
        }
    }

    exit(0);
}
