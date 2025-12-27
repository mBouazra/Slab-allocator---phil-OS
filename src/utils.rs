//! # Fonctions Utilitaires
//! Fonctions d'aide pour l'alignement mémoire et les calculs.
/// Aligne une adresse vers le haut à l'alignement donné.

/// # Arguments
/// * addr - L'adresse à aligner
/// * align - L'alignement (doit être une puissance de 2)

/// # Retourne
/// La plus petite valeur >= addr qui est un multiple de align.



/// Aligns an address up to the given alignment.
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Vérifie si un pointeur est aligné à l'alignement donné.
/// # Arguments
/// * ptr - Le pointeur à vérifier
/// * align - L'alignement à vérifier

/// # Retourne
/// true si ptr est aligné à align, false sinon.
/// Checks if a pointer is aligned to the given alignment.
pub fn is_aligned(ptr: *const u8, align: usize) -> bool {
    (ptr as usize) % align == 0
}



/// Calcule combien d'objets tiennent dans une région mémoire.
/// # Arguments
/// total_size - Taille de la région mémoire
/// * object_size - Taille de chaque objet

/// # Retourne
/// Nombre d'objets qui tiennent (division entière).
/// Calculates the number of slabs that fit in a given memory region.
pub fn calculate_slab_count(total_size: usize, object_size: usize) -> usize {
    total_size / object_size
}


