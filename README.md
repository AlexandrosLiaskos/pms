# PMOS (Project Management and Organization System)

PMOS is a lightweight project management system that automatically handles Git synchronization and project organization. It runs silently in the background, watching your project files and automatically syncing changes to GitHub.

## System Requirements

**Important Note**: This alpha release is compatible with Unix-based systems only:
- Linux (primary platform)
- macOS (Unix-based)

Windows support is planned for future releases.

## Current Features (Alpha v0.1.0)

- **Automated Git Synchronization**
  - Real-time file system monitoring with detailed event logging
  - Automatic commits with meaningful messages
  - Automatic force pushing to GitHub
  - Smart change detection with configurable sync intervals (default: 50ms)
  - Continuous background monitoring until explicitly stopped
  - Reliable daemon process with comprehensive logging and error tracking
  - Immediate feedback on file system events and sync operations
  - Detailed activity logging for debugging and monitoring
  - Robust error handling and recovery
  - Automatic directory creation and management
  - Persistent file monitoring across sessions
  - Structured logging with JSON output
  - Real-time event tracking and processing
  - Daemon-aware logging system
  - Comprehensive JSON-formatted logs

- **Background Process Management**
  - Runs as a daemon process
  - Persistent monitoring after `pmos start`
  - Clean process management with proper cleanup
  - Reliable PID file handling

- **GitHub Integration**
  - OAuth-based authentication
  - Secure credential storage
  - Automatic repository synchronization
  - Force push to ensure remote sync

## Installation

Currently, PMOS needs to be built from source:

```bash
# Clone the repository
git clone https://github.com/yourusername/pmos.git
cd pmos

# Build and install
cargo install --path .
```

## Quick Start

1. Navigate to your project directory:
```bash
cd your-project
```

2. Initialize PMOS in your project:
```bash
pmos init
```

3. Start monitoring (runs in background):
```bash
pmos start
```
This will start continuous monitoring that persists until explicitly stopped. All changes are automatically committed and force-pushed to the remote repository.

4. Check monitoring status:
```bash
pmos status
```

5. Stop monitoring when needed:
```bash
pmos stop
```

## Project Structure

PMOS creates a `.pmos` directory in your project with the following structure:

```
.pmos/
├── pmos.pid      # Process ID file
├── pmos.log      # Activity and error logs (JSON formatted)
├── monitor_active # Indicates active monitoring status
└── config.toml   # Project configuration
```

## Available Commands

- `pmos init [--name <project-name>]`: Initialize PMOS in the current directory
- `pmos login`: Authenticate with GitHub
- `pmos start`: Start continuous project monitoring (runs until stopped)
- `pmos stop`: Stop project monitoring
- `pmos status`: Show project status and monitoring state

## Current Limitations

As this is an alpha release, please note:

- Unix/Linux systems only (Windows support planned)
- Only supports Git/GitHub (no other VCS)
- Basic authentication flow (GitHub OAuth)
- Simple commit message generation
- Fixed sync interval (2 seconds)
- Limited error recovery
- Basic project statistics

## Roadmap

- [ ] Windows support
- [ ] Configurable sync intervals
- [ ] Custom commit message templates
- [ ] Advanced project statistics
- [ ] Multiple remote support
- [ ] Conflict resolution
- [ ] Web dashboard
- [ ] VSCode extension
- [ ] Team collaboration features
- [ ] Project timeline tracking
- [ ] Resource management

## Contributing

PMOS is in active development and contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Requirements

- Unix-based operating system (Linux or macOS)
- Rust 1.56 or later
- Git installed and configured
- GitHub account

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

As this is an alpha release, please:

1. Check existing issues before reporting problems
2. Provide detailed information when reporting bugs, including:
   - Operating system and version
   - Rust version
   - Git version
   - Steps to reproduce
3. Use the GitHub issues tracker for feature requests

## Acknowledgments

PMOS is inspired by various project management and version control tools, aiming to combine their best features into a seamless, automated experience.
