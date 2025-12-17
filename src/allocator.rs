use crate::slab::Slab;

pub struct SlabAllocator {
    slab: Slab,
}

impl SlabAllocator {
    pub fn new(slab: Slab) -> Self {
        SlabAllocator { slab }
    }

    pub fn alloc(&mut self) -> *mut u8 {
        self.slab.alloc()
    }

    pub fn free(&mut self, ptr: *mut u8) {
        self.slab.free(ptr)
    }
}
