//! Slab Allocator Implementation
 
//! Authors: BOUAZRA MEHDI && MALIH OMAR

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod free_list;
pub mod slab;
pub mod allocator;
pub mod utils;



#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
