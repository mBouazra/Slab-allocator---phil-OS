	use core::ptr;

pub struct FreeList {
  head: *mut FreeNode,
}

struct FreeNode {
  next: *mut FreeNode,
}

impl FreeList {
  pub fn new() -> Self {
   FreeList {
    head: ptr::null_mut()
        }
    }

   pub fn push(&mut self, ptr: *mut u8) {
   unsafe {
  let node = ptr as *mut FreeNode;
            (*node).next = self.head;
            self.head = node;
        }
       }

    pub fn pop(&mut self) -> *mut u8 {
    if self.head.is_null() {
    return ptr::null_mut();
        }

        unsafe {
        let node = self.head;
         self.head = (*node).next;
        node as *mut u8
        }
    }
}

