#!/bin/bash
# VQL Installation Script

echo "Building VQL..."
cargo build --release

echo "Installing VQL..."
# Install to cargo bin (for cargo install compatibility)
cargo install --path . --force

# Also copy to user bin if it exists
if [ -d "$HOME/bin" ]; then
    echo "Updating $HOME/bin/vql..."
    cp target/release/vql "$HOME/bin/vql"
fi

echo "Installation complete!"
echo "VQL version: $(target/release/vql --version)"