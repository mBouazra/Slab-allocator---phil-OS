use core::ptr::null_mut;
use crate::free_list::FreeList;

pub struct Slab {
    object_size: usize,
    free_list: FreeList,
}

impl Slab {
    pub const fn new(object_size: usize) -> Self {
        Slab {
            object_size,
            free_list: FreeList::new(),
        }
    }

    pub unsafe fn alloc(&mut self) -> *mut u8 {
        self.free_list.pop().unwrap_or(null_mut())
    }

    pub unsafe fn free(&mut self, ptr: *mut u8) {
        self.free_list.push(ptr);
    }

    pub fn size(&self) -> usize {
        self.object_size
    }
}
