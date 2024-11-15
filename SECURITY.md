# Security Policy

## Supported Versions

Currently supported versions for security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously at Auto Git Sync. If you discover a security vulnerability, please follow these steps:

1. **DO NOT** open a public issue on GitHub
2. Send an email to alexliaskosga@gmail.com with:
   - A description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to expect:

1. **Initial Response**: You'll receive an acknowledgment within 48 hours
2. **Status Updates**: We'll keep you informed about the progress
3. **Resolution**: We aim to resolve critical issues within 7 days

## Security Best Practices

When using Auto Git Sync:

1. **GitHub Token Security**:
   - Use tokens with minimal required permissions (only 'repo' scope needed)
   - Regularly rotate your tokens (90-day reminder built-in)
   - Never share or expose your tokens
   - Store tokens only in the config file with proper permissions

2. **File Security**:
   - Don't sync sensitive files
   - Use .gitignore for sensitive patterns
   - Use built-in ignore patterns for common sensitive files
   - Set appropriate file permissions
   - Respect the configured file size limits

3. **Configuration Security**:
   - Keep config file at `~/.config/auto-git-sync/config.toml`
   - Set config file permissions to 600 (user read/write only)
   - Don't commit config files
   - Regularly update credentials
   - Use all available security options in config

4. **Network Security**:
   - Use HTTPS only (enforced by default)
   - Verify SSL certificates
   - Use secure networks
   - Monitor network activity
   - Rate limit API requests

## Known Security Measures

Auto Git Sync implements several security measures:

1. **Token Protection**:
   - Secure token storage with proper file permissions
   - Memory wiping after use
   - Permission validation on startup
   - Token masking in logs
   - Automatic token validation

2. **File Safety**:
   - Path validation and sanitization
   - File permission checks
   - Secure file handling with debouncing
   - Temporary file detection and filtering
   - Maximum file size enforcements

3. **Error Handling**:
   - Secure error messages (no sensitive data)
   - Contextual error information
   - Proper exception handling
   - Fail-safe defaults
   - Graceful shutdown handling

4. **Configuration**:
   - Secure config storage location
   - Strict permission enforcement
   - Input validation and sanitization
   - Secure default values
   - Configuration validation on load

## Security Updates

- Security updates are released as patch versions
- Critical updates are fast-tracked
- All security fixes are documented in CHANGELOG.md
- Users are notified through GitHub releases
- Automatic update checks (planned)

## Audit Logging

Auto Git Sync provides audit logging for security-relevant events:

- File operations (add, modify, delete, rename)
- Authentication attempts
- Configuration changes
- Error conditions
- Sync operations
- Startup and shutdown events

## Secure Development

Our development process includes:

1. **Code Review**:
   - Security-focused reviews
   - Automated scanning with clippy
   - Regular dependency audits
   - Continuous integration checks

2. **Testing**:
   - Security test cases
   - File operation tests
   - Configuration validation tests
   - Error handling tests
   - Signal handling tests

3. **Dependencies**:
   - Regular updates
   - Security audits with cargo audit
   - Minimal dependency usage
   - Version pinning in Cargo.toml

4. **Documentation**:
   - Security guidelines (this document)
   - Best practices in README.md
   - Configuration guides
   - Update procedures
   - Troubleshooting guides

## Contact

Security-related questions: alexliaskosga@gmail.com

For non-security issues, please use GitHub issues.

## Recent Security Improvements

- Improved file change debouncing
- Enhanced error messages with context
- Added file size limits
- Improved temporary file detection
- Added token refresh reminders
- Enhanced permission checks
