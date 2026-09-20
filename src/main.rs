//! The `nala` binary: a thin shell around the `nala` library.
//!
//! It parses command-line arguments and hands off to [`nala::run`]. All the
//! real logic lives in the library so it can be tested independently.

use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;

use nala::cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match nala::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // Print to stderr so it doesn't pollute piped output, and return a
            // non-zero exit code so scripts can detect the failure.
            eprintln!("{} {err}", "error:".red().bold());
            ExitCode::FAILURE
        }
    }
}
