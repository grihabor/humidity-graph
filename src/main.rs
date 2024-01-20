#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal as hal;
use arduino_hal::prelude::*;
use arduino_hal::spi;
use arduino_hal::Delay;
use dht11::Dht11;
use embedded_hal;

use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    let mut pin = pins.d2.into_opendrain();
    let mut delay = Delay::new();

    let mut dht11 = Dht11::new(pin);

    loop {
        match dht11.perform_measurement(&mut delay) {
            Ok(meas) => println!("Temp: {} Hum: {}", meas.temperature, meas.humidity),
            Err(e) => println!("Error"),
        };
        arduino_hal::delay_ms(1000);
    }
}
