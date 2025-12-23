mod clipboard;
mod core;

use clap::Parser;
use core::search::{SearchError, SearchResult, search};
use std::process::ExitCode;

const DEFAULT_ENV_FILE: &str = ".env";

/// Searches for an environment variable in a `.env` file and copies its value to the clipboard.
///
/// The program looks up a key provided via the `-k` / `--key` argument inside an environment file.
/// The environment file can be explicitly specified as an argument; otherwise, a default `.env`
/// file is used.
///
/// If the key is found, its associated value is copied to the system clipboard for easy reuse.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the environment file containing the key
    #[arg(short, long, default_value = DEFAULT_ENV_FILE)]
    filepath: String,

    /// Key to search within the environment file
    #[arg(short, long)]
    key: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match search(&args.filepath, &args.key) {
        Ok(SearchResult::Found(value)) => {
            if let Err(err) = clipboard::clipboard::copy(value.as_str()) {
                eprintln!("{}", err.get_message());
            }
        }
        Ok(SearchResult::NotFound) => {
            eprintln!("No value found");
            return ExitCode::FAILURE;
        }
        Err(SearchError::FileNotExists) => {
            eprintln!("error: getenv: {} file not found", &args.filepath);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
