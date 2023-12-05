#![no_std]
#[allow(unused)]
#[allow(dead_code)]
use good_memory_allocator::SpinLockedAllocator;

pub mod gpio;
pub mod pwr;
pub mod rcc;

mod system;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

/// Register operation trait for 32 bits register
pub trait RegOpu32 {
    fn read(&self) -> u32;

    fn write<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32;
}
