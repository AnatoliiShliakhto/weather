//! # Provider Handlers
//!
//! This module contains handler functions for managing weather service providers.

use crate::common::*;
use ::clap::ValueEnum;
use ::weather_providers::Provider;

/// Lists all supported weather providers and their current configuration status.
///
/// This function iterates through all available variants of `Provider` and checks
/// the application configuration to see if an API key is set for each.
/// It prints a formatted table to the standard output.
///
/// # Returns
///
/// Returns `Ok(())` if the list was successfully printed.
pub fn list_providers() -> Result<()> {
    let config = APP_STATE.config.get()?;

    println!("Weather providers:\n");
    println!(
        "{:<5} | {:<15} | {:<10}\n------+-----------------+---------",
        "ID", "PROVIDER", "API KEY"
    );

    for provider in Provider::value_variants() {
        let provider_id = provider.id();
        let key = config
            .providers
            .get(provider_id)
            .and_then(|p| p.key.as_deref())
            .filter(|k| !k.is_empty())
            .unwrap_or("-");

        println!("{:<5} | {:<15} | {:<10}", provider_id, provider.name(), key);
    }
    println!();

    match &config.default_provider {
        Some(id) => {
            let display_name = Provider::try_from(id.as_str())
                .map(|p| p.to_string())
                .unwrap_or_else(|_| id.clone());
            println!("Default provider: '{display_name}' ({id})");
        }
        None => println!("Default provider is not set."),
    }

    Ok(())
}

/// Configures a provider and optionally sets it as the default.
///
/// # Actions
/// 1. **Updates API Key**: If a `key` is provided and not empty, it updates the stored key for the provider.
/// 2. **Sets Default**: Attempts to make this provider the global default.
///    - **Success**: If the provider is `Mock` OR if a valid API key exists (either newly set or previously saved).
///    - **Warning**: If attempting to set a non-Mock provider as default without an API key, the default provider
///      will *not* be changed, and a warning will be displayed.
///
/// # Arguments
///
/// * `provider` - The identifier of the provider (e.g., "ow", "wa").
/// * `key` - An optional API key.
///
/// # Returns
///
/// Returns `Ok(())` if the configuration process is completed (even if a warning was issued).
pub fn set_provider(provider: impl AsRef<str>, key: Option<impl AsRef<str>>) -> Result<()> {
    let provider = Provider::try_from(provider.as_ref())?;

    let key_to_set = key.as_ref().map(|k| k.as_ref()).filter(|k| !k.is_empty());

    let mut message = String::new();

    APP_STATE.config.with_mut(|state| {
        if let Some(k) = key_to_set {
            state
                .providers
                .entry(provider.id().to_string())
                .or_default()
                .key = Some(k.to_string());
            message.push_str(&format!("API key for '{provider}' updated.\n"));
        }

        let has_key = state
            .providers
            .get(provider.id())
            .and_then(|p| p.key.as_deref())
            .filter(|k| !k.is_empty())
            .is_some();

        if provider.is_mock() || has_key {
            state.default_provider = Some(provider.id().to_string());
            message.push_str(&format!("Default provider set to: '{provider}'\n"));
        } else {
            message.push_str(&format!(
                "WARNING: API key not found for '{provider}'. Default provider NOT changed.\n\
                Please set the key first using --key <API_KEY>"
            ));
        }
    })?;

    if !message.is_empty() {
        println!("{message}");
    }

    Ok(())
}
