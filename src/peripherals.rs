pub struct Register32 {
    addr: u32,
}

impl Register32 {
    pub unsafe fn read(&self) -> u32 {
        let ptr = self.addr as *const u32;
        ptr.read_volatile()
    }

    pub unsafe fn write(&self, value: u32) {
        let ptr = self.addr as *mut u32;
        ptr.write_volatile(value);
    }

    pub unsafe fn write_or(&self, value: u32) {
        let old_reg_value = self.read();
        let new_reg_value = old_reg_value | value;
        self.write(new_reg_value);
    }

    pub unsafe fn write_masked(&self, mask: u32, value: u32) {
        let old_reg_value = self.read();
        let new_reg_value = (old_reg_value & !mask) | (value & mask);
        self.write(new_reg_value);
    }
}

pub struct RccAhb1enr;

impl RccAhb1enr {
    pub fn enable_gpiog(&self) {
        let reg = Register32 { addr: 0x40023830 };

        unsafe {
            reg.write_or(1 << 6);
        }
    }
}

pub const RCC_AHB1ENR: RccAhb1enr = RccAhb1enr;

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

    pub fn set_high(&self, pin: u8) {
        let bsrr = Register32 { addr: self.base + 0x18 };
        let value = 1 << pin;

        unsafe {
            bsrr.write(value);
        }
    }
}

pub const GPIOG: Gpio = Gpio { base: 0x40021800 };

pub enum GpioMode {
    Output = 0b01,
}
