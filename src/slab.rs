use crate::free_list::FreeList;

pub struct Slab {
    memory: *mut u8,
    free_list: FreeList,
    object_size: usize,
    capacity: usize,
}

impl Slab {
    pub fn new(memory: *mut u8, object_size: usize, capacity: usize) -> Self {
        let mut free_list = FreeList::new();

        // Découpe de la mémoire en objets fixes
        for i in 0..capacity {
            let ptr = unsafe {
                memory.add(i * object_size)
            };
            free_list.push(ptr);
        }

        Slab {
            memory,
            free_list,
            object_size,
            capacity,
        }
    }

    pub fn alloc(&mut self) -> *mut u8 {
        self.free_list.pop()
    }

    pub fn free(&mut self, ptr: *mut u8) {
        self.free_list.push(ptr)
    }
}
