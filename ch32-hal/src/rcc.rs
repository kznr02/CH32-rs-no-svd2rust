use vcell::VolatileCell;

use crate::RegOpu32;

const RCC_BASE_ADDR: usize = 0x40021000;
const RCC_CTLR_ADDR: usize = RCC_BASE_ADDR + 0x00;
const RCC_CFGR0_ADDR: usize = RCC_BASE_ADDR + 0x04;
const RCC_INTR_ADDR: usize = RCC_BASE_ADDR + 0x08;
const RCC_APB2PRSTR_ADDR: usize = RCC_BASE_ADDR + 0x0C;
const RCC_APB1PRSTR_ADDR: usize = RCC_BASE_ADDR + 0x10;
const RCC_AHBPCENR_ADDR: usize = RCC_BASE_ADDR + 0x14;
const RCC_APB2PCENR_ADDR: usize = RCC_BASE_ADDR + 0x18;
const RCC_APB1PCENR_ADDR: usize = RCC_BASE_ADDR + 0x1C;
const RCC_BDCTLR_ADDR: usize = RCC_BASE_ADDR + 0x20;
const RCC_RSTSCKR_ADDR: usize = RCC_BASE_ADDR + 0x24;
const RCC_AHBRSTR: usize = RCC_BASE_ADDR + 0x28;

pub struct RegRccCtlr {
    ptr: VolatileCell<*mut u32>,
}

impl RegRccCtlr {
    fn new() -> Self {
        Self {
            ptr: VolatileCell::new(RCC_CTLR_ADDR as *mut u32),
        }
    }
}

impl RegOpu32 for RegRccCtlr {
    fn read(&self) -> u32 {
        unsafe { self.ptr.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe {
            self.ptr.get().write_volatile(f(self.read()));
        }
    }
}

pub struct RegRccCfgr0 {
    ptr: VolatileCell<*mut u32>,
}

impl RegRccCfgr0 {
    fn new() -> Self {
        Self {
            ptr: VolatileCell::new(RCC_CFGR0_ADDR as *mut u32),
        }
    }
}

impl RegOpu32 for RegRccCfgr0 {
    fn read(&self) -> u32 {
        unsafe { self.ptr.get().read_volatile() }
    }

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        unsafe {
            self.ptr.get().write_volatile(f(self.read()));
        }
    }
}

pub struct RegRccintr(VolatileCell<*mut u32>);

impl RegRccintr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_INTR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccintr {
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

pub struct RegRccApb2prstr(VolatileCell<*mut u32>);

impl RegRccApb2prstr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_APB2PRSTR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccApb2prstr {
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

pub struct RegRccApb1prstr(VolatileCell<*mut u32>);

impl RegRccApb1prstr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_APB1PRSTR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccApb1prstr {
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

pub struct RegRccAhbpcenr(VolatileCell<*mut u32>);

impl RegRccAhbpcenr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_AHBPCENR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccAhbpcenr {
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

pub struct RegRccApb2pcenr(VolatileCell<*mut u32>);

impl RegRccApb2pcenr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_APB2PCENR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccApb2pcenr {
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

pub struct RegRccApb1pcenr(VolatileCell<*mut u32>);

impl RegRccApb1pcenr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_APB1PCENR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccApb1pcenr {
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

pub struct RegRccBdctlr(VolatileCell<*mut u32>);

impl RegRccBdctlr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_BDCTLR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccBdctlr {
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

pub struct RegRccRstsckr(VolatileCell<*mut u32>);

impl RegRccRstsckr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_RSTSCKR_ADDR as *mut u32))
    }
}

impl RegOpu32 for RegRccRstsckr {
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

pub struct RegRccAhbrstr(VolatileCell<*mut u32>);

impl RegRccAhbrstr {
    fn new() -> Self {
        Self(VolatileCell::new(RCC_AHBRSTR as *mut u32))
    }
}

impl RegOpu32 for RegRccAhbrstr {
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
pub struct RCC {
    pub ctlr: RegRccCtlr,
    pub cfgr0: RegRccCfgr0,
    pub intr: RegRccintr,
    pub apb2prstr: RegRccApb2prstr,
    pub apb1prstr: RegRccApb1prstr,
    pub ahbpcenr: RegRccAhbpcenr,
    pub apb2pcenr: RegRccApb2pcenr,
    pub apb1pcenr: RegRccApb1pcenr,
    pub bdctlr: RegRccBdctlr,
    pub rstsckr: RegRccRstsckr,
    pub ahbrstr: RegRccAhbrstr,
}

impl RCC {
    pub fn new() -> Self {
        Self {
            ctlr: RegRccCtlr::new(),
            cfgr0: RegRccCfgr0::new(),
            intr: RegRccintr::new(),
            apb1prstr: RegRccApb1prstr::new(),
            apb2prstr: RegRccApb2prstr::new(),
            ahbpcenr: RegRccAhbpcenr::new(),
            apb2pcenr: RegRccApb2pcenr::new(),
            apb1pcenr: RegRccApb1pcenr::new(),
            bdctlr: RegRccBdctlr::new(),
            rstsckr: RegRccRstsckr::new(),
            ahbrstr: RegRccAhbrstr::new(),
        }
    }
}
