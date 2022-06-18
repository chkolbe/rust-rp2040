//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use rp2040_rust_embedded::ds1820;
use rp2040_rust_embedded::Ds1820Reading;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // todo: find the correct pin on rp2040 Board
    let mut ow_pin = pins.gpio3.into_readable_output();

    // Pulling the pin high to avoid confusing the sensor when initializing
    ow_pin.set_low().ok();

    // The DHT11 datasheet suggests 1 second
    info!("Waiting on the sensor...");
    delay.delay_ms(1000_u32);

    loop {
        match ds1820::Reading::read(&mut delay, &mut ow_pin) {
            Ok(ds1820::Reading {
                temperature,
                relative_humidity,
            }) => info!("{}°, {}% RH", temperature, relative_humidity),
            Err(_) => info!("Error Reading DS1820!"),
        }

        // Delay of at least 500ms before polling the sensor again, 1 second or more advised
        delay.delay_ms(500_u32);
    }
}

// End of file
