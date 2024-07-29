#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

pub unsafe fn read_reg_32(addr: u32) -> u32 {
    let ptr = addr as *const u32;
    ptr.read_volatile()
}

pub unsafe fn write_reg_32(addr: u32, value: u32) {
    let ptr = addr as *mut u32;
    ptr.write_volatile(value);
}

pub unsafe fn masked_write_reg_32(addr: u32, mask: u32, value: u32) {
    let old_reg_value = read_reg_32(addr);
    let new_reg_value = (old_reg_value & !mask) | (value & mask);
    write_reg_32(addr, new_reg_value);
}

pub fn clock_enable_gpiog() {
    unsafe {
        let rcc_ahb1enr = 0x40023830;
        let gpiog_en = 1 << 6;
        let mut reg = read_reg_32(rcc_ahb1enr);
        reg |= gpiog_en;
        write_reg_32(rcc_ahb1enr, reg);
    }
}

pub fn conf_pg13_output() {
    unsafe {
        let gpiog_moder = 0x40021800;
        let moder13_mask = 0b11 << 26;
        let moder13_output = 0b01 << 26;
        masked_write_reg_32(gpiog_moder, moder13_mask, moder13_output);
    }
}

pub fn set_pg13_high() {
    unsafe {
        let gpiog_bsrr = 0x40021818;
        let bs13 = 1 << 13;
        write_reg_32(gpiog_bsrr, bs13);
    }
}

#[entry]
fn main() -> ! {
    clock_enable_gpiog();
    conf_pg13_output();
    set_pg13_high();

    loop {}
}
