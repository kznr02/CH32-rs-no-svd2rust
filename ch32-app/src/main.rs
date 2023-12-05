#![no_std]
#![no_main]
#![allow(unused)]
#![feature(lazy_cell)]
use panic_halt as _;

use ch32_hal::{gpio::GPIOA, pwr::PWR, rcc::RCC, *};
use core::{cell::LazyCell, ptr};
use riscv::asm;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    system_init();
    let mut gpioa = GPIOA::new();

    gpioa.cfglr.write(|mut v| {
        v &= 0xfffffff0;
        v |= (0x01);
        v
    });
    loop {}
}

fn system_init() {
    let mut rcc = RCC::new();
    rcc.ctlr.write(|mut v| {
        v |= (0x01 << 0);
        v
    });

    rcc.cfgr0.write(|mut v| {
        v &= 0xf8ff0000;
        v
    });

    rcc.ctlr.write(|mut v| {
        v &= 0xfef6ffff;
        v
    });

    rcc.ctlr.write(|mut v| {
        v &= 0xfffbffff;
        v
    });

    rcc.cfgr0.write(|mut v| {
        v &= 0xff80ffff;
        v
    });

    rcc.intr.write(|mut v| {
        v = 0x009f0000;
        v
    });

    use vcell::VolatileCell;
    let start_up_counter = VolatileCell::new(0);
    let hse_status = VolatileCell::new(0);
    rcc.ctlr.write(|mut v| {
        v |= 0x00010000;
        v
    });

    while hse_status.get() == 0 && start_up_counter.get() != 0x500 {
        hse_status.set(rcc.ctlr.read() & 0x00020000);
        start_up_counter.set(start_up_counter.get() + 1);
    }

    if (rcc.ctlr.read() & 0x00020000) != 0 {
        hse_status.set(0x01);
    } else {
        hse_status.set(0x00);
    }

    if hse_status.get() == 0x01 {
        rcc.cfgr0.write(|mut v| {
            v |= 0;
            v
        });
        rcc.cfgr0.write(|mut v| {
            v |= 0;
            v
        });
        rcc.cfgr0.write(|mut v| {
            v |= 0x0400;
            v
        });
        rcc.cfgr0.write(|mut v| {
            v &= !(0x00100000 | 0x00200000 | 0x003c0000);
            v
        });
        rcc.ctlr.write(|mut v| {
            v |= 0x01000000;
            v
        });
        while rcc.ctlr.read() & 0x02000000 == 0 {}
        rcc.cfgr0.write(|mut v| {
            v &= !0x03;
            v
        });
        rcc.cfgr0.write(|mut v| {
            v |= 0x02;
            v
        });
        while rcc.cfgr0.read() & 0x0c != 0x08 {}

        let mut pwr = PWR::new();
        pwr.ctlr.write(|mut v| {
            v |= (1 << 8);
            v
        });
        rcc.apb1pcenr.write(|mut v| {
            v |= (3 << 27);
            v
        });

        rcc.apb2pcenr.write(|mut v| {
            v |= (1 << 2);
            v
        })
    } else {
        // user define
    }
}
