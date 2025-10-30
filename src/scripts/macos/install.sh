#!/bin/bash
# Ratifact Installation Script for macOS
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
    echo -e "${CYAN}║       Ratifact Installation Script - macOS               ║${NC}"
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

# Install Homebrew if not present
print_section "Checking Homebrew"
if ! command -v brew &> /dev/null; then
    print_info "Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" > /dev/null 2>&1
    print_success "Homebrew installed"
else
    print_success "Homebrew already installed"
fi

# Install Rust via Homebrew
print_section "Installing Rust"
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_success "Rust already installed: $RUST_VERSION"
else
    print_info "Installing Rust via Homebrew..."
    brew install rustup-init > /dev/null 2>&1
    rustup-init -y > /dev/null 2>&1
    source $HOME/.cargo/env
    print_success "Rust installed: $(rustc --version)"
fi

# Install Docker Desktop via Homebrew
print_section "Installing Docker"
if command -v docker &> /dev/null; then
    DOCKER_VERSION=$(docker --version)
    print_success "Docker already installed: $DOCKER_VERSION"
else
    print_info "Installing Docker Desktop via Homebrew..."
    brew install --cask docker > /dev/null 2>&1
    print_success "Docker Desktop installed"
    print_warning "Please start Docker Desktop from Applications before continuing"
    read -p "Press Enter after starting Docker Desktop..."
fi

# Wait for Docker to be ready
print_section "Waiting for Docker daemon"
max_retries=60
retry_count=0
while ! docker ps > /dev/null 2>&1; do
    retry_count=$((retry_count + 1))
    if [ $retry_count -ge $max_retries ]; then
        print_error "Docker daemon failed to start after 60 seconds"
        exit 1
    fi
    print_info "Waiting for Docker... ($retry_count/60)"
    sleep 1
done
print_success "Docker is running"

# Install make and git via Homebrew
print_section "Installing build tools"
print_info "Installing make and git..."
brew install make git > /dev/null 2>&1
print_success "Build tools installed"

# Clone repository if not already in it
print_section "Setting up Ratifact"
if [ ! -f "Cargo.toml" ]; then
    print_info "Cloning Ratifact repository..."
    git clone https://github.com/adolfousier/ratifact.git ratifact
    cd ratifact
    print_success "Repository cloned"
else
    print_success "Already in Ratifact directory"
fi

# Generate .env file
print_info "Generating .env file with random credentials..."
PASSWORD=$(openssl rand -base64 12 | tr -d "=+/" | cut -c1-16)

cat > .env << EOF
DATABASE_URL=postgres://ratifact:${PASSWORD}@localhost:25851/ratifact
POSTGRES_USERNAME=ratifact
POSTGRES_PASSWORD=${PASSWORD}
DEBUG_LOGS_ENABLED=true
EOF
print_success ".env file generated"

# Start PostgreSQL container
print_section "Starting PostgreSQL"
print_info "Starting PostgreSQL container via Docker Compose..."
docker compose up -d > /dev/null 2>&1
print_success "PostgreSQL container started"
print_info "Waiting for PostgreSQL to be ready (30 seconds)..."
sleep 30

# Build the project
print_section "Building Ratifact"
print_info "Running cargo build..."
cargo build 2>&1 | tail -1
print_success "Build completed successfully"

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
