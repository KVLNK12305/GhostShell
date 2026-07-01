#!/bin/bash
# Project Ghost Installation Script

set -e

echo "👻 Installing Project Ghost..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Clone or build
if [ ! -d "project-ghost" ]; then
    echo "Building Project Ghost..."
    git clone https://github.com/yourusername/project-ghost.git
    cd project-ghost
else
    cd project-ghost
    git pull
fi

# Build
cargo build --release --features full

# Install
sudo cp target/release/project-ghost /usr/local/bin/ghost

# Create config directory
sudo mkdir -p /etc/ghost
sudo cp config/ghost.yaml /etc/ghost/

# Create systemd service (Linux)
if [ -d "/etc/systemd/system" ]; then
    sudo cp systemd/ghost.service /etc/systemd/system/
    sudo systemctl daemon-reload
fi

echo "✅ Project Ghost installed successfully!"
echo "🚀 Run: ghost --deploy --stealth"
