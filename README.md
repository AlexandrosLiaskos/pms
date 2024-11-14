# auto-git-sync

A simple tool that automatically creates GitHub repositories and syncs changes.

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
- Create a GitHub repository using the directory name
- Set up remote automatically
- Create initial commit if needed
- Monitor for changes
- Auto-commit changes every 2 seconds
- Push to GitHub automatically

Press Ctrl+C to stop monitoring.

## Example

```bash
$ mkdir my-project
$ cd my-project
$ ../auto-git-sync/target/debug/auto-git-sync
Initializing Git repository...
Setting up GitHub repository...
Monitoring directory: /path/to/my-project
Auto-sync started. Press Ctrl+C to stop.
```

Your project will be available at `https://github.com/YourUsername/my-project`
