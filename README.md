# PMS (Project Management System) - Alpha v0.1.0

PMS is a management system built entirely in Rust, that automatically handles Git synchronization and project organization. It runs in the background, watching your project files and pushing immediately any change that takes place locally to a connected GitHub repository.

## Features

- 🚀 Real-time file monitoring and Git synchronization
- ✔️ Colored status output for different operations
- ⚡ Intelligent debouncing for file changes
- 🔒 Built-in security features

## Installation

### Prerequisites

1. **Rust Environment**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustc --version  
   ```

2. **Git Setup**
   ```bash
   git --version  
   git config --global user.name "Your Name"
   git config --global user.email "your.email@example.com"
   ```

### Install PMS

```bash
cargo install pms
```

Or build from source:

```bash
git clone https://github.com/yourusername/pms
cd pms
cargo install --path .
```

## Quick Start

1. **Configure PMS**
   ```bash
   # Set up your GitHub credentials
   pms config --token "your-github-token" --username "your-github-username" --email "your.email@example.com"
   ```

2. **Initialize a Project**
   ```bash
   # Create a new project
   mkdir my-project
   cd my-project
   pms init --name "my-awesome-project"
   ```

3. **Start Monitoring**
   ```bash
   # Start watching for changes
   pms watch
   ```

## Usage

PMS provides several commands through its CLI:

### Watch Command
```bash
# Watch current directory
pms watch

# Watch specific directory
pms watch /path/to/project

# Watch with verbose output
pms watch -v
```

### Init Command
```bash
# Initialize current directory
pms init

# Initialize with specific name
pms init --name my-project

# Initialize specific directory
pms init /path/to/project

# Initialize with verbose output
pms init -v
```

### Config Command
```bash
# Set GitHub token
pms config --token "your-github-token"

# Set Git username
pms config --username "your-username"

# Set Git email
pms config --email "your.email@example.com"

# Set multiple values at once
pms config --token "token" --username "user" --email "email"
```

### Help
```bash
# Show general help
pms --help

# Show command-specific help
pms watch --help
pms init --help
pms config --help
```

## Configuration

PMS uses a configuration file at `~/.config/pms/config.toml`:

```toml
# Required: GitHub and Git settings
github_token = "ghp_your_token_here"
git_username = "your-github-username"
git_email = "your.email@example.com"

# Optional: Sync settings
sync_interval = 2      # Sync interval in seconds
batch_size = 10       # Number of files to process in batch

# Security settings
[security]
ignore_patterns = [
    "*.env",
    "*.key",
    "*.pem",
    "id_rsa",
    "id_rsa.pub",
    "*.log"
]
max_file_size = 104857600  # Maximum file size in bytes (100MB)
allow_force_push = true    # Allow force pushing to repository
token_refresh_days = 90    # GitHub token refresh reminder
```

## Status Indicators

When watching a directory, PMS uses colored indicators to show file status:

- `+` Yellow: File added
- `~` Blue: File modified
- `→` Bright Blue: File renamed
- `-` Red: File deleted

Example output:
```
04:20:00 ~ modified README.md
04:20:04 SUCCESS Changes synced ✓
04:22:37 ~ modified src/main.rs
04:22:41 SUCCESS Changes synced ✓
04:23:13 → renamed docs/CONTRIBUTING.md
04:23:17 SUCCESS Changes synced ✓
04:23:28 + added tests/test_file.rs
04:23:38 SUCCESS Changes synced ✓
```

## Troubleshooting

Common error messages and solutions:

1. **Configuration Errors**
   ```
   ERROR Failed to read config: Config file not found
   ```
   Solution: Run `pms config` with required parameters

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

## Security

For security best practices and guidelines, please refer to [SECURITY.md](SECURITY.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.
