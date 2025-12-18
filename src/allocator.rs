use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;

use crate::slab::Slab;

pub struct SlabAllocator {   slab_32: Mutex<Slab>,  
    slab_64: Mutex<Slab>,   slab_128: Mutex<Slab>,
}

impl SlabAllocator {
    pub const fn new() -> Self {
        SlabAllocator {
            slab_32: Mutex::new(Slab::new(32)),  
            slab_64: Mutex::new(Slab::new(64)),    
            slab_128: Mutex::new(Slab::new(128)),
        }
    }
}


pub unsafe fn init(&self, heap_start: *mut u8, heap_size: usize) {
        let region = heap_size / 3;

        let count_32 = region / 32;
        let count_64 = region / 64;
        let count_128 = region / 128;

        let mut offset = 0;
        *self.slab_32.lock() = Slab::new(heap_start.add(offset), 32, count_32);
        offset += count_32 * 32;

        *self.slab_64.lock() = Slab::new(heap_start.add(offset), 64, count_64);
        offset += count_64 * 64;

        *self.slab_128.lock() = Slab::new(heap_start.add(offset), 128, count_128);
    }


unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(layout.align());
        match layout.size() {
            0..=32 => self.slab_32.lock().alloc(),
            33..=64 => self.slab_64.lock().alloc(),
            65..=128 => self.slab_128.lock().alloc(),
            _ => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(layout.align());
        match layout.size() {
            0..=32 => self.slab_32.lock().free(ptr),
            33..=64 => self.slab_64.lock().free(ptr),
            65..=128 => self.slab_128.lock().free(ptr),
            _ => {}
        }
    }
}
