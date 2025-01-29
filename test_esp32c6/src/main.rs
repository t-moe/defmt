#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};
use defmt::Format;

#[derive(Format)]
struct MyStruct {
    a: u32,
    b: u32,
}

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    esp_println::logger::init_logger_from_env();

    let world = "world";
    let arr = &[10, 11, 12, 4, 5];
    let s = MyStruct { a: 1, b: 2 };
    loop {
        defmt::error!("Hello {:?} {:?} {:?} {:?}:{:?}", world, 13, true, arr, &s);
        delay.delay(500.millis());
    }
}
