# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-10-27

### Added

- Automatic .env file generation in Makefile with random PostgreSQL password
- PostgreSQL startup check and auto-start in Makefile run target
- Sudo support for artifact deletion with password prompt
- Artifact actions popup for delete/rebuild operations
- Clear all builds feature with Shift+D shortcut
- Progress and info popups for better user feedback
- Password input masking for sudo operations

### Changed

- Default retention_days increased from 1 to 2 days
- Default POSTGRES_USERNAME changed to 'ratifact'
- UI title updated to "Build Artifact Tool"
- Footer shortcuts updated to include Shift+D for clear all
- Artifact list display improved with relative paths and reordered columns
- Fixed Docker Compose volume name typo

### Fixed

- Language detection tests updated to use detect_language_for_path function
- Removed dependency on changing current directory in tests

---

## [0.1.0] - 2025-10-26

### Added

- **Initial Release**: Ratifact TUI application for tracking and managing build artifacts.
- **Multi-Language Support**: Tracks build artifacts from Python, C, Rust, JavaScript, TypeScript, and more.
- **Artifact Scanning**: On-demand scanning of project directories for common build folders (target/, node_modules/, **pycache**/, etc.).
- **Interactive TUI**: Ratatui-based terminal interface with tabs for artifacts, history, and settings.
- **Selective Deletion**: Select and delete individual build artifacts with safety checks.
- **Unusual File Detection**: Warns and skips deletion if artifacts contain bundles or binaries.
- **Rebuild Integration**: One-click rebuild for detected projects (Cargo, npm, etc.).
- **PostgreSQL Database**: Uses PostgreSQL for storing build logs and metadata.
- **Docker Support**: Includes compose.yml for easy PostgreSQL setup.
- **Makefile**: Build system with targets for build, run, test, clean, etc.
- **Unit Tests**: Comprehensive tests for database, utilities, and core functionality.
- **Modular Architecture**: Organized code in src/ with separate modules for DB, tracking, UI, config, and utils.

### Features

- Tab navigation between views
- Keyboard shortcuts: s (scan), d (delete), r (rebuild), q (quit)
- Highlighted selection in artifact list
- Footer with instructions
- Configurable scan paths and retention settings
- Async database operations with sqlx

### Technical Details

- Built with Rust and Ratatui
- PostgreSQL backend with sqlx
- Currently tested on Linux (Wayland and X11); support for Windows and macOS may come soon
- No external dependencies for core functionality

### Known Issues

- Tests require running PostgreSQL instance
- UI is basic; future versions may add more widgets

### Contributors

- Initial development by opencode team

---

## Types of changes

- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities
