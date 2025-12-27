#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod free_list;
mod slab;
mod allocator;
mod utils;

use bootloader::{entry_point, BootInfo};
use allocator::SlabAllocator;

#[global_allocator]
static ALLOCATOR: SlabAllocator = SlabAllocator::new();

static mut HEAP: [u8; 65536] = [0; 65536];

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    
    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr(), HEAP.len());
    }


    loop {}
}



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
   
    loop {}
}
