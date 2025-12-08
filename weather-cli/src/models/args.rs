//! # CLI Argument Definitions
//!
//! This module defines the command-line interface (CLI) structure using the `clap` crate.
//! It specifies the available subcommands, arguments, and flags for the application.

use ::clap::{Parser, Subcommand};

/// The main CLI structure parsing command-line arguments.
#[derive(Parser)]
#[command(name = "weather")]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Enable debug console output.
    #[arg(long, global = true)]
    pub debug: bool,

    /// The main subcommand to execute.
    #[command(subcommand)]
    pub command: Option<AppCommands>,
}

/// Enumeration of available application subcommands.
#[derive(Subcommand)]
pub enum AppCommands {
    /// Retrieve weather information.
    Get {
        /// The address or address alias to query.
        #[arg(value_name = "LOCATION")]
        address: Option<String>,

        /// The date to retrieve weather information for.
        #[arg(short, long, value_name = "DATE")]
        date: Option<String>,

        /// Explicitly select the weather provider to use for this request.
        #[arg(short, long, value_name = "PROVIDER")]
        provider: Option<String>,
    },

    /// Manage weather service providers.
    #[command(arg_required_else_help = true)]
    Provider {
        /// Set the specified provider as the default.
        #[arg(value_name = "PROVIDER")]
        provider: Option<String>,

        /// Set or update the API key for the selected provider.
        #[arg(short, long, value_name = "API_KEY")]
        key: Option<String>,

        /// List all supported providers and their configuration status.
        #[arg(short, long, conflicts_with_all = ["provider", "key"])]
        list: bool,
    },

    /// Manage location aliases, e.g., "home" -> "London, UK"
    #[command(arg_required_else_help = true)]
    Alias {
        /// Set the specified alias as the default.
        #[arg(value_name = "ALIAS")]
        name: Option<String>,

        /// The full address to assign to the alias.
        #[arg(short, long, value_name = "ADDRESS", requires = "name")]
        address: Option<String>,

        /// Remove the specified alias.
        #[arg(short, long, requires = "name", conflicts_with = "address")]
        remove: bool,

        /// List all configured aliases.
        #[arg(short, long, conflicts_with_all = ["name", "address", "remove"])]
        list: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_get_basic() {
        let args = Cli::try_parse_from(["weather", "get", "London"]).unwrap();
        match args.command {
            Some(AppCommands::Get { address, date, provider }) => {
                assert_eq!(address, Some("London".to_string()));
                assert_eq!(date, None);
                assert_eq!(provider, None);
            }
            _ => panic!("Expected Get command"),
        }
    }

    #[test]
    fn test_parse_get_full() {
        let args = Cli::try_parse_from([
            "weather", "get", "Paris", "--date", "2023-01-01", "--provider", "ow"
        ]).unwrap();

        match args.command {
            Some(AppCommands::Get { address, date, provider }) => {
                assert_eq!(address, Some("Paris".to_string()));
                assert_eq!(date, Some("2023-01-01".to_string()));
                assert_eq!(provider, Some("ow".to_string()));
            }
            _ => panic!("Expected Get command"),
        }
    }

    #[test]
    fn test_provider_conflicts() {
        let result = Cli::try_parse_from(["weather", "provider", "ow", "--list"]);
        assert!(result.is_err());

        let result = Cli::try_parse_from(["weather", "provider", "--key", "123", "--list"]);
        assert!(result.is_err());

        let args = Cli::try_parse_from(["weather", "provider", "--list"]).unwrap();
        match args.command {
            Some(AppCommands::Provider { list, .. }) => assert!(list),
            _ => panic!("Expected Provider command"),
        }
    }

    #[test]
    fn test_alias_constraints() {
        // --address requires a name
        let result = Cli::try_parse_from(["weather", "alias", "--address", "London"]);
        assert!(result.is_err());

        // --remove requires a name
        let result = Cli::try_parse_from(["weather", "alias", "--remove"]);
        assert!(result.is_err());

        // --remove conflicts with --address
        let result = Cli::try_parse_from([
            "weather", "alias", "home", "--address", "London", "--remove"
        ]);
        assert!(result.is_err());

        // Valid alias setting
        let args = Cli::try_parse_from([
            "weather", "alias", "home", "--address", "London, UK"
        ]).unwrap();

        match args.command {
            Some(AppCommands::Alias { name, address, .. }) => {
                assert_eq!(name, Some("home".to_string()));
                assert_eq!(address, Some("London, UK".to_string()));
            }
            _ => panic!("Expected Alias command"),
        }
    }

    #[test]
    fn test_global_debug_flag() {
        let args = Cli::try_parse_from(["weather", "--debug", "get"]).unwrap();
        assert!(args.debug);
    }
}