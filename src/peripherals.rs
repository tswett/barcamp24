use core::ops::{BitOr, BitAnd, Not};

pub trait Bitwise: Copy + BitOr<Output = Self> + BitAnd<Output = Self> + Not<Output = Self> { }
impl Bitwise for u16 { }
impl Bitwise for u32 { }

pub trait Register<T: Bitwise> {
    unsafe fn read(&self) -> T;
    unsafe fn write(&self, value: T);

    unsafe fn write_or(&self, value: T) {
        let old_reg_value = self.read();
        let new_reg_value = old_reg_value | value;
        self.write(new_reg_value);
    }

    unsafe fn write_masked(&self, mask: T, value: T) {
        let old_reg_value = self.read();
        let new_reg_value = (old_reg_value & !mask) | (value & mask);
        self.write(new_reg_value);
    }
}

pub struct Register16 {
    addr: u32,
}

impl Register<u16> for Register16 {
    unsafe fn read(&self) -> u16 {
        let ptr = self.addr as *const u16;
        ptr.read_volatile()
    }

    unsafe fn write(&self, value: u16) {
        let ptr = self.addr as *mut u16;
        ptr.write_volatile(value);
    }
}

pub struct Register32 {
    addr: u32,
}

impl Register<u32> for Register32 {
    unsafe fn read(&self) -> u32 {
        let ptr = self.addr as *const u32;
        ptr.read_volatile()
    }

    unsafe fn write(&self, value: u32) {
        let ptr = self.addr as *mut u32;
        ptr.write_volatile(value);
    }
}

pub struct RccAhb1enr;

impl RccAhb1enr {
    const REG: Register32 = Register32 { addr: 0x40023830 };

    pub fn enable_gpioa(&self) {
        unsafe {
            Self::REG.write_or(1 << 0);
        }
    }

    pub fn enable_gpioc(&self) {
        unsafe {
            Self::REG.write_or(1 << 2);
        }
    }

    pub fn enable_gpiod(&self) {
        unsafe {
            Self::REG.write_or(1 << 3);
        }
    }

    pub fn enable_gpiof(&self) {
        unsafe {
            Self::REG.write_or(1 << 5);
        }
    }

    pub fn enable_gpiog(&self) {
        unsafe {
            Self::REG.write_or(1 << 6);
        }
    }
}

pub const RCC_AHB1ENR: RccAhb1enr = RccAhb1enr;

pub struct RccApb2enr;

impl RccApb2enr {
    const REG: Register32 = Register32 { addr: 0x40023844 };

    pub fn enable_spi5(&self) {
        unsafe {
            Self::REG.write_or(1 << 20);
        }
    }

    pub fn enable_usart1(&self) {
        unsafe {
            Self::REG.write_or(1 << 4);
        }
    }
}

pub const RCC_APB2ENR: RccApb2enr = RccApb2enr;

pub struct Gpio {
    base: u32,
}

impl Gpio {
    pub fn set_mode(&self, pin: u8, mode: GpioMode) {
        let moder = Register32 { addr: self.base + 0x00 };
        let mask = 0b11 << (2 * pin);
        let value = (mode as u32) << (2 * pin);

        unsafe {
            moder.write_masked(mask, value);
        }
    }

    pub fn set_low(&self, pin: u8) {
        let bsrr = Register32 { addr: self.base + 0x18 };
        let value = 1 << (pin + 16);

        unsafe {
            bsrr.write(value);
        }
    }

    pub fn set_high(&self, pin: u8) {
        let bsrr = Register32 { addr: self.base + 0x18 };
        let value = 1 << pin;

        unsafe {
            bsrr.write(value);
        }
    }

