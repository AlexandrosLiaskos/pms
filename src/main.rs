use crate::error::{AutoGitSyncError, Result};
use crate::git::GitHandler;
use crate::watcher::FileWatcher;
use std::path::PathBuf;
use tokio::sync::oneshot;
use std::process;

mod config;
mod error;
mod git;
mod logging;
mod watcher;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let shutdown_tx = std::sync::Arc::new(std::sync::Mutex::new(Some(shutdown_tx)));
    
    let shutdown_tx_clone = shutdown_tx.clone();
    ctrlc::set_handler(move || {
        logging::info("Received shutdown signal, stopping...");
        if let Some(tx) = shutdown_tx_clone.lock().unwrap().take() {
            let _ = tx.send(());
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let path = get_target_directory()?;

    error::validate_path(&path)?;

    let config = config::Config::load()?;

    let git_handler = GitHandler::new(path.clone(), config.clone());
    git_handler.init_repository().await?;

    let repo_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed-project");

    logging::startup_message(&path, &config.git_username, repo_name);

    let mut watcher = FileWatcher::new(path, git_handler, config.sync_interval)?;
    watcher.start_watching()?;

    match watch_loop(&mut watcher, shutdown_rx).await {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            logging::error(&format!("Error in watch loop: {}", e));
            process::exit(1);
        }
    }
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

async fn watch_loop(watcher: &mut FileWatcher, mut shutdown_rx: oneshot::Receiver<()>) -> Result<()> {
    loop {
        tokio::select! {
            result = watcher.handle_events() => {
                if let Err(e) = result {
                    logging::error(&format!("Error handling events: {}", e));
                    continue;
                }
            }
            _ = &mut shutdown_rx => {
                if let Err(e) = watcher.sync_pending_changes().await {
                    logging::error(&format!("Error syncing final changes: {}", e));
                }
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_get_target_directory() {
        let result = get_target_directory();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::env::current_dir().unwrap());

        std::env::set_args(vec!["program".to_string(), "/tmp".to_string()].into_iter());
        let result = get_target_directory();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/tmp"));
    }

    #[tokio::test]
    async fn test_initialization() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().to_path_buf();

        let config = config::Config {
            config_path: PathBuf::new(),
            github_token: secrecy::Secret::new("test_token".to_string()),
            git_username: "test-user".to_string(),
            git_email: "test@example.com".to_string(),
            sync_interval: 2,
            batch_size: 10,
            security: config::SecurityConfig::default(),
        };

        let git_handler = GitHandler::new(path.clone(), config.clone());
        assert!(git_handler.init_repository().await.is_ok());

        let watcher = FileWatcher::new(path, git_handler, config.sync_interval);
        assert!(watcher.is_ok());
    }
}
