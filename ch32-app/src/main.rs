#![no_std]
#![no_main]
#![allow(unused)]
#![feature(lazy_cell)]
use panic_halt as _;

use ch32_hal::{gpio::GPIOA, rcc::RCC, *};
use core::{cell::LazyCell, ptr};
use riscv::asm;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    system_init();
    let mut gpioa = GPIOA::new();
    gpioa.cfglr.write(|mut v| {
        v &= 0xfffffff0;
        // v |= (0x02 << 2);
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
        start_up_counter.set(start_up_counter.get()+1);
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
        while rcc.ctlr.read() & 0x02000000 == 0 {
            
        }
        rcc.cfgr0.write(|mut v| {
            v &= !0x03;
            v
        });
        rcc.cfgr0.write(|mut v| {
            v |= 0x02;
            v
        });
        while rcc.cfgr0.read() & 0x0c != 0x08 {
            
        }

        unsafe {
            ptr::write_volatile(0x4002101c as *mut u32 , 3 << 27);
            ptr::write_volatile(0x40007000 as *mut u32, 1 << 8);
            ptr::write_volatile(0x40006c04 as *mut u16, 1);
        }
    } else {
        // user define
    }
}

fn gpioa0_led_test() {
    let mut current_mode: u32 = 0x00;
    let mut current_pin: u32 = 0x00;
    let mut pinpos: u32 = 0x00;
    let mut pos: u32 = 0;
    let mut tmpreg: u32 = 0x00;
    let mut pinmask: u32 = 0x00;

    current_mode = 0x10 & 0x0f;
    if (0x10 & 0x10) != 0x00 {
        current_mode |= 1;
    }

    if (0x01 & 0xff) != 0x00 {
        todo!()
    }

}