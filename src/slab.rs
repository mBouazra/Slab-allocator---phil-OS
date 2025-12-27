//! # Slab
//!
//! Un slab est une région mémoire contiguë divisée en objets de taille fixe.
//!
//! Cette conception est inspirée de l'allocateur SLUB du noyau Linux.
use core::ptr::null_mut;
use crate::free_list::FreeList;

/// Un slab d'objets mémoire de taille fixe.
/// # Vue d'ensemble
/// Le slab pré-alloue une région mémoire contiguë et la divise en blocs
/// de taille égale. Cela réduit la fragmentation et fournit une allocation en O(1).

pub struct Slab {
    /// Taille de chaque objet en octets
    object_size: usize,
    /// Liste des objets libres
    free_list: FreeList,
    /// Adresse de base du slab
    base: *mut u8,
    /// Nombre d'objets dans le slab
    capacity: usize,
}

impl Slab {
    /// Crée un slab vide sans mémoire de support.
    /// Utilisé pour l'initialisation à la compilation. Il faut appeler `new()` avant utilisation.
    pub const fn empty(object_size: usize) -> Self {
        Slab {
            object_size,
            free_list: FreeList::new(),
            base: null_mut(),
            capacity: 0,
        }
    }
        /// Crée un nouveau slab avec la région mémoire donnée.
    ///
    /// # Arguments
    ///
    /// * `base` - Pointeur vers la région mémoire
    /// * `object_size` - Taille de chaque objet en octets
    /// * `count` - Nombre d'objets
    ///
    /// # Sécurité (Safety)
    ///
    /// L'appelant doit s'assurer que :
    /// - `base` est un pointeur valide vers de la mémoire allouée
    /// - La région mémoire fait au moins `object_size * count` octets
    /// - `base` est aligné à au moins 8 octets
    /// - `object_size` est au moins `size_of::<*mut u8>()` (8 octets)
    /// - La mémoire reste valide pendant la durée de vie du Slab
    /// - Aucun autre code n'accède à cette mémoire tant que le Slab la possède


    pub unsafe fn new(base: *mut u8, object_size: usize, count: usize) -> Self {
        let mut slab = Slab {
            object_size,
            free_list: FreeList::new(),
            base,
            capacity: count,
        };
        // Initialiser la liste libre avec tous les objets
        for i in 0..count {
            let ptr = base.add(i * object_size);
            slab.free_list.push(ptr);
        }
        slab
    }
        /// Alloue un objet du slab.
    ///
    /// Retourne un pointeur vers l'objet, ou null si le slab est épuisé.
    ///
    /// # Sécurité (Safety)
    /// L'appelant doit s'assurer que :
    /// - Le pointeur retourné n'est pas utilisé après avoir appelé `free()` dessus
    /// - La mémoire n'est pas écrite au-delà de `object_size` octets


    pub unsafe fn alloc(&mut self) -> *mut u8 {
        self.free_list.pop().unwrap_or(null_mut())
    }
        /// Retourne un objet au slab.
    ///
    /// # Sécurité (Safety)
    ///
    /// L'appelant doit s'assurer que :
    /// - `ptr` a été retourné par `alloc()` sur ce même slab
    /// - `ptr` n'a pas déjà été libéré (pas de double-free)
    /// - `ptr` ne sera pas utilisé après cet appel


    pub unsafe fn free(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            self.free_list.push(ptr);
        }
    }
    /// Retourne la taille des objets de ce slab.

    pub fn object_size(&self) -> usize {
        self.object_size
    }
    /// Retourne l'adresse de base de ce slab.

    pub fn base(&self) -> *mut u8 {
        self.base
    }
    /// Retourne la capacité (nombre d'objets) de ce slab.

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
// Sécurité : Slab peut être envoyé entre threads.
// La synchronisation est gérée par Mutex dans SlabAllocator.
unsafe impl Send for Slab {}