# Makefile for Ratifact by Neura

.PHONY: build run clean test check release help

# Build the project
build:
	cargo build

# Build and run the project
run: build
	@echo "Checking for .env file..."
	@if [ ! -f .env ]; then \
		echo "Generating .env file..."; \
		PASSWORD=$$(openssl rand -base64 12 | tr -d "=+/" | cut -c1-16); \
		echo "DATABASE_URL=postgres://ratifact:$$PASSWORD@localhost:25851/ratifact" > .env; \
		echo "POSTGRES_USERNAME=ratifact" >> .env; \
		echo "POSTGRES_PASSWORD=$$PASSWORD" >> .env; \
		echo "DEBUG_LOGS_ENABLED=true" >> .env; \
		echo ".env file generated with random password."; \
	else \
		echo ".env file already exists, skipping generation."; \
	fi
	@echo "Checking PostgreSQL status..."
	@if ss -tln | grep -q :25851; then \
		echo "PostgreSQL port 25851 is in use, assuming running."; \
	else \
		echo "Starting PostgreSQL..."; \
		docker-compose up -d; \
	fi
	@echo "Starting Ratifact application..."
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
