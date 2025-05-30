.PHONY: build install test clean dev release

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Install to all locations
install: release
	./install.sh

# Run tests
test:
	cargo test
	cd vscode-extension && npm test

# Clean build artifacts
clean:
	cargo clean
	rm -rf vscode-extension/out
	rm -rf vscode-extension/node_modules

# Development build and install
dev: build
	cp target/debug/vql ~/.cargo/bin/vql
	[ -d "$$HOME/bin" ] && cp target/debug/vql $$HOME/bin/vql || true
	@echo "Development build installed"

# Check version
version:
	@vql --version
	@echo "Binary locations:"
	@which -a vql || true