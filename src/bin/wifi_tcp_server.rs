//! This example uses the RP Pico W board Wifi chip (cyw43).
//! Connects to specified Wifi network and creates a TCP endpoint on port 1234.

#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

use core::str::from_utf8;

use core::fmt::Write as corewt;
use heapless::String;
use heapless::Vec;

use cyw43::JoinOptions;
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use rand::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/.secrets/wifi_config.rs"));

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});


#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");

    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;

    let fw = include_bytes!("../../firmware/43439A0.bin");
    let clm = include_bytes!("../../firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut test28 = Output::new(p.PIN_28, Level::High);
    let mut test27 = Output::new(p.PIN_27, Level::High);
    let mut test26 = Output::new(p.PIN_26, Level::High);
    let mut test22 = Output::new(p.PIN_22, Level::High);
    let mut test21 = Output::new(p.PIN_21, Level::High);
    let mut test20 = Output::new(p.PIN_20, Level::High);
    let mut test19 = Output::new(p.PIN_19, Level::High);
    let mut test18 = Output::new(p.PIN_18, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());

    // Generate random seed
    let seed = rng.next_u64();

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    unwrap!(spawner.spawn(net_task(runner)));

    loop {
        control.gpio_set(0, true).await;
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
                control.gpio_set(0, false).await;
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up!");

    // And now we can use it!

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];

    const MAX_LINES: usize = 300;
    const MAX_CHARS: usize = 100;

    // Define the Vec with explicit capacity
let mut input_lines: Vec<[u8; MAX_CHARS], MAX_LINES> = Vec::new();
let mut received = 0;
    let mut state = InputState::WaitingForStart;
    let mut line_count = 0;

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        control.gpio_set(0, false).await; // LED Off for waiting
        info!("Listening on TCP:1234...");
        if let Err(e) = socket.accept(1234).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());
        let mut phase = Phase::Input;
        control.gpio_set(0, true).await; // LED On for active

        let mut input_lines = Vec::new();
        let mut buf = [0; 4096];

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };

            let command = from_utf8(&buf[..n]).unwrap().trim();
            match state {
                InputState::WaitingForStart => {
                    if command == "Start" {
                        info!("Received Start command. Ready for line count.");
                        state = InputState::WaitingForLineCount;
                    } else {
                        warn!("Unexpected command: {}", command);
                    }
                }
                InputState::WaitingForLineCount => {
                    if let Ok(count) = command.parse::<usize>() {
                        if count <= MAX_LINES {
                            line_count = count;
                            info!("Expecting {} lines.", count);
                            state = InputState::ReceivingLines {
                                expected_lines: count
                            };
                        } else {
                            warn!("Too many lines requested: {}", count);
                        }
                    } else {
                        warn!("Invalid line count: {}", command);
                    }
                }
                InputState::ReceivingLines {
                    expected_lines
                } => {
                    if received < expected_lines {
                        let mut buffer = [0u8; MAX_CHARS];
                        let bytes = command.as_bytes();
                        if bytes.len() <= MAX_CHARS {
                            buffer[..bytes.len()].copy_from_slice(bytes);
                            received += 1;
                            input_lines[received] = buffer;
                            info!(
                                "Received line {}/{}: {}",
                                received + 1,
                                expected_lines,
                                command
                            );

                            state = InputState::ReceivingLines {
                                expected_lines
                            };

                            if received + 1 == expected_lines {
                                info!("All lines received. Ready to process.");
                                state = InputState::ReadyToProcess;
                            }
                        } else {
                            warn!("Line too long, skipping!");
                        }
                    }
                }
                InputState::ReadyToProcess => {
                    if command == "GO" {
                        info!("Processing input...");
                        let result = process_input(&input_lines, line_count);

                        // Respond with the result
                        if let Err(e) = socket.write_all(result.as_bytes()).await {
                            warn!("write error: {:?}", e);
                        }

                        state = InputState::WaitingForStart; // Reset for new session
                    } else {
                        warn!("Unexpected command: {}", command);
                    }
                }
            }
        }
    }
    fn process_input(input_lines: &Vec<[u8; MAX_CHARS], MAX_LINES>, line_count: usize) -> String<256> {
        use core::fmt::Write;
    
        const MAX_RESULT_LEN: usize = 256;
        let mut result: String<MAX_RESULT_LEN> = String::new();
    
        for i in 0..line_count {
            let line = core::str::from_utf8(&input_lines[i]).unwrap_or("[Invalid UTF-8]");
            writeln!(result, "Line {}: {}", i + 1, line).unwrap();
        }
    
        result
    }
    
}
enum Phase {
    Input,
    Process,
    Result,
}
enum InputState {
    WaitingForStart,
    WaitingForLineCount,
    ReceivingLines {
        expected_lines: usize
    },
    ReadyToProcess,
}
