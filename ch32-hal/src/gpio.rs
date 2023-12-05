use vcell::VolatileCell;

use crate::RegOpu32;

const GPIO_BASE_ADDR: usize = 0x40010800;
const GPIOA_CFGLR_ADDR: usize = GPIO_BASE_ADDR + 0x0000;
const GPIOB_CFGLR_ADDR: usize = GPIO_BASE_ADDR + 0x0400;
const GPIOC_CFGLR_ADDR: usize = GPIO_BASE_ADDR + 0x0800;
const GPIOD_CFGLR_ADDR: usize = GPIO_BASE_ADDR + 0x0C00;
const GPIOA_CFGHR_ADDR: usize = GPIOA_CFGLR_ADDR + 0x04;
const GPIOB_CFGHR_ADDR: usize = GPIOB_CFGLR_ADDR + 0x04;
const GPIOC_CFGHR_ADDR: usize = GPIOC_CFGLR_ADDR + 0x04;
const GPIOD_CFGHR_ADDR: usize = GPIOD_CFGLR_ADDR + 0x04;
const GPIOA_INDR_ADDR: usize = GPIOA_CFGLR_ADDR + 0x08;
const GPIOB_INDR_ADDR: usize = GPIOB_CFGLR_ADDR + 0x08;
const GPIOC_INDR_ADDR: usize = GPIOC_CFGLR_ADDR + 0x08;
const GPIOD_INDR_ADDR: usize = GPIOD_CFGLR_ADDR + 0x08;
const GPIOA_OUTDR_ADDR: usize = GPIOA_CFGLR_ADDR + 0x0c;
const GPIOB_OUTDR_ADDR: usize = GPIOB_CFGLR_ADDR + 0x0c;
const GPIOC_OUTDR_ADDR: usize = GPIOC_CFGLR_ADDR + 0x0c;
const GPIOD_OUTDR_ADDR: usize = GPIOD_CFGLR_ADDR + 0x0c;
const GPIOA_BSHR_ADDR: usize = GPIOA_CFGLR_ADDR + 0x10;
const GPIOB_BSHR_ADDR: usize = GPIOB_CFGLR_ADDR + 0x10;
const GPIOC_BSHR_ADDR: usize = GPIOC_CFGLR_ADDR + 0x10;
const GPIOD_BSHR_ADDR: usize = GPIOD_CFGLR_ADDR + 0x10;
const GPIOA_BCR_ADDR: usize = GPIOA_CFGLR_ADDR + 0x14;
const GPIOB_BCR_ADDR: usize = GPIOB_CFGLR_ADDR + 0x14;
const GPIOC_BCR_ADDR: usize = GPIOC_CFGLR_ADDR + 0x14;
const GPIOD_BCR_ADDR: usize = GPIOD_CFGLR_ADDR + 0x14;
const GPIOA_LCKR_ADDR: usize = GPIOA_CFGLR_ADDR + 0x18;
const GPIOB_LCKR_ADDR: usize = GPIOB_CFGLR_ADDR + 0x18;
const GPIOC_LCKR_ADDR: usize = GPIOC_CFGLR_ADDR + 0x18;
const GPIOD_LCKR_ADDR: usize = GPIOD_CFGLR_ADDR + 0x18;

pub struct RegGpioACfglr(VolatileCell<*mut u32>);
impl RegGpioACfglr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOA_CFGLR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioACfglr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe {
            self.0.get().write_volatile(f(self.read()));
        }
    }
}

pub struct RegGpioBCfglr(VolatileCell<*mut u32>);
impl RegGpioBCfglr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOB_CFGLR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioBCfglr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe {
            self.0.get().write_volatile(f(self.read()));
        }
    }
}

pub struct RegGpioCCfglr(VolatileCell<*mut u32>);
impl RegGpioCCfglr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOC_CFGLR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioCCfglr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe {
            self.0.get().write_volatile(f(self.read()));
        }
    }
}

pub struct RegGpioDCfglr(VolatileCell<*mut u32>);
impl RegGpioDCfglr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOD_CFGLR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioDCfglr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe {
            self.0.get().write_volatile(f(self.read()));
        }
    }
}

pub struct RegGpioACfghr(VolatileCell<*mut u32>);
impl RegGpioACfghr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOA_CFGHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioACfghr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioBCfghr(VolatileCell<*mut u32>);
impl RegGpioBCfghr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOB_CFGHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioBCfghr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioCCfghr(VolatileCell<*mut u32>);
impl RegGpioCCfghr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOC_CFGHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioCCfghr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioDCfghr(VolatileCell<*mut u32>);
impl RegGpioDCfghr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOD_CFGHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioDCfghr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioAIndr(VolatileCell<*mut u32>);
impl RegGpioAIndr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOA_INDR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioAIndr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioBIndr(VolatileCell<*mut u32>);
impl RegGpioBIndr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOB_INDR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioBIndr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioCIndr(VolatileCell<*mut u32>);
impl RegGpioCIndr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOC_INDR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioCIndr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioDIndr(VolatileCell<*mut u32>);
impl RegGpioDIndr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOD_INDR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioDIndr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioAOutdr(VolatileCell<*mut u32>);
impl RegGpioAOutdr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOA_OUTDR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioAOutdr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioBOutdr(VolatileCell<*mut u32>);
impl RegGpioBOutdr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOB_OUTDR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioBOutdr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioCOutdr(VolatileCell<*mut u32>);
impl RegGpioCOutdr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOC_OUTDR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioCOutdr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioDOutdr(VolatileCell<*mut u32>);

