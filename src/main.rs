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
    lcd_set_up_driver();

    draw_demo();

    echo_characters_on_screen();
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

fn echo_characters_on_screen() -> ! {
    let mut row = 0;
    let mut column = 0;

    loop {
        draw_cursor_at(row, column);

        let byte = USART1.receive_byte();

        match byte {
            13 => {
                draw_char_at(row, column, 0);
                row += 1;
                column = 0;
            },
            27 => {
                let _byte2 = USART1.receive_byte();
                let byte3 = USART1.receive_byte();

                draw_char_at(row, column, 0);

                match byte3 {
                    65 => row -= 1,
                    66 => row += 1,
                    67 => column += 1,
                    68 => column -= 1,
                    _ => { },
                }
            }
            127 => {
                draw_char_at(row, column, 0);
                column -= 1;
            },
            _ => {
                draw_char_at(row, column, byte % 32);
                column += 1;
            },
        }

        if column >= 53 {
            row += 1;
        }

        if column < 0 {
            row -= 1;
        }

        row = row.rem_euclid(30);
        column = column.rem_euclid(53);
    }
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

fn lcd_set_up_driver() {
    lcd_command(LcdCommand::ExitSleepMode);

    lcd_command(LcdCommand::DisplayOn);

    lcd_command(LcdCommand::MemoryAccessControl);
    lcd_data(0b11100000); // reverse row order, reverse column order, exchange rows with columns
}

fn draw_demo() {
    draw_string_at(14, 9, "PLEASE TYPE SOME STUFF ON THE UART");
}

fn draw_string_at(row: i16, column: i16, str: &str) {
    let mut i = 0;

    for char_byte in str.as_bytes() {
        draw_char_at(row, column + i, *char_byte % 32);
        i += 1;
    }
}

fn draw_char_at(row: i16, column: i16, char: u8) {
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

fn draw_cursor_at(row: i16, column: i16) {
    lcd_column_range(column * 6, column * 6 + 5);
    lcd_row_range(row * 8, row * 8 + 7);

    lcd_command(LcdCommand::MemoryWrite);

    for _pixel in 0..(6*8) {
        lcd_data(0x00); // blue
        lcd_data(0x00); // green
        lcd_data(0xff); // red
    }
}

fn lcd_column_range(first_column: i16, last_column: i16) {
    lcd_command(LcdCommand::ColumnAddressSet);
    lcd_data((first_column >> 8 & 0xff) as u8);
    lcd_data((first_column & 0xff) as u8);
    lcd_data((last_column >> 8 & 0xff) as u8);
    lcd_data((last_column & 0xff) as u8);
}

fn lcd_row_range(first_row: i16, last_row: i16) {
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
    MemoryAccessControl = 0x36,
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
