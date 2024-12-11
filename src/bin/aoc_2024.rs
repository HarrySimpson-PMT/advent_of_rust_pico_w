#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

use advent_of_rust_pico_w::aoc2024::day01::Day01;
use advent_of_rust_pico_w::aoc2024::Solver;
use advent_of_rust_pico_w::tcp_server::TcpServer;


use cyw43::JoinOptions;
use cyw43_pio::PioSpi;
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
    let mut test28 = Output::new(p.PIN_28, Level::High);
    let mut test27 = Output::new(p.PIN_27, Level::High);
    let mut test26 = Output::new(p.PIN_26, Level::High);
    let mut test22 = Output::new(p.PIN_22, Level::High);
    let mut test21 = Output::new(p.PIN_21, Level::High);
    let mut test20 = Output::new(p.PIN_20, Level::High);
    let mut test19 = Output::new(p.PIN_19, Level::High);
    let mut test18 = Output::new(p.PIN_18, Level::High);

    let mut rng = RoscRng;

    let fw = include_bytes!("../../firmware/43439A0.bin");
    let clm = include_bytes!("../../firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
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
    control.leave().await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());


    test28.set_low();
    test27.set_low();
    test26.set_low();
    test22.set_low();
    test21.set_low();
    test20.set_low();
    test19.set_low();
    test18.set_low();

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
    test28.set_high();    
    info!("DHCP is now up!");
    
    info!("waiting for link up...");
    while !stack.is_link_up() {
        Timer::after_millis(500).await;
    }
    test27.set_high();
    info!("Link is up!");
    
    info!("waiting for stack to be up...");
    stack.wait_config_up().await;
    test26.set_high();
    info!("Stack is up!");

    info!(
        "DHCP is now up! ip addr {}",
        stack.config_v4().unwrap().address
    );
    test22.set_high();
    //wait 2 seconds
    Timer::after(Duration::from_secs(1)).await;
    test28.set_low();
    test27.set_low();
    test26.set_low();
    test22.set_low();
    test21.set_low();
    test20.set_low();
    test19.set_low();
    test18.set_low();
  
    loop {
        let server = TcpServer::new(&stack);
        if let Err(e) = server
            .listen(1234, |input| {
                info!("Handling input: {:?}", input);
    
                // Use the Day01 solver
                let result = Day01::solve(input.clone());
    
                // Log and return the response
                info!("Solver result: {:?}", result);
                result
            })
            .await
        {
            warn!("Listener encountered an error: {:?}", e);
        }
    
        // Add a small delay to avoid rapid retries in case of persistent errors
        embassy_time::Timer::after(Duration::from_secs(1)).await;
    
        info!("Restarting listener...");
    }    
}
