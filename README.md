# Auto Git Sync

A professional Git auto-sync tool with real-time file monitoring and colored status output.

## Features

- 🚀 Automatic GitHub repository creation
- 🔄 Real-time file monitoring
- 🎨 Colored status output
- 📊 Detailed change tracking
- 🔒 Secure token-based authentication
- ⚡ Fast and efficient syncing

## Setup

1. Build the project:
```bash
cargo build
```

2. Run once to create example config:
```bash
./target/debug/auto-git-sync
```

3. Get a GitHub token:
   - Go to GitHub.com → Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Click "Generate new token (classic)"
   - Give it a name (e.g., "auto-git-sync")
   - Select the "repo" scope (Full control of private repositories)
   - Click "Generate token"
   - Copy the token immediately (you won't see it again)

4. Edit the config file at `~/.config/auto-git-sync/config.toml`:
```toml
github_token = "ghp_xxxxxxxxxxxxxxxxxxxx"  # Your GitHub token
git_username = "YourGitHubUsername"
git_email = "your.email@example.com"
```

## Usage

Simply run in any directory:
```bash
./target/debug/auto-git-sync
```

The tool will:
- Initialize Git if needed
- Create GitHub repository automatically
- Set up remote with proper authentication
- Monitor for file changes with status display:
  - 🟢 Added files (green)
  - 🔵 Modified files (blue)
  - 🟡 Renamed files (yellow)
  - 🔴 Deleted files (red)
- Auto-commit and push changes

## Example Output

```
12:34:56 STARTUP Auto Git Sync
12:34:56 STARTUP Monitoring directory: /path/to/project
12:34:56 STARTUP Press Ctrl+C to stop

12:34:57 + added    new_file.txt
12:34:58 ~ modified config.json
12:34:59 → renamed  old.txt → new.txt
12:35:00 - deleted  temp.log

12:35:01 GIT Creating commit...
12:35:02 SUCCESS Changes pushed successfully
```

Press Ctrl+C to stop monitoring.

## Notes

- Initial setup creates a private GitHub repository
- Files are committed automatically when changes are detected
- Rename operations are handled properly
- Colored output for better visibility
- Timestamps for all operations
