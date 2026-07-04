use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let text = toml::to_string(self)?;
        fs::write(path, text)?;

        Ok(())
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        let text = fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&text)?;

        Ok(cfg)
    }
}
