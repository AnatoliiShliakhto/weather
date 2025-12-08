use crate::common::*;
use ::std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use ::tracing::level_filters::LevelFilter;
use ::tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use ::tracing_subscriber::{
    Layer, Registry,
    fmt::{MakeWriter, layer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// Suffix for log files.
const LOG_FILE_SUFFIX: &str = "log";
/// Maximum number of log files to keep during rotation.
const MAX_LOG_FILES: usize = 10;

/// Initializes the logging system for the application.
///
/// Configures `tracing` to output logs to both stdout (only in debug builds)
/// and a rotating file appender.
///
/// # Arguments
///
/// * `debug_mode` - If `true`, sets the log level to `DEBUG`. Otherwise, defaults to `INFO`.
///
/// # Returns
///
/// Returns a `Result` containing a `WorkerGuard`. This guard **must** be held
/// by the main function (e.g., assigned to a variable like `_guard`) to ensure
/// that logs are flushed to the file before the application exits.
///
/// # Errors
///
/// Returns an error if:
/// * The log directory cannot be created.
/// * The file appender cannot be initialized.
/// * The global subscriber cannot be set (e.g., if logging was already initialized).
pub fn init(debug_mode: bool) -> Result<WorkerGuard> {
    let logs_path = resolve_log_path();

    create_dir_all(&logs_path).map_err(|e| format!("Failed to create log directory: {e}"))?;

    let stdout_filter = if debug_mode {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    let file_filter = LevelFilter::DEBUG;

    let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = Vec::new();

    let stdout_layer = create_stdout_layer().with_filter(stdout_filter);
    layers.push(Box::new(stdout_layer));

    let file_appender = create_file_appender(logs_path)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = create_file_layer(non_blocking).with_filter(file_filter);
    layers.push(Box::new(file_layer));

    tracing_subscriber::registry()
        .with(layers)
        .try_init()
        .map_err(|e| format!("Failed to initialize logging: {e}"))?;

    Ok(guard)
}

fn create_stdout_layer() -> impl Layer<Registry> {
    layer().compact().with_target(false).without_time()
}

fn create_file_layer<W>(writer: W) -> impl Layer<Registry>
where
    W: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
{
    layer()
        .compact()
        .with_ansi(false)
        .with_target(true)
        .with_writer(writer)
}

fn create_file_appender<P: AsRef<Path>>(path: P) -> Result<RollingFileAppender> {
    RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(env!("CARGO_PKG_NAME"))
        .filename_suffix(LOG_FILE_SUFFIX)
        .max_log_files(MAX_LOG_FILES)
        .build(path)
        .map_err(|e| format!("Failed to create file appender: {e}").into())
}

fn resolve_log_path() -> PathBuf {
    if cfg!(debug_assertions) {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        }
        path.join(".dev").join("logs")
    } else {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(env!("CARGO_PKG_NAME"))
            .join("logs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_log_path() {
        let path = resolve_log_path();
        assert!(path.ends_with("logs"));
    }

    #[test]
    fn test_create_file_appender() {
        let temp_dir = std::env::temp_dir().join(format!("weather-cli-test-logs-{}", std::process::id()));
        create_dir_all(&temp_dir).expect("Failed to create temp dir");

        let appender = create_file_appender(&temp_dir);
        assert!(appender.is_ok(), "Should successfully create file appender in a valid directory");

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}