impl RegGpioDOutdr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOD_OUTDR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegGpioDOutdr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioABshr(VolatileCell<*mut u32>);
impl RegGpioABshr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOA_BSHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioABshr {
    fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe { self.0.get().write_volatile(f(self.read())) }
    }
}

pub struct RegGpioBBshr(VolatileCell<*mut u32>);
impl RegGpioBBshr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOB_BSHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioBBshr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioCBshr(VolatileCell<*mut u32>);
impl RegGpioCBshr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOC_BSHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioCBshr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioDBshr(VolatileCell<*mut u32>);
impl RegGpioDBshr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOD_BSHR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioDBshr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioABcr(VolatileCell<*mut u32>);
impl RegGpioABcr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOA_BCR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioABcr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioBBcr(VolatileCell<*mut u32>);
impl RegGpioBBcr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOB_BCR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioBBcr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioCBcr(VolatileCell<*mut u32>);
impl RegGpioCBcr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOC_BCR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioCBcr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioDBcr(VolatileCell<*mut u32>);
impl RegGpioDBcr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOD_BCR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioDBcr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioALckr(VolatileCell<*mut u32>);
impl RegGpioALckr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOA_LCKR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioALckr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioBLckr(VolatileCell<*mut u32>);
impl RegGpioBLckr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOB_LCKR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioBLckr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioCLckr(VolatileCell<*mut u32>);
impl RegGpioCLckr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOC_LCKR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioCLckr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct RegGpioDLckr(VolatileCell<*mut u32>);
impl RegGpioDLckr {
    fn new() -> Self {
        Self(VolatileCell::new(GPIOD_LCKR_ADDR as *mut u32))
    }
}
impl RegOpu32 for RegGpioDLckr {
    fn read(&self) -> u32 {
        unsafe {
            self.0.get().read_volatile()
        }
    }

    fn write<F>(&mut self, f: F)
        where
            F: FnOnce(u32) -> u32 {
        unsafe {
            self.0.get().write_volatile(f(self.read()))
        }
    }
}

pub struct GPIOA {
    pub cfglr: RegGpioACfglr,
    pub cfghr: RegGpioACfghr,
    pub indr: RegGpioAIndr,
    pub outdr: RegGpioAOutdr,
    pub bshr: RegGpioABshr,
    pub bcr: RegGpioABcr,
    pub lckr: RegGpioALckr
}
impl GPIOA {
    pub fn new() -> Self {
        Self {
            cfglr: RegGpioACfglr::new(),
            cfghr: RegGpioACfghr::new(),
            indr: RegGpioAIndr::new(),
            outdr: RegGpioAOutdr::new(),
            bshr: RegGpioABshr::new(),
            bcr: RegGpioABcr::new(),
            lckr: RegGpioALckr::new()
        }
    }
}

pub struct GPIOB {
    pub cfglr: RegGpioBCfglr,
    pub cfghr: RegGpioBCfghr,
    pub indr: RegGpioBIndr,
    pub outdr: RegGpioBOutdr,
    pub bshr: RegGpioBBshr,
    pub bcr: RegGpioBBcr,
    pub lckr: RegGpioBLckr
}
impl GPIOB {
    pub fn new() -> Self {
        Self {
            cfglr: RegGpioBCfglr::new(),
            cfghr: RegGpioBCfghr::new(),
            indr: RegGpioBIndr::new(),
            outdr: RegGpioBOutdr::new(),
            bshr: RegGpioBBshr::new(),
            bcr: RegGpioBBcr::new(),
            lckr: RegGpioBLckr::new()
        }
    }
}

pub struct GPIOC {
    pub cfglr: RegGpioCCfglr,
    pub cfghr: RegGpioCCfghr,
    pub indr: RegGpioCIndr,
    pub outdr: RegGpioCOutdr,
    pub bshr: RegGpioCBshr,
    pub bcr: RegGpioCBcr,
    pub lckr: RegGpioCLckr
}
impl GPIOA {
    pub fn new() -> Self {
        Self {
            cfglr: RegGpioCCfglr::new(),
            cfghr: RegGpioCCfghr::new(),
            indr: RegGpioCIndr::new(),
            outdr: RegGpioCOutdr::new(),
            bshr: RegGpioCBshr::new(),
            bcr: RegGpioCBcr::new(),
            lckr: RegGpioCLckr::new()
        }
    }
}

pub struct GPIOD {
    pub cfglr: RegGpioDCfglr,
    pub cfghr: RegGpioDCfghr,
    pub indr: RegGpioDIndr,
    pub outdr: RegGpioDOutdr,
    pub bshr: RegGpioDBshr,
    pub bcr: RegGpioDBcr,
    pub lckr: RegGpioDLckr
}
impl GPIOD {
    pub fn new() -> Self {
        Self {
            cfglr: RegGpioDCfglr::new(),
            cfghr: RegGpioDCfghr::new(),
            indr: RegGpioDIndr::new(),
            outdr: RegGpioDOutdr::new(),
            bshr: RegGpioDBshr::new(),
            bcr: RegGpioDBcr::new(),
            lckr: RegGpioDLckr::new()
        }
    }
}
