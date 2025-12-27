//! # Liste Libre
//!
//! Une liste chaînée de blocs mémoire libres pour l'allocateur slab.
//!
//! La liste libre est intrusive : chaque bloc libre stocke un pointeur vers le
//! prochain bloc libre à son début, évitant les métadonnées externes.
//! 
use core::ptr::null_mut;

/// Une liste chaînée de blocs mémoire libres.
///
/// # Implémentation
///
/// Chaque bloc libre contient un pointeur vers le prochain bloc libre à son début.
/// C'est une liste chaînée intrusive qui ne nécessite pas d'allocation externe.

pub struct FreeList {
    head: *mut u8,
}

impl FreeList {
    /// Crée une nouvelle liste libre vide.
    pub const fn new() -> Self {
        FreeList { head: null_mut() }
    }
        /// Ajoute un bloc mémoire au début de la liste libre.
    ///
    /// # Sécurité (Safety)
    ///
    /// L'appelant doit s'assurer que :
    /// - `ptr` est un pointeur valide et non-null
    /// - `ptr` est aligné à au moins `align_of::<*mut u8>()` (8 octets sur 64-bit)
    /// - `ptr` pointe vers une région mémoire d'au moins `size_of::<*mut u8>()` octets
    /// - `ptr` n'est pas déjà dans la liste libre (pas de double-free)
    /// - La mémoire à `ptr` n'est pas utilisée ailleurs


    pub unsafe fn push(&mut self, ptr: *mut u8) {
        let node = ptr as *mut *mut u8;
        *node = self.head;
        self.head = ptr;
    }

    /// Retire et retourne un bloc mémoire du début de la liste libre.
    ///
    /// Retourne `None` si la liste libre est vide.
    ///
    /// # Sécurité (Safety)
    ///
    /// L'appelant doit s'assurer que :
    /// - Le pointeur retourné est utilisé avant d'être remis dans la liste
    /// - L'appelant prend possession du bloc mémoire
    /// - La mémoire n'est pas accédée après avoir été libérée

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

// Sécurité : FreeList peut être envoyée entre threads.
// Les pointeurs bruts n'ont pas d'affinité de thread.
// La synchronisation est gérée par Mutex dans SlabAllocator.

unsafe impl Send for FreeList {}