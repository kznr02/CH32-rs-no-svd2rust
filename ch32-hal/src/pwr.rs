use vcell::VolatileCell;

use crate::RegOpu32;

const PWR_BASE_ADDR: usize = 0x40007000;
const PWR_CTLR_ADDR: usize = PWR_BASE_ADDR + 0x00;
const PWR_CSR_ADDR: usize = PWR_BASE_ADDR + 0x04;

pub struct RegPwrCtlr(VolatileCell<*mut u32>);

impl RegPwrCtlr {
    fn new() -> Self {
        Self(VolatileCell::new(PWR_CTLR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegPwrCtlr {
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

pub struct RegPwrCsr(VolatileCell<*mut u32>);

impl RegPwrCsr {
    fn new() -> Self {
        Self(VolatileCell::new(PWR_CSR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegPwrCsr {
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

pub struct PWR {
    pub ctlr: RegPwrCtlr,
    pub csr: RegPwrCsr,
}

impl PWR {
    pub fn new() -> Self {
        Self {
            ctlr: RegPwrCtlr::new(),
            csr: RegPwrCsr::new(),
        }
    }
}
