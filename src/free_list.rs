	use core::ptr::null_mut;

pub struct FreeList {
  head: *mut u8,
}

impl FreeList {
  pub const  fn new() -> Self {
   FreeList {
    head: null_mut()
        }
    }

     
 pub unsafe fn push(&mut self, ptr: *mut u8) {
        let node = ptr as *mut *mut u8;
        *node = self.head;
        self.head = ptr;
    }


    pub unsafe fn pop(&mut self) -> Option<*mut u8> {
        if self.head.is_null() {
            None
        } else {
            let ptr = self.head;
            self.head = *(ptr as *mut *mut u8);
            Some(ptr)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }
}

unsafe impl Send for FreeList {}