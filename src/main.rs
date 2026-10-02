use notes::cli::{self, Error};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match cli::run(&args, &mut std::io::stdin(), &mut std::io::stdout()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Error::Usage(message)) => {
            eprintln!("notes: {message}\n{}", cli::USAGE);
            ExitCode::from(2)
        }
        Err(Error::Failed(message)) => {
            eprintln!("notes: {message}");
            ExitCode::FAILURE
        }
    }
}
