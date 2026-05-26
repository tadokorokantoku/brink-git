mod cli;

use clap::Parser;
use std::process::ExitCode;

use cli::Cli;

fn main() -> ExitCode {
    match cli::run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e:#}");
            ExitCode::FAILURE
        }
    }
}
