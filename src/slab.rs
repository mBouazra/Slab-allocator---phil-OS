use core::ptr::null_mut;
use crate::free_list::FreeList;

pub struct Slab {
    object_size: usize,
    free_list: FreeList,
    base: *mut u8,
    capacity: usize,
}

impl Slab {
    pub const fn empty(object_size: usize) -> Self {
        Slab {
            object_size,
            free_list: FreeList::new(),
            base: null_mut(),
            capacity: 0,
        }
    }

    /// # Safety
    /// - `base` must point to valid memory of `object_size * count` bytes
    /// - `object_size` must be >= `size_of::<*mut u8>()`
    pub unsafe fn new(base: *mut u8, object_size: usize, count: usize) -> Self {
        let mut slab = Slab {
            object_size,
            free_list: FreeList::new(),
            base,
            capacity: count,
        };
        for i in 0..count {
            let ptr = base.add(i * object_size);
            slab.free_list.push(ptr);
        }
        slab
    }

    /// # Safety
    /// - Returned pointer valid until freed
    pub unsafe fn alloc(&mut self) -> *mut u8 {
        self.free_list.pop().unwrap_or(null_mut())
    }

    /// # Safety
    /// - `ptr` must come from `alloc()` on this slab
    pub unsafe fn free(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            self.free_list.push(ptr);
        }
    }

    pub fn object_size(&self) -> usize {
        self.object_size
    }

    pub fn base(&self) -> *mut u8 {
        self.base
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

unsafe impl Send for Slab {}