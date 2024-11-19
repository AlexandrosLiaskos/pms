use clap::Parser;
use crate::error::{PMSError, Result};
use crate::git::GitHandler;
use crate::watcher::FileWatcher;
use std::path::PathBuf;
use tokio::sync::oneshot;
use std::process;

mod cli;
mod config;
mod error;
mod git;
mod logging;
mod watcher;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Parse command line arguments
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Watch { path, verbose } => {
            watch_directory(path, verbose).await?;
        }
        cli::Commands::Init { path, name, verbose } => {
            init_project(path, name, verbose).await?;
        }
        cli::Commands::Config { token, username, email } => {
            configure_settings(token, username, email).await?;
        }
    }

    Ok(())
}

async fn watch_directory(path: PathBuf, verbose: bool) -> Result<()> {
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

    error::validate_path(&path)?;

    let config = config::Config::load()?;

    let mut git_handler = GitHandler::new(path.clone(), config.clone());
    git_handler.set_verbose(verbose);
    git_handler.init_repository().await?;

    let repo_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed-project");

    logging::startup_message(&path, &config.git_username, repo_name);

    let mut watcher = FileWatcher::new(path, git_handler, config.sync_interval)?;
    watcher.start_watching()?;

    watch_loop(&mut watcher, shutdown_rx).await
}

async fn init_project(path: PathBuf, name: Option<String>, verbose: bool) -> Result<()> {
    let config = config::Config::load()?;
    
    let mut git_handler = GitHandler::new(path.clone(), config);
    git_handler.set_verbose(verbose);
    
    if let Some(project_name) = name {
        git_handler.set_project_name(&project_name);
    }
    
    git_handler.init_repository().await
}

async fn configure_settings(
    token: Option<String>,
    username: Option<String>,
    email: Option<String>,
) -> Result<()> {
    let mut config = config::Config::load().unwrap_or_default();
    
    if let Some(token) = token {
        config.set_github_token(token)?;
    }
    
    if let Some(username) = username {
        config.set_git_username(username);
    }
    
    if let Some(email) = email {
        config.set_git_email(email)?;
    }
    
    config.save()?;
    logging::success("Configuration updated successfully");
    Ok(())
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
