[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Ratatui](https://img.shields.io/badge/ratatui-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://ratatui.rs)
[![Docker](https://img.shields.io/badge/docker-%23000000.svg?style=for-the-badge&logo=docker&logoColor=white)](https://docker.com)
[![Make](https://img.shields.io/badge/Make-%23000000.svg?style=for-the-badge&logo=gnu&logoColor=white)](https://www.gnu.org/software/make/)
[![PostgreSQL](https://img.shields.io/badge/postgresql-%23000000.svg?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org)

[![Ratabuild Chad](https://img.shields.io/badge/Ratabuild%20Chad-7f56da)](https://meetneura.ai) [![Powered by Neura AI](https://img.shields.io/badge/Powered%20by-Neura%20AI-7f56da)](https://meetneura.ai)

# Ratabuild Chad

**Track and manage build artifacts from multiple programming languages.**

This TUI app runs in your terminal and helps you monitor build processes, track artifacts, and clean up old builds. Built with Ratatui.

![Demo](src/screenshots/ratabuild-chad-demo.GIF)

## Table of Contents

- [What Does This Do?](#what-does-this-do)
- [Quick Start](#quick-start)
- [How to Use It](#how-to-use-it)
- [What You Need](#what-you-need)
- [Special Notes](#special-notes)
- [Contributing](#contributing)
- [License](#license)

## What Does This Do?

- **Tracks build artifacts** - Monitors directories for build outputs from Rust, JavaScript, Python, Go, C/C++, Java, PHP, Ruby, Swift, Kotlin, Scala, Haskell, Elixir, and more.
- **Shows artifact details** - Displays size, modification time, and language type in a table.
- **Selective deletion** - Choose individual or bulk delete with confirmations.
- **Timeframe cleanup** - Set rules to auto-remove old artifacts.
- **Rebuild integration** - Trigger rebuilds for tracked projects.
- **Works everywhere** - Currently tested on Linux (Wayland and X11); support for Windows and macOS may come soon.

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed
- [Docker](https://www.docker.com/products/docker-desktop) for PostgreSQL
- PostgreSQL running (use provided compose.yml)

### Installation

```bash
git clone https://github.com/yourusername/rats-build-chad.git
cd rats-build-chad
docker-compose up -d  # Start PostgreSQL
make build
./target/debug/rats-build-chad
```

### Build with Make

Use the provided Makefile for common tasks:

```bash
make build    # Build the project
make run      # Build and run
make test     # Run tests
make release  # Build release version
make clean    # Clean artifacts
make help     # Show all targets
```

## How to Use It

Once the app is running:

- **Tab** - Switch between views (artifacts, history, charts, settings, summary)
- **↑↓** - Navigate within panels
- **Enter** - Select/rebuild in artifacts, edit settings in settings panel
- **s** - Start scanning for artifacts
- **d** - Delete selected artifacts
- **r** - Rebuild a project
- **h** - Load history
- **q** - Quit

In settings panel, use Enter to open popup for editing retention days, scan path, or toggling automatic removal. For scan path, browse directories with ↑↓ and Enter.

The app detects languages automatically and tracks builds once scanned.

## Settings

Customize the app behavior:

- **Retention Days**: Set how long to keep artifacts (default: 30 days)
- **Scan Path**: Choose the directory to scan for builds (default: current directory)
- **Automatic Removal**: Enable/disable auto-cleanup of old artifacts

Use Enter in the settings panel to edit these options via popups.

## What You Need

- **Computer**: Linux, macOS, or Windows
- **Rust**: Latest stable version
- **Space**: Minimal, depends on your build artifacts

## Special Notes

**First time running**: The app connects to PostgreSQL and creates tables automatically.

**Permissions**: Ensure read/write access to project directories and PostgreSQL access.

## Contributing

Found a bug or want to add something? Check [CONTRIBUTING.md](CONTRIBUTING.md).

## License

See [LICENSE](LICENSE) file for details.

## Star History Chart

[![Star History Chart](https://api.star-history.com/svg?repos=adolfousier/ratabuild-chad&type=date&legend=top-left)](https://www.star-history.com/#adolfousier/ratabuild-chad&type=date&legend=top-left)

**Built with ❤️ by the Neura AI team** | [Website](https://meetneura.ai) | [Issues](https://github.com/adolfousier/ratabuild-chad/issues)
