# Makefile for rats-build-chad

.PHONY: build run clean test check release help

# Build the project
build:
	cargo build

# Build and run the project
run: build
	cargo run

# Run tests
test:
	cargo test

# Check code without building
check:
	cargo check

# Build release version
release:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Lint code
clippy:
	cargo clippy

# Run all checks
all: check test

# Show help
help:
	@echo "Available targets:"
	@echo "  build    - Build the project"
	@echo "  run      - Build and run the project"
	@echo "  test     - Run tests"
	@echo "  check    - Check code without building"
	@echo "  release  - Build release version"
	@echo "  clean    - Clean build artifacts"
	@echo "  fmt      - Format code"
	@echo "  clippy   - Lint code"
	@echo "  all      - Run check and test"
	@echo "  help     - Show this help"