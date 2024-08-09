#![no_std]
#![no_main]

mod peripherals;

use cortex_m_rt::entry;
use panic_halt as _;

use peripherals::{AltFunc, GpioMode, SpiBaudDivisor, GPIOA, GPIOC, GPIOD, GPIOF, GPIOG, RCC_AHB1ENR, RCC_APB2ENR, SPI5, USART1};

const SYSCLK_SPEED: u32 = 16_000_000;

#[entry]
fn main() -> ! {
    RCC_AHB1ENR.enable_gpioa();
    RCC_AHB1ENR.enable_gpioc();
    RCC_AHB1ENR.enable_gpiod();
    RCC_AHB1ENR.enable_gpiof();
    RCC_AHB1ENR.enable_gpiog();
    // RCC_AHB1ENR.enable!(GPIOA, GPIOG);

    RCC_APB2ENR.enable_usart1();
    RCC_APB2ENR.enable_spi5();

    GPIOG.set_mode(13, GpioMode::Output);
    GPIOG.set_high(13);

    uart1_set_up();

    // As a bit of a hack, write '.' 32 times as a delay in order to allow the display controller to come up
    for _ in 0..32 {
        USART1.transmit_byte('.' as u8);
    }

    lcd_set_up_spi();
    lcd_test();

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

fn lcd_set_up_spi() {
    GPIOF.set_alt_func(7, AltFunc::AF5);
    GPIOF.set_alt_func(9, AltFunc::AF5);

    GPIOF.set_mode(7, GpioMode::AltFunc);
    GPIOF.set_mode(9, GpioMode::AltFunc);

    GPIOD.set_mode(13, GpioMode::Output);
    GPIOC.set_mode(2, GpioMode::Output);

    GPIOC.set_high(2);

    SPI5.set_baud_divisor(SpiBaudDivisor::Div2);

    SPI5.software_sub_management_sub_select_high_main_mode_spi_enable();
}

fn lcd_test() {
    lcd_command(LcdCommand::ExitSleepMode);
    lcd_command(LcdCommand::DisplayOn);
}

enum LcdCommand {
    ExitSleepMode = 0x11,
    DisplayOn = 0x29,
}

fn lcd_command(cmd: LcdCommand) {
    // Set data/command select low
    GPIOD.set_low(13);

    // Turn chip select on (low)
    GPIOC.set_low(2);

    // Transmit the command byte
    SPI5.write_byte(cmd as u8);

    // Turn chip select off (high)
    GPIOC.set_high(2);
}
