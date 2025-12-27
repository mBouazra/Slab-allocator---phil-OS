use crate::slab::Slab;
use crate::free_list::FreeList;
use crate::utils::{align_up, is_aligned, calculate_slab_count};

#[test]
fn test_free_list_new_is_empty() {
    let free_list = FreeList::new();
    assert!(free_list.is_empty());
}

#[test]
fn test_free_list_push_pop() {
    let mut buffer = [0u8; 64];
    let mut free_list = FreeList::new();
    unsafe {
        free_list.push(buffer.as_mut_ptr());
        assert!(!free_list.is_empty());
        assert_eq!(free_list.pop(), Some(buffer.as_mut_ptr()));
        assert!(free_list.is_empty());
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
fn test_free_list_multiple() {
    let mut buffer = [0u8; 128];
    let mut free_list = FreeList::new();
    unsafe {
        let ptr1 = buffer.as_mut_ptr();
        let ptr2 = buffer.as_mut_ptr().add(64);
        free_list.push(ptr1);
        free_list.push(ptr2);
        assert_eq!(free_list.pop(), Some(ptr2));
        assert_eq!(free_list.pop(), Some(ptr1));
        assert_eq!(free_list.pop(), None);
    }
}

#[test]
fn test_slab_empty() {
    let slab = Slab::empty(32);
    assert_eq!(slab.object_size(), 32);
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

#[test]
fn test_slab_capacity() {
    let mut buffer = [0u8; 256];
    let slab = unsafe { Slab::new(buffer.as_mut_ptr(), 32, 8) };
    assert_eq!(slab.capacity(), 8);
}

#[test]
fn test_slab_base() {
    let mut buffer = [0u8; 256];
    let base = buffer.as_mut_ptr();
    let slab = unsafe { Slab::new(base, 32, 8) };
    assert_eq!(slab.base(), base);
}

#[test]
fn test_align_up() {
    assert_eq!(align_up(0, 8), 0);
    assert_eq!(align_up(1, 8), 8);
    assert_eq!(align_up(8, 8), 8);
    assert_eq!(align_up(9, 8), 16);
}

#[test]
fn test_is_aligned() {
    let val: u64 = 0;
    let ptr = &val as *const u64 as *const u8;
    assert!(is_aligned(ptr, 8));
}

#[test]
fn test_calculate_slab_count() {
    assert_eq!(calculate_slab_count(1024, 16), 64);
    assert_eq!(calculate_slab_count(1024, 32), 32);
}
