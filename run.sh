#!/bin/bash
trap 'exit' INT


#try to build and exit if it fails
cargo build
if [ $? -ne 0 ]; then
    echo "Build failed"
    exit 1
fi


clear

# Run the code on the board
probe-rs reset --chip rp2040 --protocol swd

sleep 2

probe-rs run --chip RP2040 --allow-erase-all ./target/thumbv6m-none-eabi/debug/aoc_2024
# probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/wifi_tcp_server -- moved from the build folder to the deployment folder.
# ./scripts/run.sh -This is to run
# chmod +x myscript.sh -This is to make executable
