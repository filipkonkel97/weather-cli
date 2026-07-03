use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    fn config_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "your-name", "weather-cli")
            .expect("Cannot determine config directory");

        proj_dirs.config_dir().join("config.toml")
    }
}
