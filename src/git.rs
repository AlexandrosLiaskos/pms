use crate::config::Config;
use crate::error::{AutoGitSyncError, Result};
use secrecy::ExposeSecret;
use std::path::PathBuf;
use tokio::process::Command;

pub struct GitHandler {
    repo_path: PathBuf,
    config: Config,
}

impl GitHandler {
    pub fn new(repo_path: PathBuf, config: Config) -> Self {
        Self { repo_path, config }
    }

    pub async fn init_repository(&self) -> Result<()> {
        // Get repository name
        let repo_name = self.repo_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| AutoGitSyncError::InvalidPath("Invalid directory name".to_string()))?;

        let repo_name = crate::error::sanitize_repo_name(repo_name);
        if repo_name.is_empty() {
            return Err(AutoGitSyncError::InvalidPath(
                "Repository name is empty after sanitization".to_string(),
            ));
        }

        // Initialize Git if needed
        if !self.repo_path.join(".git").exists() {
            self.execute_git(&["init"])
                .await
                .context("Failed to initialize Git repository")?;
        }

        // Configure Git
        self.execute_git(&["config", "user.name", &self.config.git_username])
            .await
            .context("Failed to set git username")?;

        self.execute_git(&["config", "user.email", &self.config.git_email])
            .await
            .context("Failed to set git email")?;

        // Create GitHub repository
        self.create_github_repository(&repo_name).await?;

        // Set up remote
        self.execute_git(&["remote", "remove", "origin"])
            .await
            .ok(); // Ignore error if remote doesn't exist

        let remote_url = format!(
            "https://{}@github.com/{}/{}",
            self.config.get_token().expose_secret(),
            self.config.git_username,
            repo_name
        );

        self.execute_git(&["remote", "add", "origin", &remote_url])
            .await
            .context("Failed to add remote")?;

        // Initial commit and push
        self.execute_git(&["add", "."]).await?;
        
        let _ = self.execute_git(&["commit", "-m", "Initial commit"])
            .await
            .context("Failed to create initial commit")?;

        self.execute_git(&["branch", "-M", "main"])
            .await?;

        self.execute_git(&["push", "-f", "origin", "main"])
            .await
            .context("Failed to push initial commit")?;

        Ok(())
    }

    pub async fn sync_changes(&self) -> Result<bool> {
        // Add all changes
        self.execute_git(&["add", "."]).await?;

        // Check for changes
        let status = self.execute_git(&["status", "--porcelain"])
            .await
            .context("Failed to check git status")?;

        if status.is_empty() {
            return Ok(false);
        }

        // Create commit
        self.execute_git(&["commit", "-m", "Auto-sync update"])
            .await
            .context("Failed to create commit")?;

        // Reset main branch to current HEAD for clean history
        self.execute_git(&["branch", "-f", "main", "HEAD"])
            .await
            .context("Failed to reset main branch")?;

        // Force push changes
        self.execute_git(&["push", "-f", "origin", "main"])
            .await
            .context("Failed to push changes")?;

        Ok(true)
    }

    async fn execute_git(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| AutoGitSyncError::GitInitError(e.to_string()))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AutoGitSyncError::GitInitError(error.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn create_github_repository(&self, name: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.github.com/user/repos")
            .header(
                "Authorization",
                format!("Bearer {}", self.config.get_token().expose_secret()),
            )
            .header("User-Agent", "auto-git-sync")
            .json(&serde_json::json!({
                "name": name,
                "private": true,
                "auto_init": false,
            }))
            .send()
            .await
            .map_err(|e| AutoGitSyncError::GitHubApiError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            
            // Ignore if repository already exists
            if !error.contains("already exists") {
                return Err(AutoGitSyncError::GitHubApiError(error));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_git_handler() {
        let temp_dir = tempdir().unwrap();
        let config = Config {
            config_path: PathBuf::new(),
            github_token: secrecy::Secret::new("test_token".to_string()),
            git_username: "test-user".to_string(),
            git_email: "test@example.com".to_string(),
            sync_interval: 2,
            batch_size: 10,
            security: crate::config::SecurityConfig::default(),
        };

        let handler = GitHandler::new(temp_dir.path().to_path_buf(), config);

        // Test execute_git
        let result = handler.execute_git(&["--version"]).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("git version"));
    }
}