    pub fn set_alt_func(&self, pin: u8, alt_func: AltFunc) {
        let afr: Register32;
        let offset: u8;

        if pin < 8 {
            afr = Register32 { addr: self.base + 0x20 };
            offset = pin * 4;
        } else {
            afr = Register32 { addr: self.base + 0x24 };
            offset = (pin - 8) * 4;
        }

        let mask = 0b1111 << offset;
        let value = (alt_func as u32) << offset;

        unsafe {
            afr.write_masked(mask, value);
        }
    }
}

pub const GPIOA: Gpio = Gpio { base: 0x40020000 };
pub const GPIOC: Gpio = Gpio { base: 0x40020800 };
pub const GPIOD: Gpio = Gpio { base: 0x40020c00 };
pub const GPIOF: Gpio = Gpio { base: 0x40021400 };
pub const GPIOG: Gpio = Gpio { base: 0x40021800 };

pub enum GpioMode {
    Output = 0b01,
    AltFunc = 0b10,
}

pub enum AltFunc {
    AF5 = 5,
    AF7 = 7,
}

pub struct Usart {
    base: u32,
}

impl Usart {
    fn sr(&self) -> Register16 { Register16 { addr: self.base + 0x00 } }
    fn dr(&self) -> Register16 { Register16 { addr: self.base + 0x04 } }

    pub fn set_brr(&self, value: u16) {
        let brr = Register16 { addr: self.base + 0x08 };

        unsafe {
            brr.write(value);
        }
    }

    pub fn enable_rx_tx(&self) {
        let cr1 = Register16 { addr: self.base + 0x0c };
        let ue_enable = 1 << 13;
        let te_enable = 1 << 3;
        let re_enable = 1 << 2;

        unsafe {
            cr1.write_or(ue_enable | te_enable | re_enable);
        }
    }

    pub fn byte_received(&self) -> bool {
        unsafe {
            self.sr().read() & (1 << 5) != 0
        }
    }

    pub fn transmit_byte(&self, byte: u8) {
        unsafe {
            while self.sr().read() & (1 << 7) == 0 {}
            self.dr().write(byte as u16);
        }
    }

    pub fn receive_byte(&self) -> u8 {
        unsafe {
            while !self.byte_received() {}
            self.dr().read() as u8
        }
    }
}

pub const USART1: Usart = Usart { base: 0x40011000 };

pub struct Spi {
    base: u32,
}

impl Spi {
    fn cr1(&self) -> Register16 { Register16 { addr: self.base + 0x00 } }
    fn sr(&self) -> Register16 { Register16 { addr: self.base + 0x08 } }
    fn dr(&self) -> Register16 { Register16 { addr: self.base + 0x0c } }

    pub fn set_baud_divisor(&self, div: SpiBaudDivisor) {
        unsafe {
            self.cr1().write_masked(0b111 << 3, div as u16);
        }
    }

    pub fn software_sub_management_sub_select_high_main_mode_spi_enable(&self) {
        let ssm_enable = 1 << 9;
        let ssi_high = 1 << 8;
        let spe_enable = 1 << 6;
        let mstr_main = 1 << 2;

        unsafe {
            self.cr1().write_or(ssm_enable | ssi_high | spe_enable | mstr_main);
        }
    }

    pub fn write_byte(&self, b: u8) {
        unsafe {
            while !self.transmit_buf_empty() {}
            self.dr().write(b as u16);
        }
    }

    pub fn flush(&self) {
        while !self.transmit_buf_empty() {}
        while self.busy() {}
    }

    pub fn write_byte_flush(&self, b: u8) {
        self.write_byte(b);
        self.flush();
    }

    pub fn transmit_buf_empty(&self) -> bool {
        unsafe {
            let status_word = self.sr().read();
            let result = (status_word & (1 << 1)) != 0;
            return result;
        }
    }

    pub fn busy(&self) -> bool {
        unsafe {
            let status_word = self.sr().read();
            let result = (status_word & (1 << 7)) != 0;
            return result;
        }
    }
}

pub const SPI5: Spi = Spi { base: 0x40015000 };

pub enum SpiBaudDivisor {
    Div2 = 0 << 3,
}
