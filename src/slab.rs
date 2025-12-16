use crate::free_list::FreeList;

pub struct Slab {
    memory: *mut u8,
    free_list: FreeList,
    object_size: usize,
    capacity: usize,
}

impl Slab {
    pub fn new(memory: *mut u8, object_size: usize, capacity: usize) -> Self {
     
        todo!()
    }

 pub fn alloc(&mut self) -> *mut u8 {
  self.free_list.pop() 
    }

  pub fn free(&mut self, ptr: *mut 
   self.free_list.push(ptr) 
    }
}
