#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
mod free_list;
mod slab;
mod allocator;
mod utils;

use bootloader::{entry_point, BootInfo};
use allocator::SlabAllocator;

static ALLOCATOR: SlabAllocator = SlabAllocator::new();

static mut HEAP: [u8; 65536] = [0; 65536];

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    vga_buffer::print_string("Slab Allocator Kernel\n");
    vga_buffer::print_string("By BOUAZRA Mehdi & MALIH Omar\n");
    
    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr(), HEAP.len());
    }

    vga_buffer::print_string("Allocator initialized!\n");
    

    loop {}
}



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::print_string("KERNEL PANIC!\n");
    if let Some(location) = info.location() {
        vga_buffer::print_string("File: ");
        vga_buffer::print_string(location.file());
        vga_buffer::print_string("\n");
    }
    loop {}
}