# Auto Git Sync

🔄 A secure, real-time Git synchronization tool that automatically commits and pushes your changes to GitHub.

## Features

- 🚀 **Real-time Synchronization**: Automatically detects and syncs file changes
- 🔒 **Secure**: Safe token handling and secure GitHub integration
- 🎨 **Visual Feedback**: Clear, colored status indicators for all operations
- 🛡️ **Smart Handling**: Intelligent file event processing and Windows compatibility
- ⚡ **Efficient**: Minimal resource usage with smart batching of changes

## Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/auto-git-sync
cd auto-git-sync

# Build the project
cargo build --release

# The binary will be available at
./target/release/auto-git-sync
```

### Security Setup

1. Generate a GitHub Personal Access Token:
   - Go to GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Click "Generate new token (classic)"
   - Select the "repo" scope only (minimum required permissions)
   - Copy the generated token

2. Configure the tool:
   ```bash
   # First run will create example config
   ./auto-git-sync

   # Edit the config file
   nano ~/.config/auto-git-sync/config.toml
   ```

3. Add your credentials:
   ```toml
   github_token = "your_token_here"
   git_username = "YourGitHubUsername"
   git_email = "your.email@example.com"
   ```

## Usage

```bash
# Monitor current directory
auto-git-sync

# Monitor specific directory
auto-git-sync /path/to/directory
```

### Status Indicators

The tool provides clear visual feedback for all operations:

- 🟡 `added` - New files (yellow)
- 🔵 `modified` - Content changes (blue)
- 🌟 `renamed` - File renames (bright blue)
- 🔴 `deleted` - Removed files (red)
- ✅ `SUCCESS` - Successful sync (green)

## Security Considerations

1. **Token Security**:
   - Store tokens securely in the config file
   - Use minimum required permissions
   - Regularly rotate tokens
   - Never commit tokens to version control

2. **File Safety**:
   - Only monitors specified directory
   - Ignores system and temporary files
   - Respects .gitignore rules
   - Safe handling of file operations

3. **Network Security**:
   - HTTPS connections only
   - Secure GitHub API integration
   - Token-based authentication
   - No plain-text password storage

## Advanced Usage

### Custom Configuration

You can customize the behavior by editing `~/.config/auto-git-sync/config.toml`:

```toml
[sync]
interval = 2  # Sync interval in seconds
batch_size = 10  # Maximum changes per commit

[git]
default_branch = "main"
commit_message = "Auto-sync update"

[security]
ignore_patterns = [".env", "*.key"]
```

### Integration Examples

1. **Development Workflow**:
   ```bash
   # Start in project directory
   cd my-project
   auto-git-sync &
   
   # Continue working, changes auto-sync
   ```

2. **Document Sync**:
   ```bash
   # Sync documents folder
   auto-git-sync ~/Documents/notes
   ```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

1. Clone and setup:
   ```bash
   git clone https://github.com/yourusername/auto-git-sync
   cd auto-git-sync
   cargo build
   ```

2. Run tests:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust 🦀
- Uses [notify](https://github.com/notify-rs/notify) for file system events
- Thanks to all contributors!
