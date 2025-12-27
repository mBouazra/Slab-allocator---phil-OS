#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod free_list;
mod slab;
mod allocator;
mod utils;

use allocator::SlabAllocator;

static ALLOCATOR: SlabAllocator = SlabAllocator::new();

static mut HEAP: [u8; 65536] = [0; 65536];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let heap_ptr = unsafe { HEAP.as_mut_ptr() };
    let heap_size = 65536;
    unsafe {
        ALLOCATOR.init(heap_ptr, heap_size);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}