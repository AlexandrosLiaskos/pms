use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutoGitSyncError {
    #[error("Failed to initialize Git repository: {0}")]
    GitInitError(String),

    #[error("Failed to push changes: {0}")]
    GitPushError(String),

    #[error("Failed to create GitHub repository: {0}")]
    GitHubApiError(String),

    #[error("Failed to read config file: {0}")]
    ConfigError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Failed to watch directory {path}: {error}")]
    WatchError {
        path: PathBuf,
        error: String,
    },

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Token error: {0}")]
    TokenError(String),

    #[error("Security error: {0}")]
    SecurityError(String),
}

pub type Result<T> = std::result::Result<T, AutoGitSyncError>;

// Security validation functions
pub fn validate_path(path: &PathBuf) -> Result<()> {
    // Ensure path exists
    if !path.exists() {
        return Err(AutoGitSyncError::InvalidPath(
            "Path does not exist".to_string(),
        ));
    }

    // Ensure we have read/write permissions
    let metadata = path.metadata().map_err(|e| {
        AutoGitSyncError::SecurityError(format!("Failed to read path metadata: {}", e))
    })?;

    if !metadata.is_dir() {
        return Err(AutoGitSyncError::InvalidPath(
            "Path must be a directory".to_string(),
        ));
    }

    // Check for suspicious paths
    let path_str = path.to_string_lossy().to_lowercase();
    let suspicious_paths = [
        "system32", "windows", "program files", "etc", "var", "usr",
        "bin", "sbin", "dev", "proc", "sys",
    ];

    if suspicious_paths.iter().any(|p| path_str.contains(p)) {
        return Err(AutoGitSyncError::SecurityError(
            "Cannot monitor system directories".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_token(token: &str) -> Result<()> {
    // Basic token validation
    if token.is_empty() {
        return Err(AutoGitSyncError::TokenError(
            "GitHub token cannot be empty".to_string(),
        ));
    }

    if token.len() < 40 {
        return Err(AutoGitSyncError::TokenError(
            "Invalid GitHub token format".to_string(),
        ));
    }

    // Check for common token prefixes
    if !token.starts_with("ghp_") && !token.starts_with("github_pat_") {
        return Err(AutoGitSyncError::TokenError(
            "Invalid GitHub token format: must start with 'ghp_' or 'github_pat_'".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_git_config(username: &str, email: &str) -> Result<()> {
    // Validate username
    if username.is_empty() {
        return Err(AutoGitSyncError::InvalidConfig(
            "Git username cannot be empty".to_string(),
        ));
    }

    // Basic email validation
    if !email.contains('@') || !email.contains('.') {
        return Err(AutoGitSyncError::InvalidConfig(
            "Invalid email format".to_string(),
        ));
    }

    Ok(())
}

pub fn sanitize_repo_name(name: &str) -> String {
    // Remove special characters and spaces
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();

    // Ensure it doesn't start or end with special characters
    sanitized.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_validate_path() {
        // Test valid directory
        let temp_dir = tempdir().unwrap();
        assert!(validate_path(&temp_dir.path().to_path_buf()).is_ok());

        // Test non-existent path
        let non_existent = PathBuf::from("/non/existent/path");
        assert!(validate_path(&non_existent).is_err());

        // Test suspicious paths
        let system_path = PathBuf::from("/etc/something");
        assert!(validate_path(&system_path).is_err());
    }

    #[test]
    fn test_validate_token() {
        // Test valid token
        assert!(validate_token("ghp_1234567890abcdef1234567890abcdef123456").is_ok());
        assert!(validate_token("github_pat_1234567890abcdef1234567890abcdef123456").is_ok());

        // Test invalid tokens
        assert!(validate_token("").is_err());
        assert!(validate_token("short_token").is_err());
        assert!(validate_token("invalid_prefix_1234567890abcdef1234567890abcdef123456").is_err());
    }

    #[test]
    fn test_validate_git_config() {
        // Test valid config
        assert!(validate_git_config("username", "email@example.com").is_ok());

        // Test invalid config
        assert!(validate_git_config("", "email@example.com").is_err());
        assert!(validate_git_config("username", "invalid-email").is_err());
    }

    #[test]
    fn test_sanitize_repo_name() {
        assert_eq!(sanitize_repo_name("My Project!"), "my-project");
        assert_eq!(sanitize_repo_name("test_repo"), "test_repo");
        assert_eq!(sanitize_repo_name("---test---"), "test");
        assert_eq!(sanitize_repo_name("!@#$%^"), "");
    }
}
