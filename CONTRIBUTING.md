# Contributing to Auto Git Sync

First off, thank you for considering contributing to Auto Git Sync! It's people like you that make Auto Git Sync such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps which reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed after following the steps
* Explain which behavior you expected to see instead and why
* Include any error messages or logs

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* Use a clear and descriptive title
* Provide a step-by-step description of the suggested enhancement
* Provide specific examples to demonstrate the steps
* Describe the current behavior and explain which behavior you expected to see instead
* Explain why this enhancement would be useful

### Pull Requests

* Fork the repo and create your branch from `main`
* If you've added code that should be tested, add tests
* Ensure the test suite passes
* Make sure your code lints
* Update the documentation

## Development Setup

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:
```bash
git clone https://github.com/yourusername/auto-git-sync
cd auto-git-sync
```

3. Install dependencies:
```bash
cargo build
```

4. Run tests:
```bash
cargo test
cargo clippy
cargo fmt
```

## Project Structure

```
auto-git-sync/
├── src/
│   ├── main.rs          # Application entry point
│   ├── config.rs        # Configuration handling
│   ├── error.rs         # Error types and handling
│   ├── git.rs           # Git operations
│   ├── logging.rs       # Logging utilities
│   └── watcher.rs       # File system monitoring
├── tests/               # Integration tests
└── examples/            # Usage examples
```

## Coding Guidelines

### Rust Style

* Follow the official Rust style guide
* Use `rustfmt` to format your code
* Run `clippy` and fix any warnings
* Write documentation for public APIs
* Include unit tests for new functionality

### Security

* Never store sensitive information in code
* Use secure token handling practices
* Validate all user inputs
* Handle errors appropriately
* Follow secure coding practices

### Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line

Example:
```
Add file size limit for syncing

- Implement configurable file size limit
- Add validation for file sizes
- Update documentation
- Add tests for size validation

Fixes #123
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test
```

### Writing Tests

* Write unit tests for new functionality
* Include integration tests for new features
* Test edge cases and error conditions
* Mock external dependencies when appropriate

## Documentation

* Keep README.md up to date
* Document new features
* Update API documentation
* Include examples for new functionality

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create a new release branch
4. Run full test suite
5. Create GitHub release
6. Publish to crates.io

## Questions?

Feel free to open an issue with your question or contact the maintainers directly.

Thank you for contributing!
