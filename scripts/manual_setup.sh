#!/bin/bash
set -e  # Stop on first error

BINARY_NAME="wifi_tcp_server"

# Build the binary
echo "Building $BINARY_NAME..."
cargo build --release --bin $BINARY_NAME

# Convert to UF2
echo "Converting to UF2..."
elf2uf2-rs target/thumbv6m-none-eabi/release/$BINARY_NAME

# Provide feedback
echo "UF2 file created: target/thumbv6m-none-eabi/release/$BINARY_NAME.uf2"

# Optional: Flash to Pico
# echo "Flashing the Pico..."
# cp target/thumbv6m-none-eabi/release/$BINARY_NAME.uf2 /media/$USER/RPI-RP2