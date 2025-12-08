//! # Utility Functions
//!
//! This module contains general utility functions used across the application,
//! primarily focusing on date parsing and formatting helpers.

use ::chrono::{NaiveDate, Utc};

/// A list of supported date formats used when attempting to parse a date string.
///
/// The formats are checked in the order defined here:
/// - ISO 8601: `YYYY-MM-DD`
/// - Dotted: `DD.MM.YYYY`
/// - US Slash: `MM/DD/YYYY`
/// - Hyphenated: `DD-MM-YYYY`
/// - Written Month: `DD Mon YYYY`
/// - Slash: `YYYY/MM/DD`
const POSSIBLE_FORMATS: &[&str] = &[
    "%Y-%m-%d", "%d.%m.%Y", "%m/%d/%Y", "%d-%m-%Y", "%d %b %Y", "%Y/%m/%d",
];

/// Attempts to parse a date string using a variety of common formats.
///
/// This function iterates through `POSSIBLE_FORMATS` and returns the first successful
/// parse result. It abstracts away the need to know the exact format of the input string.
///
/// # Arguments
///
/// * `date_str` - A string slice or type that can be referenced as a string containing the date.
///
/// # Returns
///
/// * `Some(NaiveDate)` - If the string matches one of the supported formats.
/// * `None` - If the string does not match any of the supported formats.
fn parse_date_with_unknown_format(date_str: impl AsRef<str>) -> Option<NaiveDate> {
    let date_str = date_str.as_ref();
    POSSIBLE_FORMATS
        .iter()
        .find_map(|&format| NaiveDate::parse_from_str(date_str, format).ok())
}

/// Normalizes a date string to the ISO 8601 format (`YYYY-MM-DD`).
///
/// If the input string is `None` or cannot be parsed using any of the supported formats,
/// the current UTC date is returned.
///
/// # Arguments
///
/// * `date_str` - An optional string slice containing the date to normalize.
///
/// # Returns
///
/// * `String` - The date formatted as `YYYY-MM-DD`.
pub fn normalize_date(date_str: Option<impl AsRef<str>>) -> String {
    date_str
        .as_ref()
        .and_then(parse_date_with_unknown_format)
        .unwrap_or_else(|| Utc::now().date_naive())
        .format("%Y-%m-%d")
        .to_string()
}

// pub fn resolve_date(date_str: Option<impl AsRef<str>>) -> Option<String> {
//     date_str
//         .as_ref()
//         .and_then(parse_date_with_unknown_format)
//         .map(|date| date.format("%Y-%m-%d").to_string())
// }
