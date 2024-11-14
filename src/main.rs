use anyhow::{Context, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use tokio::process::Command;
use reqwest::Client;
use serde_json::json;

mod config;
use config::Config;

async fn create_github_repository(token: &str, username: &str, name: &str) -> Result<String> {
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
        if error.contains("already exists") {
            // Repository exists, just return the URL
            return Ok(format!("https://github.com/{}/{}", username, name));
        }
        anyhow::bail!("Failed to create repository: {}", error);
    }

    let repo = response.json::<serde_json::Value>().await?;
    Ok(repo["html_url"].as_str().unwrap_or_default().to_string())
}

async fn init_repository(path: &PathBuf, config: &Config) -> Result<()> {
    // Get repository name from directory name
    let repo_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid directory name"))?;

    // Initialize Git if needed
    if !path.join(".git").exists() {
        println!("Initializing Git repository...");
        Command::new("git")
            .arg("init")
            .current_dir(path)
            .output()
            .await
            .context("Failed to initialize Git repository")?;
    }

    // Set Git config
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

    // Create or get GitHub repository
    println!("Setting up GitHub repository...");
    let repo_url = create_github_repository(&config.github_token, &config.git_username, repo_name).await?;

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
    println!("Performing initial force push...");
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

    // Force push to main branch
    Command::new("git")
        .args(["push", "-f", "origin", "HEAD:main"])
        .current_dir(path)
        .output()
        .await
        .context("Failed to push initial commit")?;

    // Set upstream
    Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(path)
        .output()
        .await?;

    Command::new("git")
        .args(["branch", "--set-upstream-to=origin/main", "main"])
        .current_dir(path)
        .output()
        .await?;

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
            println!("Created example config at: {}", config_path.display());
            println!("Please edit this file with your GitHub credentials and run again.");
            return Ok(());
        }
    };

    // Initialize repository if needed
    init_repository(&path, &config).await?;

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
            .args(["push", "origin", "HEAD"])
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
