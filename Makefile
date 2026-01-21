.PHONY: all build release clean test install uninstall cross-cross-cross-help help

# Default target
all: build

# Get current version from Cargo.toml
VERSION := $(shell grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
BINARY_NAME := ascii-fmt

# Build for current platform
build:
	cargo build

# Build release for current platform
release:
	cargo build --release

# Install locally
install: release
	cargo install --path .

# Uninstall locally
uninstall:
	cargo uninstall $(BINARY_NAME)

# Run tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt --check

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist

# Install cross tool for cross-compilation
install-cross:
	cargo install cross --git https://github.com/cross-rs/cross

# Cross-compilation targets
cross-x86_64-apple-darwin:
	cargo build --release --target x86_64-apple-darwin

cross-aarch64-apple-darwin:
	cargo build --release --target aarch64-apple-darwin

cross-x86_64-unknown-linux-gnu:
	cross build --release --target x86_64-unknown-linux-gnu

cross-x86_64-unknown-linux-musl:
	cross build --release --target x86_64-unknown-linux-musl

cross-aarch64-unknown-linux-gnu:
	cross build --release --target aarch64-unknown-linux-gnu

cross-armv7-unknown-linux-gnueabihf:
	cross build --release --target armv7-unknown-linux-gnueabihf

cross-x86_64-pc-windows-msvc:
	cargo build --release --target x86_64-pc-windows-msvc

# Build all cross-compilation targets
cross-all: \
	cross-x86_64-apple-darwin \
	cross-aarch64-apple-darwin \
	cross-x86_64-unknown-linux-gnu \
	cross-x86_64-unknown-linux-musl \
	cross-aarch64-unknown-linux-gnu \
	cross-armv7-unknown-linux-gnueabihf \
	cross-x86_64-pc-windows-msvc

# Package a specific target
package-target = \
	if echo $(TARGET) | grep -q "windows"; then \
		BINARY="target/$(TARGET)/release/$(BINARY_NAME).exe"; \
		ARCHIVE="dist/$(BINARY_NAME)-$(VERSION)-$(TARGET).zip"; \
		if command -v powershell &> /dev/null; then \
			powershell -command "Compress-Archive -Path $$BINARY -DestinationPath $$ARCHIVE"; \
		else \
			zip "$$ARCHIVE" "$$BINARY"; \
		fi; \
	else \
		BINARY="target/$(TARGET)/release/$(BINARY_NAME)"; \
		ARCHIVE="dist/$(BINARY_NAME)-$(VERSION)-$(TARGET).tar.gz"; \
		tar -czf "$$ARCHIVE" -C "$$(dirname $$BINARY)" "$$(basename $$BINARY)"; \
	fi

# Package specific target (usage: make package TARGET=x86_64-apple-darwin)
package:
	@mkdir -p dist
	$(call package-target)

# Package all targets
package-all: cross-all
	@mkdir -p dist
	@$(foreach TARGET,$(shell ls -d target/*/release | cut -d'/' -f2), \
		make package TARGET=$(TARGET) && echo "Packaged $(TARGET)" || echo "Failed to package $(TARGET)"; \
	)
	@echo "Generating checksums..."
	@cd dist && sha256sum *.tar.gz *.zip 2>/dev/null | sort > SHA256SUMS.txt
	@cat dist/SHA256SUMS.txt

# Run benchmarks
bench:
	cargo bench

# Generate documentation
doc:
	cargo doc --open

# Generate documentation for private items
doc-private:
	cargo doc --document-private-items --open

# Show help
help:
	@echo "$(BINARY_NAME) - Makefile commands"
	@echo ""
	@echo "Build commands:"
	@echo "  make build          - Build for current platform (debug)"
	@echo "  make release       - Build for current platform (release)"
	@echo "  make install       - Install locally (release)"
	@echo "  make uninstall     - Uninstall locally"
	@echo ""
	@echo "Test commands:"
	@echo "  make test          - Run all tests"
	@echo "  make test-verbose  - Run tests with output"
	@echo "  make bench         - Run benchmarks"
	@echo ""
	@echo "Code quality:"
	@echo "  make fmt           - Format code"
	@echo "  make fmt-check     - Check formatting"
	@echo "  make clippy        - Run clippy lints"
	@echo ""
	@echo "Cross-compilation:"
	@echo "  make install-cross - Install cross tool"
	@echo "  make cross-all     - Build all cross-compilation targets"
	@echo ""
	@echo "  Specific targets:"
	@echo "    make cross-x86_64-apple-darwin"
	@echo "    make cross-aarch64-apple-darwin"
	@echo "    make cross-x86_64-unknown-linux-gnu"
	@echo "    make cross-x86_64-unknown-linux-musl"
	@echo "    make cross-aarch64-unknown-linux-gnu"
	@echo "    make cross-armv7-unknown-linux-gnueabihf"
	@echo "    make cross-x86_64-pc-windows-msvc"
	@echo ""
	@echo "Packaging:"
	@echo "  make package        - Package specific target (make package TARGET=...)"
	@echo "  make package-all    - Package all targets with checksums"
	@echo ""
	@echo "Other:"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make doc           - Generate and open documentation"
	@echo "  make help          - Show this help message"
