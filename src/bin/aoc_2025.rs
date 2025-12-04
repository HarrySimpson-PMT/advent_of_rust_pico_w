#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]
use advent_of_rust_pico_w::solver::Solver;
use advent_of_rust_pico_w::tcp_server::TcpServer;

use cyw43::JoinOptions;
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use rand::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/.secrets/wifi_config.rs"
));

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

    let mut pwr = Output::new(p.PIN_23, Level::Low);
    let mut cs = Output::new(p.PIN_25, Level::High);

    pwr.set_low();
    Timer::after_millis(10).await;
    pwr.set_high();
    Timer::after_millis(50).await;
    cs.set_low();
    Timer::after_millis(1).await;
    cs.set_high();
    Timer::after_millis(10).await;

    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(unwrap!(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());

    let seed = rng.next_u64();

    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(unwrap!(net_task(runner)));

    let net_up = embassy_time::with_timeout(Duration::from_secs(5), async {
        while let Err(err) = control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            info!("join failed with status={}", err.status);
        }

        info!("waiting for link...");
        stack.wait_link_up().await;

        info!("Waiting for DHCP lease...");
        let dhcp_ok = embassy_time::with_timeout(Duration::from_secs(20), async {
            while !stack.is_config_up() {
                Timer::after_millis(1000).await;
            }
            Ok::<(), ()>(())
        })
        .await
        .is_ok();

        if !dhcp_ok {
            warn!("DHCP timeout");
            return Err(());
        }

        info!("Stack is up!");

        info!(
            "DHCP is now up! ip addr {}",
            stack.config_v4().unwrap().address
        );

        Ok::<(), ()>(())
    })
    .await
    .is_ok();

    if !net_up {
        warn!("Network timeout – restart device");
        // Simplest recovery: full MCU reset (no stack state to clean)
        cortex_m::peripheral::SCB::sys_reset();
    }

    Timer::after(Duration::from_secs(1)).await;

    loop {
        let server = TcpServer::new(&stack);
        if let Err(e) = server
            .listen(1234, |input| {
                info!("Received input of length: {}", input.len());

                let result = advent_of_rust_pico_w::aoc2025::day01::Day01::solve(input);

                info!("Solver result: {:?}", result);
                result
            })
            .await
        {
            warn!("Listener encountered an error: {:?}", e);
        }

        embassy_time::Timer::after(Duration::from_secs(1)).await;

        info!("Restarting listener...");
    }
}
