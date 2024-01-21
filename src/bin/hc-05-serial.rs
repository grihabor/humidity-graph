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
    let mut pins = arduino_hal::pins!(dp);
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
        38400.into_baudrate(),
    );

    ufmt::uwriteln!(&mut master, "Hello from Mega!").unwrap();
    loop {
        'tx: loop {
            let byte = nb::block!(master.read()).unwrap();
            if byte == b'\n' {
                bluetooth.write_byte(b'\r');
                bluetooth.write_byte(byte);
                break 'tx;
            }
            bluetooth.write_byte(byte);
        }
        'rx: loop {
            let byte = nb::block!(bluetooth.read()).unwrap();
            if byte == b'\r' {
                continue;
            }
            master.write_byte(byte);
            if byte == b'\n' {
                break 'rx;
            }
        }
    }
}
