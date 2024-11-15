# PMS (Project Management System)

PMS is a comprehensive project management system with integrated Git version control. It provides real-time file monitoring and automatic Git synchronization with intelligent debouncing and secure GitHub integration.

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
   # Linux/macOS
   mkdir -p ~/.config/pms

   # Windows (PowerShell)
   New-Item -ItemType Directory -Force -Path $env:APPDATA\pms
   ```

2. Create and edit the configuration file:
   ```bash
   # Linux/macOS
   nano ~/.config/pms/config.toml

   # Windows (PowerShell)
   notepad $env:APPDATA\pms\config.toml
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

### 1. Initialize a Project

```bash
cd your-project-directory  # Must not be a system directory
pms init [--name custom-name]
```

Expected output:
```
HH:MM:SS INIT Initializing Git repository
HH:MM:SS SUCCESS Repository initialized successfully
```

### 2. Start Monitoring

```bash
pms start
```

Expected output:
```
HH:MM:SS STARTUP Project Management System (PMS)
HH:MM:SS STARTUP Monitoring directory: /path/to/your/project
HH:MM:SS STARTUP Repository: https://github.com/username/project
HH:MM:SS STARTUP Press Ctrl+C to stop
```

File status indicators:
- `+` Yellow: File added
- `~` Blue: File modified
- `→` Bright Blue: File renamed
- `-` Red: File deleted

### 3. Monitor Status

```bash
pms status
```

This will show:
- Current monitoring status
- Last sync time
- Pending changes
- Repository status

### 4. Stop Monitoring

Press `Ctrl+C` or run:
```bash
pms stop
```

The system will:
1. Sync any pending changes
2. Clean up resources
3. Exit gracefully

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

## Project Structure

After initialization:
```
your-project/
├── .pms/
│   ├── pms.log          # JSON-formatted logs
│   ├── monitor_active   # Status indicator
│   └── config.toml      # Project-specific settings
└── ... your files ...
```

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
