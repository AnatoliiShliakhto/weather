//! # Weather CLI
//!
//! `weather` is the command-line interface entry point for the Awesome Weather application for cool guys.
//! It handles parsing command-line arguments, initializing the application environment (logging, configuration),
//! and dispatching commands to the appropriate handlers.
//!
//! ## Modules
//!
//! - `common`: Shared utilities, configuration, error types, and logging setup.
//! - `handlers`: Implementation of business logic for each CLI command.
//! - `models`: Data structures representing CLI arguments and application state.
//!
//! ## Execution Flow
//!
//! 1.  **Parse Arguments**: Uses `clap` to parse arguments into the `Cli` struct.
//! 2.  **Initialize Logging**: Sets up tracing/logging based on the debug flag.
//! 3.  **Dispatch Command**: Matches the parsed subcommand (`get`, `provider`, `alias`) and calls the corresponding handler function.
//! 4.  **Error Handling**: Catches any errors bubbled up from handlers, prints them to `stderr`, and exits with a non-zero status code.

mod common;
mod handlers;
mod models;

use crate::{common::*, models::args::*};
use ::clap::Parser;
use ::tracing::debug;

/// The main entry point of the application.
///
/// It initializes the Tokio runtime and delegates the execution to `run()`.
/// If `run()` returns an error, it prints the error message to stderr and terminates the process with exit code 1.
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// Orchestrates the application logic.
///
/// This function:
/// 1. Parses CLI arguments.
/// 2. Initializes the logging system.
/// 3. Matches the requested subcommand and invokes the relevant handler from the `handlers` module.
///
/// # Returns
///
/// Returns `Ok(())` if the command executes successfully, or an `Error` if any step fails.
async fn run() -> Result<()> {
    let cli = Cli::parse();

    let _logger_guard = logging::init(cli.debug)?;

    if cli.debug {
        debug!("Debug output enabled.");
    }

    let Some(command) = cli.command else {
        return Ok(());
    };

    match command {
        AppCommands::Get {
            address,
            date,
            provider,
        } => {
            handlers::get_weather(address, date, provider).await?;
        }

        AppCommands::Provider {
            provider,
            key,
            list,
        } => {
            if list {
                return handlers::list_providers();
            }

            if let Some(provider_str) = provider {
                handlers::set_provider(provider_str, key)?;
            }
        }

        AppCommands::Alias {
            name,
            address,
            remove,
            list,
        } => {
            if list {
                return handlers::list_aliases();
            }

            if let Some(alias_name) = name {
                if remove {
                    handlers::remove_alias(alias_name.as_str())?;
                } else {
                    handlers::set_alias(alias_name.as_str(), address.as_deref())?;
                }
            }
        }
    }

    Ok(())
}
