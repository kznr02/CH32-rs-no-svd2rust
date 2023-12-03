#![no_std]
#![no_main]
#![allow(unused)]
#![feature(lazy_cell)]
use panic_halt as _;

use ch32_hal::{*, rcc::RCC};
use core::cell::LazyCell;
use riscv::asm;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    system_init();

    loop {
        
    }
}

fn system_init() {
    let mut rcc = RCC::new();
    rcc.ctlr.write(|mut v| {
        v |= 0x00000001;
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
    })

}