#!/usr/bin/env bash
set -euo pipefail
trap 'echo -e "\n[!] Aborted by user"; exit 130' INT

BIN="${1:-aoc_2025}"
TRIPLE="thumbv6m-none-eabi"
PROFILE="debug"

echo "[1/4] Building ${BIN} (${PROFILE})..."
cargo build --bin "${BIN}" --target "${TRIPLE}" ${PROFILE:+--release}

LOCAL_BIN="./target/${TRIPLE}/${PROFILE}/${BIN}"

echo "[2/4] Deploying → Peregrinus..."
scp "${LOCAL_BIN}" harry@10.0.0.86:/home/harry/pico_deployment/"${BIN}"

echo "[3/4] Reset + flash + run – full interactive TTY, progress bars, defmt, zero bullshit"
ssh -t -t harry@10.0.0.86 \
  env PATH="/home/harry/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" \
  probe-rs run --chip RP2040 --allow-erase-all "/home/harry/pico_deployment/${BIN}"

echo "Done – back on cogito."