use anyhow::Context;
use crate::config::Config;
use crate::error::{AutoGitSyncError, Result};
use secrecy::ExposeSecret;
use std::path::PathBuf;
use tokio::process::Command;
use std::fs;
use crate::logging;

pub struct GitHandler {
    repo_path: PathBuf,
    config: Config,
    verbose: bool,
}

impl GitHandler {
    pub fn new(repo_path: PathBuf, config: Config) -> Self {
        Self { 
            repo_path, 
            config,
            verbose: false,  
        }
    }

    fn log_git(&self, operation: &str) {
        if self.verbose {
            logging::git_operation(operation);
        }
    }

    pub async fn init_repository(&self) -> Result<()> {
        logging::init_message("Initializing Git repository");
        
        // Get repository name
        let repo_name = self.repo_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| AutoGitSyncError::InvalidPath("Invalid directory name".to_string()))?;

        let repo_name = crate::error::sanitize_repo_name(repo_name);
        if repo_name.is_empty() {
            return Err(AutoGitSyncError::InvalidPath(
                "Repository name is empty after sanitization".to_string(),
            ).into());
        }

        // Initialize Git if needed
        if !self.repo_path.join(".git").exists() {
            self.log_git("init");
            self.execute_git(&["init"])
                .await
                .context("Failed to initialize Git repository")?;
        }

        // Configure Git
        self.log_git("config");
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

        self.log_git("remote add");
        self.execute_git(&["remote", "add", "origin", &remote_url])
            .await
            .context("Failed to add remote")?;

        // Create initial README if directory is empty
        let readme_path = self.repo_path.join("README.md");
        if !readme_path.exists() {
            fs::write(&readme_path, format!("# {}\n\nAutomatically synced with auto-git-sync", repo_name))
                .map_err(|e| AutoGitSyncError::GitInitError(format!("Failed to create README: {}", e)))?;
        }

        // Initial commit and push
        self.log_git("add");
        self.execute_git(&["add", "."]).await?;
        
        self.log_git("commit");
        let _ = self.execute_git(&["commit", "-m", "Initial commit"])
            .await
            .context("Failed to create initial commit")?;

        self.log_git("branch");
        self.execute_git(&["branch", "-M", "main"])
            .await?;

        self.log_git("push");
        self.execute_git(&["push", "-f", "origin", "main"])
            .await
            .map_err(|e| AutoGitSyncError::GitPushError(e.to_string()))
            .context("Failed to push initial commit")?;

        logging::success("Repository initialized successfully");
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

        // Get current branch name
        let current_branch = self.execute_git(&["rev-parse", "--abbrev-ref", "HEAD"])
            .await
            .context("Failed to get current branch")?
            .trim()
            .to_string();

        // If we're not on main branch, create/switch to it
        if current_branch != "main" {
            // Try to create main branch
            let result = self.execute_git(&["branch", "main"]).await;
            if result.is_err() {
                // Branch might already exist, try to switch to it
                self.execute_git(&["checkout", "main"])
                    .await
                    .context("Failed to switch to main branch")?;
            }
        }

        // Get the latest commit hash
        let commit_hash = self.execute_git(&["rev-parse", "HEAD"])
            .await
            .context("Failed to get commit hash")?
            .trim()
            .to_string();

        // Reset main branch to this commit
        self.execute_git(&["update-ref", "refs/heads/main", &commit_hash])
            .await
            .context("Failed to update main branch")?;

        // Force push changes
        self.execute_git(&["push", "-f", "origin", "main"])
            .await
            .map_err(|e| AutoGitSyncError::GitPushError(e.to_string()))
            .context("Failed to push changes")?;

        logging::success("Changes synced ✓");
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
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let error = if stderr.is_empty() { stdout } else { stderr };
            return Err(AutoGitSyncError::GitInitError(format!("Git command failed: {} ({})", error, args.join(" "))).into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn create_github_repository(&self, name: &str) -> Result<()> {
        self.log_git("create repository");
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
            .map_err(|e| AutoGitSyncError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            
            // Ignore if repository already exists
            if !error.contains("already exists") {
                logging::warning("Failed to create GitHub repository");
                return Err(AutoGitSyncError::GitHubApiError(error).into());
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
