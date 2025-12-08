use ::std::borrow::Cow;

/// The central error type for the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Represents a generic or unspecified error with a custom message.
    /// Uses `Cow` to efficiently handle both static string slices and owned strings.
    #[error("{0}")]
    Any(Cow<'static, str>),

    /// Represents input/output errors (e.g., file not found, permission denied).
    // #[error("I/O error: {0}")]
    // Io(#[from] std::io::Error),

    /// Represents errors occurring during HTTP requests.
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl From<String> for Error {
    /// Converts an owned `String` into an `Error::Any`.
    fn from(msg: String) -> Self {
        Self::Any(Cow::Owned(msg))
    }
}
impl From<&'static str> for Error {
    /// Converts a static string slice (`&'static str`) into an `Error::Any`.
    fn from(msg: &'static str) -> Self {
        Self::Any(Cow::Borrowed(msg))
    }
}

/// A specialized `Result` type for the application.
///
/// This type ID simplifies function signatures by setting the default error type
/// to `Error`. This avoids the need to repeatedly specify `Error` throughout
/// the codebase and ensures consistent error handling.
///
/// # Examples
///
/// ```rust
/// use weather_providers::Result;
///
/// fn task() -> Result<()> {
///     // ...
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;
