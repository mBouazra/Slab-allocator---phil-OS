	use core::ptr::null_mut;

pub struct FreeList {
  head: *mut FreeNode,
}

impl FreeList {
  pub const  fn new() -> Self {
   FreeList {
    head: null_mut()
        }
    }
 pub unsafe fn push(&mut self, ptr: *mut u8) {
        *(ptr as *mut *mut u8) = self.head;
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
}
