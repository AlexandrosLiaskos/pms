use crate::error::{AutoGitSyncError, Result};
use crate::git::GitHandler;
use crate::logging;
use colored::*;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::ModifyKind;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, Instant};

pub struct FileWatcher {
    path: PathBuf,
    git_handler: GitHandler,
    watcher: RecommendedWatcher,
    receiver: Receiver<notify::Result<Event>>,
    last_sync: Instant,
    sync_interval: Duration,
    waiting_for_rename: bool,
    last_modified_file: Option<PathBuf>,
    force_sync: bool,
}

impl FileWatcher {
    pub fn new(path: PathBuf, git_handler: GitHandler, sync_interval: u64) -> Result<Self> {
        let (tx, rx) = channel();
        
        let watcher = RecommendedWatcher::new(
            move |res| {
                tx.send(res).unwrap_or_else(|e| logging::error(&e.to_string()));
            },
            notify::Config::default(),
        ).map_err(|e| AutoGitSyncError::WatchError {
            path: path.clone(),
            error: e.to_string(),
        })?;

        Ok(Self {
            path,
            git_handler,
            watcher,
            receiver: rx,
            last_sync: Instant::now(),
            sync_interval: Duration::from_secs(sync_interval),
            waiting_for_rename: false,
            last_modified_file: None,
            force_sync: false,
        })
    }

    pub fn start_watching(&mut self) -> Result<()> {
        self.watcher
            .watch(&self.path, RecursiveMode::Recursive)
            .map_err(|e| AutoGitSyncError::WatchError {
                path: self.path.clone(),
                error: e.to_string(),
            })?;

        Ok(())
    }

    fn is_temp_file(path: &PathBuf) -> bool {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        file_name.ends_with(".tmp") ||
        file_name.ends_with(".TMP") ||
        file_name.contains("~RF") ||  // Office temp files
        file_name.starts_with("~") ||
        file_name == "New Text Document.txt" ||
        file_name == "New Microsoft Word Document.docx" ||
        file_name == "New Microsoft Excel Worksheet.xlsx" ||
        file_name == "New Microsoft PowerPoint Presentation.pptx" ||
        file_name.starts_with("New ")
    }

    fn should_ignore_file(path: &PathBuf) -> bool {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Ignore Git's internal files and temp files
        if file_name == "index.lock" || 
           file_name.starts_with(".git") ||
           file_name == ".DS_Store" ||
           file_name == "Thumbs.db" {
            return true;
        }

        // Check if path contains .git directory
        path.components().any(|c| c.as_os_str() == ".git")
    }

    pub async fn handle_events(&mut self) -> Result<()> {
        match self.receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(event)) => {
                // Skip ignored files
                if event.paths.iter().any(|p| Self::should_ignore_file(p)) {
                    return Ok(());
                }

                match event.kind {
                    EventKind::Create(_) => {
                        if let Some(file_path) = event.paths.first() {
                            if !Self::is_temp_file(file_path) {
                                logging::status_change(file_path, "added", Color::Yellow);
                                self.force_sync = true;
                            } else {
                                self.waiting_for_rename = true;
                            }
                        }
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    },
                    EventKind::Modify(ModifyKind::Name(_)) => {
                        if let Some(file_path) = event.paths.get(1) {
                            if self.waiting_for_rename && !Self::is_temp_file(file_path) {
                                logging::status_change(file_path, "added", Color::Yellow);
                                self.force_sync = true;
                                self.waiting_for_rename = false;
                            } else if !self.waiting_for_rename {
                                logging::status_change(file_path, "renamed", Color::BrightBlue);
                            }
                        }
                    },
                    EventKind::Remove(_) => {
                        if let Some(file_path) = event.paths.first() {
                            if !Self::is_temp_file(file_path) {
                                logging::status_change(file_path, "deleted", Color::Red);
                            }
                        }
                    },
                    EventKind::Modify(_) => {
                        if let Some(file_path) = event.paths.first() {
                            if !self.waiting_for_rename && !Self::is_temp_file(file_path) {
                                if self.last_modified_file.as_ref() != Some(file_path) {
                                    logging::status_change(file_path, "modified", Color::Blue);
                                    self.last_modified_file = Some(file_path.clone());
                                }
                            }
                        }
                    },
                    _ => {}
                }

                self.try_sync().await?;
            },
            Ok(Err(e)) => {
                logging::error(&format!("Watch error: {}", e));
            },
            Err(_) => {
                // Timeout - check if we need to sync
                self.try_sync().await?;
            }
        }

        Ok(())
    }

    async fn try_sync(&mut self) -> Result<()> {
        if self.force_sync || (!self.waiting_for_rename && self.last_sync.elapsed() >= self.sync_interval) {
            if self.git_handler.sync_changes().await? {
                self.last_sync = Instant::now();
                self.force_sync = false;
                self.last_modified_file = None;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_file_watcher() {
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

        let git_handler = GitHandler::new(temp_dir.path().to_path_buf(), config);
        let mut watcher = FileWatcher::new(
            temp_dir.path().to_path_buf(),
            git_handler,
            1,
        ).unwrap();

        // Start watching
        watcher.start_watching().unwrap();

        // Create a test file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        // Give some time for events to be processed
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}
