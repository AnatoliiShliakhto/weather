use super::config::AppConfig;
use ::std::{path::PathBuf, sync::LazyLock};

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(AppState::new);

pub struct AppState {
    pub config: AppConfig,
}

// pub type SharedState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn new() -> Self {
        let config_file_file = resolve_config_file();

        let config = AppConfig::new(config_file_file);

        Self { config }
    }
}

fn resolve_config_file() -> PathBuf {
    if cfg!(debug_assertions) {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        }
        path.join(".dev").join("config.json")
    } else {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(env!("CARGO_PKG_NAME"))
            .join("config.json")
    }
}
