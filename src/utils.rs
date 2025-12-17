/// Aligne une adresse sur la valeur donnée
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Vérifie si un pointeur est aligné
pub fn is_aligned(ptr: *const u8, align: usize) -> bool {
    (ptr as usize) % align == 0
}

/// Calcule le nombre de slabs nécessaires
pub fn calculate_slab_count(total_size: usize, object_size: usize) -> usize {
    total_size / object_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(10, 8), 16);
        assert_eq!(align_up(16, 8), 16);
    }
}
		
