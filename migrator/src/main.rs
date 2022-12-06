use std::env;
use std::process::exit;

mod executor;
mod commands;

use crate::executor::Executor;

fn main() {
    let execute_path: String;
    match env::current_exe() {
        Ok(exe_path) =>
            execute_path = exe_path.display().to_string(),
        Err(e) => {
            println!("Failed to get current execute path: {e}.");

            exit(1);
        },
    };

    let arguments = extract_user_arguments(env::args().collect(), execute_path);
    match arguments.first() {
        Some(command)    => {
            Executor::execute(command);
        }
        None => {
            println!("Use --help.");

            exit(1);
        }
    }

    exit(0);
}

fn extract_user_arguments(arguments: Vec<String>, execute_path: String) -> Vec<String> {
    let user_arguments: Vec<String> = arguments
        .iter()
        .filter_map(|s| {
            if s.to_string() != execute_path {
                return s.parse::<String>().ok()
            }

            None
        })
        .collect();

    user_arguments.clone()
}
