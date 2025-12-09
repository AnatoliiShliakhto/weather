//! # Weather Handlers
//!
//! This module orchestrates the process of retrieving weather information.
//! It acts as a bridge between the CLI input, the application configuration,
//! and the specific weather provider services.

use crate::common::*;
use ::weather_providers::{Provider, create_provider};

/// Retrieves and displays weather information for a specified location.
///
/// This function acts as the primary handler for the `get` command. It orchestrates the entire
/// flow from input resolution to displaying the final result.
///
/// # Process
///
/// 1.  **Provider Resolution**: Determines which weather service (provider) to use.
///     It checks the explicit `provider` argument first, falling back to the configuration's default.
///     It also ensures the necessary API key is available.
/// 2.  **Address Resolution**: Resolves the target location. If `address` matches a configured alias,
///     it uses the mapped value; otherwise, it treats the input as a raw location string.
/// 3.  **Data Retrieval**: Instantiates the resolved provider and requests weather data, passing
///     the resolved address and optional date.
/// 4.  **Display**: Prints the formatted weather information to the standard output.
///
/// # Arguments
///
/// *   `address` - An optional location string or alias. If `None`, the application attempts to use the default alias from the config.
/// *   `date` - An optional date string. The format is flexible (handled by the provider's normalization logic).
/// *   `provider` - An optional provider identifier (e.g., "ow", "wa"). If `None`, the default provider is used.
///
/// # Returns
///
/// Returns `Ok(())` if the operation completes successfully.
///
/// Returns an `Error` in the following cases:
/// *   No address is specified and no default alias is found.
/// *   The specified or default provider requires an API key that is missing from the configuration.
/// *   The weather provider encounters an error (e.g., network failure, invalid location).
pub async fn get_weather(
    address: Option<String>,
    date: Option<String>,
    provider: Option<String>,
) -> Result<()> {
    let (provider, api_key) = resolve_provider(provider)?;
    let address = resolve_address(address)?;

    println!("Fetching weather from '{provider}' for '{address}'...");

    let weather_provider = create_provider(provider);
    let weather_info = weather_provider
        .get_weather(api_key.as_deref(), &address, date.as_deref())
        .await?;

    println!("{weather_info}");

    Ok(())
}

/// Determines the weather provider to use and retrieves its configuration.
///
/// # Logic
///
/// 1. If a `provider_input` is given, it tries to parse it.
/// 2. If not, it looks for a default provider in the configuration.
/// 3. If neither is present, it falls back to the `Mock` provider.
///
/// It also retrieves the API key for the selected provider from the config.
///
/// # Errors
///
/// Returns an error if the selected provider is NOT the Mock provider and no API key
/// is found in the configuration.
fn resolve_provider(provider_input: Option<String>) -> Result<(Provider, Option<String>)> {
    let config = APP_STATE.config.get()?;

    let provider = match provider_input {
        Some(p) => Provider::try_from(p.as_str())?,
        None => config
            .default_provider
            .as_deref()
            .map(Provider::try_from)
            .transpose()?
            .unwrap_or(Provider::Mock),
    };

    let api_key = config
        .providers
        .get(provider.id())
        .and_then(|p| p.key.clone());

    if !provider.is_mock() && api_key.is_none() {
        Err(format!(
            "API key not found for provider '{provider}'. Please configure it first."
        ))?;
    }

    Ok((provider, api_key))
}

/// Resolves the target location string from the input.
///
/// # Logic
///
/// 1. **Input Provided**:
///    - Checks if the input string matches a saved alias key. If yes, returns the associated address.
///    - If no match, treats the input as the raw address.
/// 2. **No Input**:
///    - Checks if a `default_alias` is set in the configuration.
///    - If set, looks up the address for that alias.
///
/// # Errors
///
/// Returns an error if no address is provided and no default alias is configured.
/// Logs a warning if a default alias is set but points to a non-existent entry.
fn resolve_address(address_input: Option<String>) -> Result<String> {
    let config = APP_STATE.config.get()?;
    let addresses = &config.addresses;

    if let Some(input) = address_input {
        if let Some(mapped_address) = addresses.get(&input) {
            return Ok(mapped_address.clone());
        }
        return Ok(input);
    }

    if let Some(default_alias) = &config.default_alias {
        if let Some(mapped_address) = addresses.get(default_alias) {
            return Ok(mapped_address.clone());
        }
        println!("Default alias '{default_alias}' is set but not found in saved aliases.");
    }

    Err("No address specified and no default address alias found. \
         Use --address <LOCATION> or set a default alias.")?
}
