# Auto Git Sync

🔄 A secure, real-time Git synchronization tool that automatically commits and pushes your changes to GitHub.

## Features

- 🚀 **Real-time Synchronization**: Automatically detects and syncs file changes
- 🔒 **Secure**: Safe token handling and secure GitHub integration
- 🎨 **Visual Feedback**: Clear, colored status indicators for all operations
- 🛡️ **Smart Handling**: Intelligent file event processing with debouncing
- ⚡ **Efficient**: Minimal resource usage with smart batching of changes
- 🎯 **Clean Exit**: Graceful shutdown with Ctrl+C

## Quick Start

```bash
# Install from source
git clone https://github.com/yourusername/auto-git-sync
cd auto-git-sync
cargo build --release

# Run for the first time to generate config
./target/release/auto-git-sync

# Edit config with your GitHub credentials
nano ~/.config/auto-git-sync/config.toml

# Start monitoring a directory
./target/release/auto-git-sync /path/to/directory
```

## Configuration

The config file is located at `~/.config/auto-git-sync/config.toml`:

```toml
# Required GitHub credentials
github_token = "your_github_token"
git_username = "YourGitHubUsername"
git_email = "your.email@example.com"

# Optional sync settings
sync_interval = 2     # Sync interval in seconds (default: 2)
batch_size = 10      # Maximum changes per commit (default: 10)

[security]
# Files to ignore (default patterns shown)
ignore_patterns = [
    "*.env",
    "*.key",
    "*.pem",
    "id_rsa",
    "id_rsa.pub",
    "*.log"
]
max_file_size = 104857600  # Maximum file size in bytes (100MB)
allow_force_push = false   # Whether to allow force pushing
token_refresh_days = 90    # Reminder to refresh token
```

### GitHub Token Setup

1. Go to GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Select the "repo" scope only
4. Copy the generated token to your config file

## Usage

```bash
# Monitor current directory
auto-git-sync

# Monitor specific directory
auto-git-sync /path/to/directory

# Stop monitoring
Press Ctrl+C to exit cleanly
```

### Status Indicators

The tool provides clear visual feedback:

- 🟡 `added` - New files
- 🔵 `modified` - Content changes
- 🌟 `renamed` - File renames
- 🔴 `deleted` - Removed files
- ✅ `SUCCESS` - Successful sync

## Real-World Examples

### Development Workflow
```bash
# Start monitoring your project
cd ~/projects/my-webapp
auto-git-sync &

# Continue development, changes auto-sync
npm run dev
```

### Document Sync
```bash
# Sync your notes
auto-git-sync ~/Documents/notes

# Sync multiple directories (in separate terminals)
auto-git-sync ~/Documents/blog-posts
auto-git-sync ~/Documents/research
```

### Team Collaboration
```bash
# Set up shared project sync
cd ~/team-projects/shared-docs
auto-git-sync

# All team members' changes auto-sync to GitHub
```

## Security Features

- 🔐 **Token Security**
  - Secure storage in config file
  - Automatic token validation
  - Regular rotation reminders
  - Permission scope validation

- 🛡️ **File Safety**
  - Smart file ignore patterns
  - Temporary file detection
  - Size limit enforcement
  - .gitignore respect

- 🔒 **Network Security**
  - HTTPS only
  - Token-based auth
  - No plain-text storage
  - Secure API integration

## Advanced Features

### Intelligent Change Detection
- Debouncing to prevent rapid commits
- Smart batching of related changes
- Temporary file filtering
- Rename operation detection

### Performance Optimization
- Minimal resource usage
- Efficient file system monitoring
- Smart caching of file states
- Batch processing of changes

## Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   # Fix config file permissions
   chmod 600 ~/.config/auto-git-sync/config.toml
   ```

2. **Token Invalid**
   ```bash
   # Verify token has 'repo' scope
   # Regenerate if needed
   ```

3. **Changes Not Syncing**
   ```bash
   # Check ignore patterns
   # Verify file sizes
   # Ensure proper permissions
   ```

## Contributing

We welcome contributions! See [Contributing Guidelines](CONTRIBUTING.md) for:
- Code style guide
- Pull request process
- Development setup
- Testing requirements

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with Rust 🦀
- Uses [notify](https://github.com/notify-rs/notify) for file system events
- Thanks to all contributors!
