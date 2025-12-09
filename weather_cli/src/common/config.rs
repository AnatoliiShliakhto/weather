use crate::{common::*, models::config::Settings};
use ::std::{
    fs,
    io::{self, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    sync::{Arc, RwLock, RwLockReadGuard},
};
use ::tracing::debug;

/// Application configuration manager.
///
/// Provides thread-safe access to `Settings`, handling automatic loading on startup
/// and atomic saving to disk upon modification.
#[derive(Clone)]
pub struct AppConfig {
    /// Path to the configuration file.
    settings_file: Arc<PathBuf>,
    /// Current settings protected by a read-write lock.
    settings: Arc<RwLock<Settings>>,
}

impl AppConfig {
    /// Creates a new `AppConfig` instance by loading settings from the specified file.
    ///
    /// If the file is not found or corrupted, default settings (`Settings::default()`) are used.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON configuration file.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        use io::ErrorKind;

        let path = path.into();

        let settings = match load_file(&path) {
            Ok(s) => s,
            Err(Error::Io(e)) if e.kind() == ErrorKind::NotFound => {
                debug!("Config file not found at {path:?}, using default.");
                Settings::default()
            }
            Err(e) => {
                debug!(
                    "Failed to load config file.\n\
                    \t{e}\n\
                    \tUsing default settings."
                );
                Settings::default()
            }
        };

        Self {
            settings_file: Arc::new(path),
            settings: Arc::new(RwLock::new(settings)),
        }
    }

    /// Acquires a read lock for the settings.
    ///
    /// Returns an `RwLockReadGuard` allowing read access to the settings fields.
    /// Blocks the current thread if the settings are currently being updated.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned due to a panic in another thread.
    pub fn get(&self) -> Result<RwLockReadGuard<'_, Settings>> {
        self.settings
            .read()
            .map_err(|e| format!("Config read lock poisoned: {e:?}").into())
    }

    /// Modifies settings and atomically saves them to disk.
    ///
    /// Provides mutable access to `Settings` within the given closure.
    /// After the closure executes, the settings are automatically serialized and saved
    /// to the file using an atomic writing strategy (write to tmp + rename).
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a mutable reference to `Settings`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The write lock could not be acquired.
    /// * An I/O error occurred while saving the file.
    /// * A JSON serialization error occurred.
    pub fn with_mut<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Settings) -> R,
    {
        let mut settings_guard = self
            .settings
            .write()
            .map_err(|e| format!("Config write lock poisoned: {e:?}"))?;

        let result = f(&mut settings_guard);

        save_file_atomic(&settings_guard, &self.settings_file)?;

        Ok(result)
    }
}

fn load_file(path: &Path) -> Result<Settings> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let settings = serde_json::from_reader(reader)?;
    Ok(settings)
}

fn save_file_atomic(settings: &Settings, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_path = path.with_extension("tmp");

    {
        let file = fs::File::create(&tmp_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, settings)?;
        writer.flush()?;
        writer.get_ref().sync_all()?;
    }

    fs::rename(&tmp_path, path).inspect_err(|_| {
        if let Err(e) = fs::remove_file(&tmp_path) {
            debug!("Failed to remove temporary file: {e:?}")
        }
    })?;

    Ok(())
}
