#![no_std]
#![no_main]

use panic_probe as _; // Panic handler for embedded environments

pub mod aoc2024;
pub mod aoc2025;
pub mod tcp_server;
pub mod input_parser;
pub mod utils;
pub mod solver;