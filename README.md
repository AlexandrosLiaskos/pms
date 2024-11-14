# auto-git-sync

A simple tool that automatically commits and pushes changes to GitHub.

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

Run in a Git repository:
```bash
./target/debug/auto-git-sync /path/to/repo
```

Or in current directory:
```bash
./target/debug/auto-git-sync
```

The tool will:
- Monitor the directory for changes
- Auto-commit changes every 2 seconds
- Push to GitHub automatically

Press Ctrl+C to stop monitoring.
