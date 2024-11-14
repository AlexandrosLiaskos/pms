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

1. Create a new repository on GitHub:
   - Go to GitHub.com
   - Click "New repository"
   - Give it a name
   - Don't initialize with README
   - Copy the repository URL

2. Run in your project directory:
```bash
./target/debug/auto-git-sync
```

3. When prompted, paste your GitHub repository URL (e.g., https://github.com/username/repo)

The tool will:
- Initialize Git if needed
- Set up the remote repository
- Create initial commit if needed
- Monitor for changes
- Auto-commit changes every 2 seconds
- Push to GitHub automatically

Press Ctrl+C to stop monitoring.

## Example

```bash
$ cd my-project
$ ../auto-git-sync/target/debug/auto-git-sync
Initializing Git repository...
Please enter your GitHub repository URL: https://github.com/username/my-project
Monitoring directory: /path/to/my-project
Auto-sync started. Press Ctrl+C to stop.
