@ -0,0 +1,34 @@
#!/bin/bash

echo "Setting up development environment for Raspberry Pi Pico W..."

# Update package list
sudo apt update

# Install essential build tools and libraries
echo "Installing build tools and system libraries..."
sudo apt install -y build-essential pkg-config

# Install Rust (if not already installed)
if ! command -v rustup &> /dev/null
then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
fi

# Add the ARM target for embedded development
echo "Adding thumbv6m-none-eabi target for ARM development..."
rustup target add thumbv6m-none-eabi

# Install probe-rs tools directly from GitHub
echo "Installing probe-rs tools..."
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

# Install elf2uf2-rs for converting ELF files to UF2 format
echo "Installing elf2uf2-rs..."
cargo install elf2uf2-rs

echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="2e8a", ATTR{idProduct}=="000c", MODE="0666"' | sudo tee /etc/udev/rules.d/99-debug-probe.rules && sudo udevadm control --reload && sudo udevadm trigger

echo "Development environment setup complete! You are ready to build and flash code for the Raspberry Pi Pico W."