use crate::slab::Slab;
use crate::free_list::FreeList;

#[test]
fn test_free_list_push_pop() {
    let mut buffer = [0u8; 64];
    let mut free_list = FreeList::new();
    unsafe {
        free_list.push(buffer.as_mut_ptr());
        assert_eq!(free_list.pop(), Some(buffer.as_mut_ptr()));
    }
}

#[test]
fn test_free_list_empty() {
    let mut free_list = FreeList::new();
    assert!(free_list.is_empty());
    unsafe {
        assert_eq!(free_list.pop(), None);
    }
}

#[test]
fn test_slab_alloc() {
    let mut buffer = [0u8; 1024];
    let mut slab = unsafe { Slab::new(buffer.as_mut_ptr(), 16, 64) };
    unsafe {
        let ptr = slab.alloc();
        assert!(!ptr.is_null());
    }
}

#[test]
fn test_slab_free_reuse() {
    let mut buffer = [0u8; 1024];
    let mut slab = unsafe { Slab::new(buffer.as_mut_ptr(), 16, 64) };
    unsafe {
        let ptr1 = slab.alloc();
        slab.free(ptr1);
        let ptr2 = slab.alloc();
        assert_eq!(ptr1, ptr2);
    }
}

#[test]
fn test_slab_exhaust() {
    let mut buffer = [0u8; 1024];
    let mut slab = unsafe { Slab::new(buffer.as_mut_ptr(), 16, 64) };
    unsafe {
        for _ in 0..64 {
            assert!(!slab.alloc().is_null());
        }
        assert!(slab.alloc().is_null());
    }
}
