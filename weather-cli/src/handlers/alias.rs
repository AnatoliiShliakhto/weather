//! # Alias Handlers
//!
//! This module provides functionality for managing location aliases.
//! Aliases allow users to assign short names to frequently used addresses
//! (e.g., "home" -> "London, UK").

use crate::common::*;

/// Lists all configured location aliases.
///
/// Prints a formatted table of all saved aliases and their corresponding addresses
/// to the standard output.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `Error` if the configuration cannot be accessed.
pub fn list_aliases() -> Result<()> {
    let config = APP_STATE.config.get()?;

    if config.addresses.is_empty() {
        println!("No aliases are set.");
        return Ok(());
    }

    println!("Aliases:\n");
    println!(
        "{:<10} | {:<30}\n-----------+--------------",
        "ALIAS", "ADDRESS"
    );

    for (alias, address) in &config.addresses {
        println!("{:<10} | {:<30}", alias, address);
    }
    println!();

    match &config.default_alias {
        Some(alias) => println!("Default alias: {alias}"),
        None => println!("No default alias is set."),
    }

    Ok(())
}

/// Creates or updates a location alias.
///
/// This function associates a short `alias` name with a full `address` string.
///
/// # Validation
///
/// - The `alias` must be between 1 and 5 characters long.
/// - The `address` must not be empty.
///
/// # Arguments
///
/// * `alias` - The short name for the location (e.g., "nyc").
/// * `address` - The full location string (e.g., "New York, USA").
///
/// # Returns
///
/// Returns `Ok(())` if the operation completes (even if validation fails),
/// or an `Error` if saving the configuration fails.
pub fn set_alias(alias: &str, address: Option<&str>) -> Result<()> {
    if address.is_none() {
        return set_default_alias(alias);
    }

    let Some(address) = address.filter(|a| !a.trim().is_empty()) else {
        Err("Address cannot be empty. Use --address <ADDRESS>")?
    };

    let alias = alias.trim();
    let char_count = alias.chars().count();

    if alias.is_empty() || char_count > 5 {
        Err("Alias must be between 1 and 5 characters long.")?
    }

    APP_STATE.config.with_mut(|s| {
        s.addresses.insert(alias.to_string(), address.to_string());
        if s.default_alias.is_none() {
            s.default_alias = Some(alias.to_string());
            println!("Alias '{alias}' set as default.");
        }
    })?;

    println!("Alias '{alias}' set to '{address}'");

    Ok(())
}

/// Removes an existing location alias.
///
/// # Arguments
///
/// * `alias` - The name of the alias to remove.
///
/// # Returns
///
/// Returns `Ok(())` if the operation completes, or an `Error` if saving the configuration fails.
pub fn remove_alias(alias: &str) -> Result<()> {
    let mut was_default = false;
    let mut existed = false;

    APP_STATE.config.with_mut(|s| {
        existed = s.addresses.remove(alias).is_some();

        if s.default_alias.as_deref() == Some(alias) {
            s.default_alias = None;
            was_default = true;
        }
    })?;

    if existed {
        println!("Alias '{alias}' removed.");
        if was_default {
            println!("Note: '{alias}' was the default alias. Default alias is now unset.");
        }
    } else {
        println!("Alias '{alias}' not found.");
    }

    Ok(())
}

/// Sets the default address alias.
///
/// # Arguments
///
/// * `alias` - The name of the alias to set as default (e.g., "home", "work").
///
/// # Returns
///
/// * `Ok(())` if the alias was found and successfully set as default.
/// * `Error` if the alias does not exist in the configuration or if saving failed.
fn set_default_alias(alias: &str) -> Result<()> {
    let alias_exists = {
        let state = APP_STATE.config.get()?;
        state.addresses.contains_key(alias)
    };

    if !alias_exists {
        Err(format!("Alias '{alias}' not found"))?
    }

    APP_STATE.config.with_mut(|s| {
        s.default_alias = Some(alias.to_string());
    })?;

    println!("Alias '{alias}' set as default.");

    Ok(())
}