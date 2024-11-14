use anyhow::{Context, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use tokio::process::Command;

mod config;
use config::Config;

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
            println!("Created example config at: {}", config_path.display());
            println!("Please edit this file with your GitHub credentials and run again.");
            return Ok(());
        }
    };

    // Set up Git credentials
    Command::new("git")
        .args(["config", "user.name", &config.git_username])
        .current_dir(&path)
        .output()
        .await
        .context("Failed to set git username")?;

    Command::new("git")
        .args(["config", "user.email", &config.git_email])
        .current_dir(&path)
        .output()
        .await
        .context("Failed to set git email")?;

    // Update origin URL to include token
    if let Ok(output) = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(&path)
        .output()
        .await
    {
        if let Ok(url) = String::from_utf8(output.stdout) {
            let url = url.trim();
            if url.starts_with("https://github.com") {
                let new_url = format!(
                    "https://{}@github.com/{}",
                    config.github_token,
                    url.trim_start_matches("https://github.com/")
                );
                Command::new("git")
                    .args(["remote", "set-url", "origin", &new_url])
                    .current_dir(&path)
                    .output()
                    .await
                    .context("Failed to update remote URL")?;
            }
        }
    }

    println!("Monitoring directory: {}", path.display());

    // Set up file system watcher
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                tx.send(event).unwrap_or_else(|e| println!("Error: {}", e));
            }
        },
        notify::Config::default(),
    )?;

    // Start watching the directory
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    // Keep track of last sync
    let mut last_sync = Instant::now();
    let sync_interval = Duration::from_secs(2);

    println!("Auto-sync started. Press Ctrl+C to stop.");

    // Main event loop
    loop {
        if rx.recv_timeout(Duration::from_secs(1)).is_ok() {
            // Wait for sync interval before pushing changes
            if last_sync.elapsed() >= sync_interval {
                sync_changes(&path).await?;
                last_sync = Instant::now();
            }
        }
    }
}

async fn sync_changes(path: &PathBuf) -> Result<()> {
    println!("Checking for changes...");

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
        println!("Changes detected, creating commit...");
        
        // Create commit
        Command::new("git")
            .args(["commit", "-m", "Auto-sync update"])
            .current_dir(path)
            .output()
            .await
            .context("Failed to create commit")?;

        println!("Pushing changes...");
        
        // Push changes
        let output = Command::new("git")
            .args(["push"])
            .current_dir(path)
            .output()
            .await
            .context("Failed to push changes")?;

        if !output.status.success() {
            println!("Push failed: {}", String::from_utf8_lossy(&output.stderr));
        } else {
            println!("Changes pushed successfully");
        }
    }

    Ok(())
}
