/// Aligns an address up to the given alignment.
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Checks if a pointer is aligned to the given alignment.
pub fn is_aligned(ptr: *const u8, align: usize) -> bool {
    (ptr as usize) % align == 0
}

/// Calculates the number of slabs that fit in a given memory region.
pub fn calculate_slab_count(total_size: usize, object_size: usize) -> usize {
    total_size / object_size
}


