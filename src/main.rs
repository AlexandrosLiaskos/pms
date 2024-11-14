use anyhow::{Context, Result};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::ModifyKind;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use tokio::process::Command;
use reqwest::Client;
use serde_json::json;
use colored::*;

mod config;
mod logging;
use config::Config;

fn should_ignore_file(path: &PathBuf) -> bool {
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // Ignore Git's internal files
    if file_name == "index.lock" || 
       file_name.starts_with(".git") ||
       file_name == ".DS_Store" ||
       file_name == "Thumbs.db" {
        return true;
    }

    // Check if path contains .git directory
    path.components().any(|c| c.as_os_str() == ".git")
}

async fn create_github_repository(token: &str, name: &str) -> Result<()> {
    let client = Client::new();
    let response = client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "auto-git-sync")
        .json(&json!({
            "name": name,
            "private": true,
            "auto_init": false,
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await?;
        if !error.contains("already exists") {
            anyhow::bail!("Failed to create repository: {}", error);
        }
        logging::warning("Repository already exists, using existing one");
    }

    Ok(())
}

async fn init_repository(path: &PathBuf, config: &Config) -> Result<()> {
    // Get repository name from directory name
    let repo_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid directory name"))?;

    // Initialize Git if needed
    if !path.join(".git").exists() {
        logging::init_message("Initializing Git repository...");
        Command::new("git")
            .arg("init")
            .current_dir(path)
            .output()
            .await
            .context("Failed to initialize Git repository")?;
    } else {
        logging::warning("Using existing Git repository");
    }

    // Set Git config
    logging::init_message("Configuring Git credentials...");
    Command::new("git")
        .args(["config", "user.name", &config.git_username])
        .current_dir(path)
        .output()
        .await
        .context("Failed to set git username")?;

    Command::new("git")
        .args(["config", "user.email", &config.git_email])
        .current_dir(path)
        .output()
        .await
        .context("Failed to set git email")?;

    // Create GitHub repository if it doesn't exist
    logging::init_message("Setting up GitHub repository...");
    create_github_repository(&config.github_token, repo_name).await?;

    // Set up remote
    Command::new("git")
        .args(["remote", "remove", "origin"])
        .current_dir(path)
        .output()
        .await
        .ok(); // Ignore error if remote doesn't exist

    let remote_url = format!(
        "https://{}@github.com/{}/{}",
        config.github_token,
        config.git_username,
        repo_name
    );

    Command::new("git")
        .args(["remote", "add", "origin", &remote_url])
        .current_dir(path)
        .output()
        .await
        .context("Failed to add remote")?;

    // Force push all files initially
    logging::git_operation("Initial commit...");
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .output()
        .await?;

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(path)
        .output()
        .await?;

    // Set branch to main
    Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(path)
        .output()
        .await?;

    // Force push to main branch
    Command::new("git")
        .args(["push", "-f", "origin", "main"])
        .current_dir(path)
        .output()
        .await
        .context("Failed to push initial commit")?;

    logging::success("Repository initialized successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get directory to monitor from command line
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        std::env::current_dir()?
    };

    // Load config or create example
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            let config_path = Config::save_example()?;
            logging::info(&format!("Created example config at: {}", config_path.display()));
            logging::info("Please edit this file with your GitHub credentials and run again.");
            return Ok(());
        }
    };

    // Initialize repository if needed
    init_repository(&path, &config).await?;

    // Get repository name for startup message
    let repo_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed-project");

    logging::startup_message(&path, &config.git_username, repo_name);

    // Set up file system watcher
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                tx.send(event).unwrap_or_else(|e| logging::error(&e.to_string()));
            }
        },
        notify::Config::default(),
    )?;

    // Start watching the directory
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    // Keep track of last sync
    let mut last_sync = Instant::now();
    let sync_interval = Duration::from_secs(2);
    let mut waiting_for_rename = false;
    let mut last_created_file: Option<PathBuf> = None;

    // Main event loop
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                // Skip Git's internal files
                if event.paths.iter().any(|p| should_ignore_file(p)) {
                    continue;
                }

                match event.kind {
                    EventKind::Create(_) => {
                        if let Some(file_path) = event.paths.first() {
                            last_created_file = Some(file_path.clone());
                            if !file_path.to_string_lossy().contains("New Text Document") {
                                logging::status_change(file_path, "added", Color::BrightGreen);
                            }
                        }
                        waiting_for_rename = true;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    },
                    EventKind::Modify(ModifyKind::Name(_)) => {
                        if let Some(file_path) = event.paths.get(1) {
                            if let Some(old_path) = &last_created_file {
                                if old_path.to_string_lossy().contains("New Text Document") {
                                    logging::status_change(file_path, "added", Color::BrightGreen);
                                } else {
                                    logging::status_change(file_path, "renamed", Color::Yellow);
                                }
                            } else {
                                logging::status_change(file_path, "renamed", Color::Yellow);
                            }
                        }
                        waiting_for_rename = false;
                        last_created_file = None;
                        if last_sync.elapsed() >= sync_interval {
                            sync_changes(&path).await?;
                            last_sync = Instant::now();
                        }
                    },
                    EventKind::Remove(_) => {
                        if let Some(file_path) = event.paths.first() {
                            logging::status_change(file_path, "deleted", Color::Red);
                        }
                        if !waiting_for_rename && last_sync.elapsed() >= sync_interval {
                            sync_changes(&path).await?;
                            last_sync = Instant::now();
                        }
                    },
                    EventKind::Modify(_) => {
                        if let Some(file_path) = event.paths.first() {
                            if !waiting_for_rename {
                                logging::status_change(file_path, "modified", Color::Blue);
                            }
                        }
                        if !waiting_for_rename && last_sync.elapsed() >= sync_interval {
                            sync_changes(&path).await?;
                            last_sync = Instant::now();
                        }
                    },
                    _ => {
                        if !waiting_for_rename && last_sync.elapsed() >= sync_interval {
                            sync_changes(&path).await?;
                            last_sync = Instant::now();
                        }
                    }
                }
            },
            Err(_) => {
                // No events for 100ms
                if !waiting_for_rename && last_sync.elapsed() >= sync_interval {
                    sync_changes(&path).await?;
                    last_sync = Instant::now();
                }
            }
        }
    }
}

async fn sync_changes(path: &PathBuf) -> Result<()> {
    // Add all changes
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .output()
        .await
        .context("Failed to stage changes")?;

    // Check if there are changes to commit
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()
        .await
        .context("Failed to check git status")?;

    if !status.stdout.is_empty() {
        // Create commit
        Command::new("git")
            .args(["commit", "-m", "Auto-sync update"])
            .current_dir(path)
            .output()
            .await
            .context("Failed to create commit")?;

        // Reset main branch to current HEAD
        Command::new("git")
            .args(["branch", "-f", "main", "HEAD"])
            .current_dir(path)
            .output()
            .await
            .context("Failed to reset main branch")?;

        // Force push changes
        let output = Command::new("git")
            .args(["push", "-f", "origin", "main"])
            .current_dir(path)
            .output()
            .await
            .context("Failed to push changes")?;

        if !output.status.success() {
            logging::error(&String::from_utf8_lossy(&output.stderr));
            return Err(anyhow::anyhow!("Failed to push changes"));
        } else {
            logging::success("Changes pushed successfully ✓");
        }
    }

    Ok(())
}
