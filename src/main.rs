#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal as hal;
use arduino_hal::prelude::*;
use arduino_hal::prelude::*;
use arduino_hal::spi;
use arduino_hal::Delay;
use dht11::Dht11;
use embedded_hal;

use avr_device::interrupt;
use core::cell::RefCell;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut master = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        57600.into_baudrate(),
    );

    let mut bluetooth = arduino_hal::Usart::new(
        dp.USART1,
        pins.d19,
        pins.d18.into_output(),
        9600.into_baudrate(),
    );
    let pin = pins.d2.into_opendrain();
    let mut delay = Delay::new();

    let mut dht11 = Dht11::new(pin);

    loop {
        _ = match dht11.perform_measurement(&mut delay) {
            Ok(meas) => {
                _ = ufmt::uwriteln!(master, "Temp: {} Hum: {}", meas.temperature, meas.humidity);
                _ = ufmt::uwriteln!(
                    bluetooth,
                    "Temp: {} Hum: {}\r",
                    meas.temperature,
                    meas.humidity
                );
            }
            Err(e) => {
                _ = ufmt::uwriteln!(master, "Error");
            }
        };
        arduino_hal::delay_ms(1000);
    }
}
