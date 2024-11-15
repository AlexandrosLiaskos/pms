# PMS (Project Management System)

PMS is a file monitoring system with integrated Git version control. It automatically watches your project directory and synchronizes changes with GitHub in real-time.

## Installation

### Prerequisites

1. **Rust Environment**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustc --version  # Must be 1.56 or later
   ```

2. **Git Setup**
   ```bash
   git --version  # Must be 2.0 or later
   git config --global user.name "Your Name"
   git config --global user.email "your.email@example.com"
   ```

3. **GitHub Token**
   - Generate a Personal Access Token at https://github.com/settings/tokens
   - Token must start with `ghp_` or `github_pat_`
   - Required scopes: `repo`, `workflow`

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/pms.git
cd pms

# Build in release mode
cargo build --release

# Install the binary
cargo install --path .
```

## Configuration

1. Create the configuration directory:
   ```bash
   mkdir -p ~/.config/pms
   ```

2. Create and edit the configuration file:
   ```bash
   nano ~/.config/pms/config.toml
   ```

3. Add required configuration:
   ```toml
   # Required: GitHub authentication
   github_token = "ghp_your_token_here"  # Must start with ghp_ or github_pat_
   git_username = "your-github-username"
   git_email = "your.email@example.com"

   # Optional: Sync settings (defaults shown)
   sync_interval = 2  # Minimum: 1 second
   batch_size = 10    # Minimum: 1 file

   # Optional: Security settings
   [security]
   max_file_size = 104857600  # 100MB
   allow_force_push = false
   token_refresh_days = 90
   ignore_patterns = [
       "*.env",
       "*.key",
       "*.pem",
       "id_rsa",
       "id_rsa.pub",
       "*.log"
   ]
   ```

## Usage

The program monitors a specified directory and automatically syncs changes to GitHub:

```bash
# Start monitoring the current directory
pms

# Monitor a specific directory
pms /path/to/your/project
```

The system will:
1. Initialize a Git repository if needed
2. Create a private GitHub repository
3. Start monitoring file changes
4. Automatically commit and push changes

File status indicators in the terminal:
- `+` Yellow: File added
- `~` Blue: File modified
- `→` Bright Blue: File renamed
- `-` Red: File deleted

To stop monitoring, press `Ctrl+C`. The system will:
1. Sync any pending changes
2. Clean up resources
3. Exit gracefully

## Project Structure

After starting PMS:
```
your-project/
├── .pms/
│   ├── pms.log          # JSON-formatted logs
│   ├── monitor_active   # Status indicator
│   └── config.toml      # Project-specific settings
└── ... your files ...
```

## Troubleshooting

### Common Error Messages

1. **Configuration Errors**
   ```
   ERROR Failed to read config: Config file not found
   ```
   Solution: Create config file at `~/.config/pms/config.toml`

2. **Token Errors**
   ```
   ERROR Invalid GitHub token format
   ```
   Solution: Ensure token starts with `ghp_` or `github_pat_`

3. **Path Errors**
   ```
   ERROR Cannot monitor system directories
   ```
   Solution: Choose a non-system project directory

4. **Git Errors**
   ```
   ERROR Failed to push changes
   ```
   Solution: Check GitHub token permissions and network connection

### Logs

Check detailed logs at:
```bash
cat .pms/pms.log
```

Logs include:
- Timestamped events
- File changes
- Git operations
- Error details

## Support

If you encounter issues:

1. Check the logs:
   ```bash
   cat .pms/pms.log
   ```

2. Submit an issue with:
   - Full error message
   - Log contents
   - OS version
   - Rust version (`rustc --version`)
   - Git version (`git --version`)
   - Steps to reproduce

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
