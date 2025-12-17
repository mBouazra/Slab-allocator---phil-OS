use slab_allocator::Slab;

    #[test]
    fn test_slab_alloc() {
        
        let mut buffer = [0u8; 1024];
	        let mut slab = Slab::new(buffer.as_mut_ptr(), 16, 64);
        
        let ptr = slab.alloc();
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_slab_free() {
        let mut buffer = [0u8; 1024];
    let mut slab = Slab::new(buffer.as_mut_ptr(), 16, 64);

    let ptr1 = slab.alloc();
    slab.free(ptr1);

    let ptr2 = slab.alloc();
    assert_eq!(ptr1, ptr2);
    }

    #[test]
    fn test_multiple_allocs() {
        let mut buffer = [0u8; 1024];
    let mut slab = Slab::new(buffer.as_mut_ptr(), 16, 64);

    for _ in 0..64 {
        assert!(!slab.alloc().is_null());
    }

assert!(slab.alloc().is_null());

}
