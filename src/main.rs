#![no_std]
#![no_main]

mod font;
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

    draw_demo();

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

fn draw_demo() {
    draw_char_at(0, 0, 1);
    draw_char_at(0, 1, 0);
    draw_char_at(0, 2, 2);
    draw_char_at(0, 3, 1);
    draw_char_at(0, 4, 2);
    draw_char_at(0, 5, 0);
    draw_char_at(0, 6, 1);
    draw_char_at(0, 7, 0);
    draw_char_at(0, 8, 3);
    draw_char_at(0, 9, 1);
    draw_char_at(0, 10, 2);

    draw_char_at(5, 0, 1);
    draw_char_at(5, 1, 2);
    draw_char_at(5, 2, 2);
    draw_char_at(5, 3, 1);
}

fn draw_char_at(row: u16, column: u16, char: u8) {
    lcd_column_range(column * 6, column * 6 + 5);
    lcd_row_range(row * 8, row * 8 + 7);

    lcd_command(LcdCommand::MemoryWrite);

    for char_row in 0..8 {
        for char_column in 0..6 {
            let on = font::FONT[char as usize][char_column] & (1 << char_row) != 0;

            lcd_data(0x00); // blue
            lcd_data(if on {0xff} else {0x00}); // green
            lcd_data(0x00); // red
        }
    }
}

fn lcd_column_range(first_column: u16, last_column: u16) {
    lcd_command(LcdCommand::ColumnAddressSet);
    lcd_data((first_column >> 8 & 0xff) as u8);
    lcd_data((first_column & 0xff) as u8);
    lcd_data((last_column >> 8 & 0xff) as u8);
    lcd_data((last_column & 0xff) as u8);
}

fn lcd_row_range(first_row: u16, last_row: u16) {
    lcd_command(LcdCommand::RowAddressSet);
    lcd_data((first_row >> 8 & 0xff) as u8);
    lcd_data((first_row & 0xff) as u8);
    lcd_data((last_row >> 8 & 0xff) as u8);
    lcd_data((last_row & 0xff) as u8);
}

enum LcdCommand {
    ExitSleepMode = 0x11,
    DisplayOn = 0x29,
    ColumnAddressSet = 0x2a,
    RowAddressSet = 0x2b,
    MemoryWrite = 0x2c,
}

fn lcd_command(cmd: LcdCommand) {
    SPI5.flush();

    // Set data/command select low
    GPIOD.set_low(13);

    // Turn chip select on (low)
    GPIOC.set_low(2);

    // Transmit the command byte
    SPI5.write_byte_flush(cmd as u8);

    // Turn chip select off (high)
    GPIOC.set_high(2);
}

fn lcd_data(data: u8) {
    // Set data/command select high
    GPIOD.set_high(13);

    // Turn chip select on (low)
    GPIOC.set_low(2);

    // Transmit the data byte
    SPI5.write_byte(data);

    // For the sake of speed, we don't turn chip select off at this point
}
