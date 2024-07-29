#![no_std]
#![no_main]

mod peripherals;

use cortex_m_rt::entry;
use panic_halt as _;

use peripherals::{GpioMode, GPIOG, RCC_AHB1ENR};

#[entry]
fn main() -> ! {
    RCC_AHB1ENR.enable_gpiog();
    GPIOG.set_mode(13, GpioMode::Output);
    GPIOG.set_high(13);

    loop {}
}
