#![no_std]
#![no_main]

mod peripherals;

use cortex_m_rt::entry;
use panic_halt as _;

use peripherals::{AltFunc, GpioMode, GPIOA, GPIOG, RCC_AHB1ENR, RCC_APB2ENR, USART1};

const SYSCLK_SPEED: u32 = 16_000_000;

#[entry]
fn main() -> ! {
    RCC_AHB1ENR.enable_gpioa();
    RCC_AHB1ENR.enable_gpiog();
    // RCC_AHB1ENR.enable!(GPIOA, GPIOG);

    RCC_APB2ENR.enable_usart1();

    GPIOG.set_mode(13, GpioMode::Output);
    GPIOG.set_high(13);

    uart1_set_up();

    // As a bit of a hack, write '.' 32 times as a delay in order to allow the display controller to come up
    for _ in 0..32 {
        USART1.transmit_byte('.' as u8);
    }

    loop {}
}

fn uart1_set_up() {
    GPIOA.set_alt_func(9, AltFunc::AF7);
    GPIOA.set_alt_func(10, AltFunc::AF7);
    // GPIOA.set_alt_funcs!(P9: AF7, P10: AF7);

    GPIOA.set_mode(9, GpioMode::AltFunc);
    GPIOA.set_mode(10, GpioMode::AltFunc);
    // GPIOA.set_modes!(P9: AltFunc, P10: AltFunc);

    USART1.set_brr((SYSCLK_SPEED / 9600) as u16);
    USART1.enable_rx_tx();
}
