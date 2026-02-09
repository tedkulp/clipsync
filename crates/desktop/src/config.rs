use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_url: Option<String>,
    pub shared_secret: Option<String>,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default)]
    pub start_minimized: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_url: None,
            shared_secret: None,
            autostart: false,
            start_minimized: false,
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        Ok(config_dir.join("clipsync").join("config.json"))
    }
}
