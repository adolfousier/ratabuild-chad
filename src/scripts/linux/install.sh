#!/bin/bash
# Ratifact Installation Script for Linux
# Complete one-liner setup: installs all dependencies and runs the app
# This script is designed to be piped directly: bash -c "$(curl -fsSL https://...install.sh)"

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging setup
LOG_FILE="/tmp/ratifact-install-$(date +%Y%m%d_%H%M%S).log"
exec > >(tee -a "$LOG_FILE")
exec 2>&1

echo "Installation log: $LOG_FILE"

# Helper functions
print_header() {
    echo ""
    echo -e "${CYAN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║      Ratifact Installation Script - Linux                ║${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ ERROR: $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  WARNING: $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

print_section() {
    echo ""
    echo -e "${CYAN}▶ $1${NC}"
}

# Start installation
print_header

# Security check - prompt user to review code
print_section "Code Review Required"
echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}IMPORTANT: Please review the installation script${NC}"
echo -e "${YELLOW}========================================${NC}"
echo "This script will:"
echo "  - Check for: Rust, Docker, OpenSSL"
echo "  - Create a .env file with database credentials"
echo "  - Start PostgreSQL container via Docker Compose"
echo "  - Build the Ratifact application"
echo ""
echo "Script location: $0"
echo ""

read -p "$(echo -e ${YELLOW})Have you reviewed the script and wish to continue? (yes/no)${NC}: " review_confirm
if [[ "$review_confirm" != "yes" ]]; then
    echo -e "${YELLOW}Installation cancelled.${NC}"
    exit 0
fi

# Detect Linux distribution
print_section "Detecting Linux distribution"
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
    print_success "Distribution detected: $PRETTY_NAME"
else
    print_warning "Could not detect distribution. Assuming generic Linux."
    DISTRO="generic"
fi

# Install system dependencies
print_section "Installing system dependencies"
case "$DISTRO" in
    ubuntu|debian)
        print_info "Installing packages via apt..."
        sudo apt-get update -qq
        sudo apt-get install -y -qq curl git openssl docker.io make > /dev/null 2>&1
        sudo usermod -aG docker $USER > /dev/null 2>&1
        print_success "System packages installed"
        ;;
    fedora)
        print_info "Installing packages via dnf..."
        sudo dnf install -y -q curl git openssl docker make > /dev/null 2>&1
        sudo usermod -aG docker $USER > /dev/null 2>&1
        print_success "System packages installed"
        ;;
    arch)
        print_info "Installing packages via pacman..."
        sudo pacman -S --noconfirm curl git openssl docker make > /dev/null 2>&1
        sudo usermod -aG docker $USER > /dev/null 2>&1
        print_success "System packages installed"
        ;;
    *)
        print_warning "Unknown distribution. Please install: curl, git, openssl, docker, make"
        ;;
esac

# Install Rust
print_section "Installing Rust"
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_success "Rust already installed: $RUST_VERSION"
else
    print_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --quiet > /dev/null 2>&1
    source $HOME/.cargo/env
    print_success "Rust installed: $(rustc --version)"
fi

# Start Docker service
print_section "Starting Docker service"
sudo systemctl start docker > /dev/null 2>&1 || true
sudo systemctl enable docker > /dev/null 2>&1 || true

# Check if Docker is accessible
if ! docker ps > /dev/null 2>&1; then
    print_warning "Docker requires sudo. Running with sudo from now on..."
    DOCKER_PREFIX="sudo "
else
    DOCKER_PREFIX=""
fi

# Clone repository if not already in it
if [ ! -f "Cargo.toml" ]; then
    print_section "Cloning Ratifact repository"
    git clone https://github.com/adolfousier/ratifact.git ratifact
    cd ratifact
fi

# Generate .env file
print_section "Setting up .env file"
if [ ! -f ".env" ]; then
    print_info "Generating .env file with random credentials..."
    PASSWORD=$(openssl rand -base64 12 | tr -d "=+/" | cut -c1-16)

    cat > .env << EOF
DATABASE_URL=postgres://ratifact:${PASSWORD}@localhost:25851/ratifact
POSTGRES_USERNAME=ratifact
POSTGRES_PASSWORD=${PASSWORD}
DEBUG_LOGS_ENABLED=true
EOF
    print_success ".env file generated"
else
    print_success ".env file already exists"
fi

# Start PostgreSQL container
print_section "Starting PostgreSQL"
print_info "Starting PostgreSQL container via Docker Compose..."
${DOCKER_PREFIX}docker compose up -d > /dev/null 2>&1
print_success "PostgreSQL container started"
print_info "Waiting for PostgreSQL to be ready (30 seconds)..."
sleep 30

# Build and run the project
print_section "Building Ratifact"
print_info "Running cargo build..."
cargo build 2>&1 | grep -E "Finished|error" || true
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Completion message
echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                Setup Completed Successfully!              ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}To run Ratifact:${NC}"
echo "  cargo run"
echo ""
echo -e "${CYAN}To build a release version:${NC}"
echo "  cargo build --release"
echo ""
echo -e "${CYAN}To run tests:${NC}"
echo "  cargo test"
echo ""
echo -e "${YELLOW}Installation log saved to: $LOG_FILE${NC}"
echo ""
