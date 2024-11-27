echo "Build Deployment started."
cargo build

probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/wifi_tcp_server
# ./scripts/run.sh -This is to run
# chmod +x myscript.sh -This is to make executable
