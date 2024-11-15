use crate::error::{AutoGitSyncError, Result};
use crate::git::GitHandler;
use crate::watcher::FileWatcher;
use std::path::PathBuf;
use tokio::signal;

mod config;
mod error;
mod git;
mod logging;
mod watcher;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get directory to monitor from command line
    let path = get_target_directory()?;

    // Validate path
    error::validate_path(&path)?;

    // Load config
    let config = match config::Config::load() {
        Ok(config) => config,
        Err(_) => {
            let config_path = config::Config::save_example()?;
            logging::info(&format!("Created example config at: {}", config_path.display()));
            logging::info("Please edit this file with your GitHub credentials and run again.");
            return Ok(());
        }
    };

    // Initialize Git handler
    let git_handler = GitHandler::new(path.clone(), config.clone());
    git_handler.init_repository().await?;

    // Get repository name for startup message
    let repo_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed-project");

    logging::startup_message(&path, &config.git_username, repo_name);

    // Initialize file watcher
    let mut watcher = FileWatcher::new(path, git_handler, config.sync_interval)?;
    watcher.start_watching()?;

    // Handle Ctrl+C gracefully
    tokio::select! {
        result = watch_loop(&mut watcher) => {
            if let Err(e) = result {
                logging::error(&format!("Error in watch loop: {}", e));
                return Err(e);
            }
        }
        _ = handle_shutdown_signal() => {
            logging::info("Received shutdown signal, stopping...");
        }
    }

    Ok(())
}

fn get_target_directory() -> Result<PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        std::env::current_dir().map_err(|e| {
            AutoGitSyncError::InvalidPath(format!("Failed to get current directory: {}", e))
        })?
    };

    Ok(path)
}

async fn watch_loop(watcher: &mut FileWatcher) -> Result<()> {
    loop {
        if let Err(e) = watcher.handle_events().await {
            logging::error(&format!("Error handling events: {}", e));
            // Continue watching even if we encounter errors
            continue;
        }
    }
}

async fn handle_shutdown_signal() {
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
        .expect("Failed to register SIGINT handler");
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("Failed to register SIGTERM handler");

    tokio::select! {
        _ = sigint.recv() => {},
        _ = sigterm.recv() => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_get_target_directory() {
        // Test with no arguments
        let result = get_target_directory();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::env::current_dir().unwrap());

        // Test with argument
        std::env::set_args(vec!["program".to_string(), "/tmp".to_string()].into_iter());
        let result = get_target_directory();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/tmp"));
    }

    #[tokio::test]
    async fn test_initialization() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().to_path_buf();

        // Create test config
        let config = config::Config {
            config_path: PathBuf::new(),
            github_token: secrecy::Secret::new("test_token".to_string()),
            git_username: "test-user".to_string(),
            git_email: "test@example.com".to_string(),
            sync_interval: 2,
            batch_size: 10,
            security: config::SecurityConfig::default(),
        };

        // Test Git handler initialization
        let git_handler = GitHandler::new(path.clone(), config.clone());
        assert!(git_handler.init_repository().await.is_ok());

        // Test watcher initialization
        let watcher = FileWatcher::new(path, git_handler, config.sync_interval);
        assert!(watcher.is_ok());
    }
}
