# PMS (Project Management System)

PMS is a comprehensive project management system with integrated Git version control. It provides real-time file monitoring and automatic Git synchronization with intelligent debouncing and secure GitHub integration.

## Features

### File Monitoring
- Real-time file system monitoring with intelligent debouncing
- Smart detection of file creation, modification, deletion, and rename events
- Automatic filtering of temporary files and system artifacts
- Configurable sync intervals and batch processing
- Cross-platform file system event handling

### Git Integration
- Automated Git repository initialization and configuration
- Secure GitHub repository creation and management
- Intelligent change detection and batching
- Automatic commits with meaningful messages
- Force push protection with configurable settings
- Clean handling of branch management

### Security
- Secure token storage and handling
- File path validation and sanitization
- Configurable file size limits
- Built-in ignore patterns for sensitive files
- Token refresh management
- Permission validation
- Secure error handling

### Process Management
- Clean process management with proper cleanup
- Cross-platform signal handling (Ctrl+C support)
- Graceful shutdown with pending changes sync
- Detailed activity logging
- Colored status output

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/pms.git
cd pms

# Build and install
cargo install --path .
```

## Configuration

Create a configuration file at:
- Linux/macOS: `~/.config/pms/config.toml`
- Windows: `%APPDATA%\pms\config.toml`

Example configuration:
```toml
# GitHub authentication
github_token = "your-github-token"
git_username = "your-username"
git_email = "your-email@example.com"

# Sync settings
sync_interval = 2  # seconds
batch_size = 10    # files per batch

# Security settings
[security]
max_file_size = 104857600  # 100MB
allow_force_push = false
token_refresh_days = 90

# Files to ignore
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

1. Initialize a project:
```bash
pms init [--name project-name]
```

2. Start monitoring:
```bash
pms start
```
This will:
- Initialize Git repository if needed
- Create private GitHub repository
- Configure Git credentials
- Start file monitoring
- Begin automatic synchronization

3. Monitor status:
```bash
pms status
```

4. Stop monitoring:
```bash
pms stop
```

## Project Structure

```
.pms/
├── pms.log          # Activity and error logs (JSON formatted)
├── monitor_active   # Monitoring status indicator
└── config.toml      # Project-specific configuration
```

## Requirements

- Rust 2021 edition (1.56 or later)
- Git 2.0 or later
- GitHub account with personal access token
- Supported platforms:
  - Linux (native)
  - macOS
  - Windows (native and WSL)

## Current State

Version 0.1.0 includes:
- Complete file monitoring system with debouncing
- Secure GitHub integration
- Intelligent change detection
- Cross-platform support
- Configuration system
- Security features
- Process management
- Error handling
- Activity logging

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Security

For security-related information and guidelines, see [SECURITY.md](SECURITY.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support:
1. Check existing GitHub issues
2. Submit new issues with:
   - Operating system and version
   - Rust version (`rustc --version`)
   - Git version (`git --version`)
   - Steps to reproduce
   - Relevant logs from `.pms/pms.log`

## Acknowledgments

PMS is inspired by various project management and version control tools, aiming to combine their best features into a seamless, automated experience.
