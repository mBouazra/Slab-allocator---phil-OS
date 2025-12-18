use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;

use crate::slab::Slab;

pub struct SlabAllocator {   slab_32: Mutex<Slab>,  slab_64: Mutex<Slab>,   slab_128: Mutex<Slab>,
}

impl SlabAllocator {
    pub const fn new() -> Self {
        SlabAllocator {
            slab_32: Mutex::new(Slab::new(32)),  slab_64: Mutex::new(Slab::new(64)),    slab_128: Mutex::new(Slab::new(128)),
        }
    }
}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match layout.size() {
            0..=32 => self.slab_32.lock().alloc(),
            33..=64 => self.slab_64.lock().alloc(),
            65..=128 => self.slab_128.lock().alloc(),
            _ => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match layout.size() {
            0..=32 => self.slab_32.lock().free(ptr),
            33..=64 => self.slab_64.lock().free(ptr),
            65..=128 => self.slab_128.lock().free(ptr),
            _ => {}
        }
    }
}
