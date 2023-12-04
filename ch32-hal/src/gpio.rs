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

pub struct GPIOA {
    pub cfglr: RegGpioACfglr,
}

impl GPIOA {
    pub fn new() -> Self {
        Self {
            cfglr: RegGpioACfglr::new(),
        }
    }
}
