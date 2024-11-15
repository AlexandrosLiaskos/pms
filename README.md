# PMS (Project Management System)

PMS is a comprehensive project management system with integrated Git version control. It helps you organize and track your projects while automatically handling Git synchronization in the background.

## System Requirements

The system is cross-platform compatible and works on:
- Linux (native)
- Windows (native and WSL)
- macOS

## Current Features (Alpha v0.1.0)

- **Project Management**
  - Project organization and tracking
  - Task management and monitoring
  - Project status tracking
  - Resource allocation tracking
  - Project timeline management
  - Activity logging and reporting
  - Cross-platform compatibility
  - Real-time project updates

- **Git Integration**
  - Automated Git synchronization
  - Real-time file system monitoring
  - Automatic commits with meaningful messages
  - Automatic pushing to GitHub
  - Smart change detection
  - Continuous background monitoring
  - Detailed activity logging

- **Process Management**
  - Clean process management with proper cleanup
  - Cross-platform signal handling
  - Reliable state management

- **GitHub Integration**
  - OAuth-based authentication
  - Secure credential storage
  - Automatic repository synchronization
  - Force push to ensure remote sync

## Installation

Currently, PMS needs to be built from source:

```bash
# Clone the repository
git clone https://github.com/yourusername/pms.git
cd pms

# Build and install
cargo install --path .
```

For Windows users:
```powershell
# Using PowerShell
git clone https://github.com/yourusername/pms.git
cd pms
cargo install --path .
```

## Quick Start

1. Navigate to your project directory:
```bash
cd your-project
# or on Windows:
cd path\to\your-project
```

2. Initialize PMS in your project:
```bash
pms init
```

3. Start project management and monitoring:
```bash
pms start
```
This will start continuous project monitoring and management that persists until explicitly stopped.

4. Check project status:
```bash
pms status
```

5. Stop monitoring when needed:
```bash
pms stop
```

## Project Structure

PMS creates a `.pms` directory in your project with the following structure:

```
.pms/
├── pms.log       # Activity and error logs (JSON formatted)
├── monitor_active # Indicates active monitoring status
└── config.toml   # Project configuration
```

## Available Commands

- `pms init [--name <project-name>]`: Initialize PMS in the current directory
- `pms login`: Authenticate with GitHub
- `pms start`: Start project monitoring and management
- `pms stop`: Stop project monitoring
- `pms status`: Show project status and monitoring state

## Platform-Specific Notes

### Windows
- Uses Windows-native file system events
- Supports both cmd.exe and PowerShell
- Works in WSL (Windows Subsystem for Linux)
- Config stored in `%APPDATA%\pms`

### Linux/macOS
- Uses inotify/FSEvents for file monitoring
- Config stored in `~/.config/pms`
- Full terminal support

## Current Limitations

As this is an alpha release, please note:

- Basic project management features
- Simple task tracking
- Basic authentication flow (GitHub OAuth)
- Simple commit message generation
- Fixed sync interval (2 seconds)
- Limited error recovery
- Basic project statistics

## Roadmap

- [ ] Advanced project tracking
- [ ] Task dependencies and critical path
- [ ] Resource allocation optimization
- [ ] Team collaboration features
- [ ] Project timeline visualization
- [ ] Custom workflow templates
- [ ] Advanced reporting
- [ ] Web dashboard
- [ ] VSCode extension
- [ ] Multiple remote support
- [ ] Conflict resolution
- [ ] Project analytics

## Contributing

PMS is in active development and contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Requirements

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

PMS is inspired by various project management and version control tools, aiming to combine their best features into a seamless, automated experience.
