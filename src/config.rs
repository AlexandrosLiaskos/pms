use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub github_token: String,
    pub git_username: String,
    pub git_email: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("auto-git-sync");

        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "Config file not found. Please create {} with your GitHub token and credentials",
                config_path.display()
            ));
        }

        let content = fs::read_to_string(config_path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_example() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("auto-git-sync");

        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.toml");

        let example = Config {
            github_token: "your_github_token".to_string(),
            git_username: "your_name".to_string(),
            git_email: "your_email".to_string(),
        };

        let content = toml::to_string_pretty(&example)?;
        fs::write(&config_path, content)?;

        Ok(config_path)
    }
}
