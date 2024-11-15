use crate::error::{PMSError, Result};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use zeroize::Zeroize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(skip)]
    config_path: PathBuf,
    
    #[serde(serialize_with = "serialize_secret", deserialize_with = "deserialize_secret")]
    github_token: Secret<String>,
    pub git_username: String,
    pub git_email: String,

    #[serde(default = "default_sync_interval")]
    pub sync_interval: u64,

    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    #[serde(default)]
    pub security: SecurityConfig,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SecurityConfig {
    #[serde(default = "default_ignore_patterns")]
    pub ignore_patterns: Vec<String>,
    
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
    
    #[serde(default)]
    pub allow_force_push: bool,
    
    #[serde(default = "default_token_refresh_days")]
    pub token_refresh_days: u32,
}

fn default_sync_interval() -> u64 { 2 }
fn default_batch_size() -> usize { 10 }
fn default_max_file_size() -> u64 { 100 * 1024 * 1024 } 
fn default_token_refresh_days() -> u32 { 90 }

fn default_ignore_patterns() -> Vec<String> {
    vec![
        String::from("*.env"),
        String::from("*.key"),
        String::from("*.pem"),
        String::from("id_rsa"),
        String::from("id_rsa.pub"),
        String::from("*.log"),
    ]
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            return Err(PMSError::ConfigError(
                "Config file not found. Please create a config file at ~/.config/pms/config.toml".to_string(),
            ).into());
        }

        let content = fs::read_to_string(&config_path).map_err(|e| {
            PMSError::ConfigError(format!("Failed to read config: {}", e))
        })?;

        let mut config: Config = toml::from_str(&content).map_err(|e| {
            PMSError::ConfigError(format!("Invalid config format: {}", e))
        })?;

        config.config_path = config_path;
        config.validate()?;

        Ok(config)
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| PMSError::ConfigError("Could not find config directory".to_string()))?
            .join("pms");

        Ok(config_dir.join("config.toml"))
    }

    pub fn validate(&self) -> Result<()> {
        crate::error::validate_token(self.github_token.expose_secret())?;
        crate::error::validate_git_config(&self.git_username, &self.git_email)?;

        if self.sync_interval < 1 {
            return Err(PMSError::InvalidConfig(
                "Sync interval must be at least 1 second".to_string(),
            ).into());
        }

        if self.batch_size < 1 {
            return Err(PMSError::InvalidConfig(
                "Batch size must be at least 1".to_string(),
            ).into());
        }

        Ok(())
    }

    pub fn get_token(&self) -> &Secret<String> {
        &self.github_token
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        // Create a mutable copy for zeroizing
        let mut token = self.github_token.expose_secret().to_string();
        token.zeroize();
    }
}

fn serialize_secret<S>(secret: &Secret<String>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

fn deserialize_secret<'de, D>(deserializer: D) -> std::result::Result<Secret<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Secret::new(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_validation() {
        let config = Config {
            config_path: PathBuf::new(),
            github_token: Secret::new("ghp_1234567890abcdef1234567890abcdef123456".to_string()),
            git_username: "test-user".to_string(),
            git_email: "test@example.com".to_string(),
            sync_interval: 2,
            batch_size: 10,
            security: SecurityConfig::default(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config() {
        let config = Config {
            config_path: PathBuf::new(),
            github_token: Secret::new("invalid-token".to_string()),
            git_username: "".to_string(),
            git_email: "invalid-email".to_string(),
            sync_interval: 0,
            batch_size: 0,
            security: SecurityConfig::default(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_security_config() {
        let config = SecurityConfig::default();
        assert!(!config.ignore_patterns.is_empty());
        assert!(config.ignore_patterns.contains(&"*.env".to_string()));
        assert_eq!(config.max_file_size, 100 * 1024 * 1024);
        assert_eq!(config.token_refresh_days, 90);
    }
}
