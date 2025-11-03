#!/bin/bash
# Ratifact Uninstallation Script for macOS
# Removes Ratifact application, Docker container, and PostgreSQL volume

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging setup
LOG_FILE="/tmp/ratifact-uninstall-$(date +%Y%m%d_%H%M%S).log"
exec > >(tee -a "$LOG_FILE")
exec 2>&1

echo "Uninstall log: $LOG_FILE"

# Helper functions
print_header() {
    echo ""
    echo -e "${CYAN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║      Ratifact Uninstallation Script - macOS               ║${NC}"
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

# Confirmation prompt
confirm() {
    local prompt="$1"
    local response
    read -p "$(echo -e "${YELLOW}$prompt (yes/no)${NC}: ")" response
    [[ "$response" == "yes" ]] && return 0 || return 1
}

# Check if Docker is available
check_docker() {
    if ! command -v docker &> /dev/null; then
        print_warning "Docker is not installed or not in PATH"
        return 1
    fi
    return 0
}

# Main uninstall process
main() {
    print_header

    # Determine installation directory
    INSTALL_DIR="${RATIFACT_INSTALL_DIR:-.}"
    if [ ! -d "$INSTALL_DIR" ] || [ ! -f "$INSTALL_DIR/Cargo.toml" ]; then
        # Try common locations
        if [ -d "$HOME/ratifact" ]; then
            INSTALL_DIR="$HOME/ratifact"
        elif [ -d "/opt/ratifact" ]; then
            INSTALL_DIR="/opt/ratifact"
        else
            print_warning "Could not locate Ratifact installation directory"
            read -p "Enter the path to your Ratifact installation: " INSTALL_DIR
            if [ ! -d "$INSTALL_DIR" ] || [ ! -f "$INSTALL_DIR/Cargo.toml" ]; then
                print_error "Invalid installation directory: $INSTALL_DIR"
                exit 1
            fi
        fi
    fi

    print_info "Found Ratifact installation at: $INSTALL_DIR"
    echo ""

    # Confirmation
    echo -e "${YELLOW}This will:${NC}"
    echo "  1. Stop the PostgreSQL Docker container"
    echo "  2. Remove the PostgreSQL Docker volume (optional)"
    echo "  3. Clean build artifacts"
    echo "  4. Remove the installation directory (optional)"
    echo ""

    if ! confirm "Do you want to continue with uninstallation?"; then
        print_info "Uninstallation cancelled"
        exit 0
    fi

    # Check Docker
    if check_docker; then
        print_section "Stopping Docker containers..."
        if cd "$INSTALL_DIR" 2>/dev/null; then
            if [ -f "compose.yml" ]; then
                if docker compose down 2>/dev/null; then
                    print_success "Docker containers stopped"
                else
                    print_warning "Could not stop Docker containers (they may not be running)"
                fi
            else
                print_warning "compose.yml not found"
            fi
        else
            print_warning "Could not change to installation directory"
        fi

        # Ask about removing volume
        echo ""
        print_info "PostgreSQL data volume: ratifact-postgres-data"
        if confirm "Do you want to remove the PostgreSQL volume (this will delete all database data)?"; then
            print_section "Removing PostgreSQL volume..."
            if docker volume rm ratifact-postgres-data 2>/dev/null; then
                print_success "PostgreSQL volume removed"
            else
                print_warning "Could not remove volume (it may not exist or requires elevated permissions)"
            fi
        else
            print_info "Keeping PostgreSQL volume"
        fi
    else
        print_warning "Skipping Docker container and volume removal (Docker not available)"
    fi

    # Clean build artifacts
    print_section "Cleaning build artifacts..."
    if [ -d "$INSTALL_DIR/target" ]; then
        if cd "$INSTALL_DIR" && cargo clean 2>/dev/null; then
            print_success "Build artifacts cleaned"
        else
            print_warning "Could not clean build artifacts (Rust may not be in PATH)"
        fi
    else
        print_info "No build artifacts found"
    fi

    # Ask about removing installation directory
    echo ""
    if confirm "Do you want to remove the entire installation directory?"; then
        print_section "Removing installation directory..."
        if rm -rf "$INSTALL_DIR"; then
            print_success "Installation directory removed: $INSTALL_DIR"
        else
            print_error "Could not remove installation directory (permission denied?)"
            print_info "You can manually remove it with: rm -rf $INSTALL_DIR"
            exit 1
        fi
    else
        print_info "Keeping installation directory at: $INSTALL_DIR"
    fi

    echo ""
    print_section "Uninstallation complete!"
    echo ""
    print_success "Ratifact has been successfully uninstalled"
    echo ""
    print_info "To reinstall, visit: https://github.com/adolfousier/ratifact"
    echo ""
}

main "$@"
