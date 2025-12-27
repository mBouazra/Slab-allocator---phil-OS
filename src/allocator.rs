//! # Allocateur Slab
//! Un allocateur mémoire global utilisant plusieurs slabs pour différentes tailles d'objets.
//! ## Sources
//! - [Tutoriel OS de Phil Opp](https://os.phil-opp.com/)
//! - [Allocateur SLUB Linux](https://lwn.net/Articles/229984/)



use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;

use crate::slab::Slab;

/// Un allocateur mémoire basé sur les slabs.
///
/// # Vue d'ensemble
///
/// L'allocateur maintient trois slabs pour différentes classes de taille :
/// - 32 octets : pour les petits objets
/// - 64 octets : pour les objets moyens
/// - 128 octets : pour les plus grands objets

/// # Thread Safety
/// Chaque slab est protégé par un spin lock (`Mutex`).
pub struct SlabAllocator {
    slab_32: Mutex<Slab>,
    slab_64: Mutex<Slab>,
    slab_128: Mutex<Slab>,
}

impl SlabAllocator {
/// Crée un nouvel allocateur non initialisé.

    /// Il faut appeler `init()` avant toute allocation.

    pub const fn new() -> Self {
        SlabAllocator {
            slab_32: Mutex::new(Slab::empty(32)),
            slab_64: Mutex::new(Slab::empty(64)),
            slab_128: Mutex::new(Slab::empty(128)),
        }
    }

/// Initialise l'allocateur avec une région mémoire de tas.
    /// Divise le tas équitablement entre les trois slabs
    /// # Argument
    /// * `heap_start` - Pointeur vers la mémoire du tas
    /// * `heap_size` - Taille du tas en octet
    /// # Sécurité Safety 
    /// L'appelant doit s'assurer que :
    /// - `heap_start` est un pointeur valide vers `heap_size` octets de mémoire
    /// - `heap_start` est aligné à au moins 8 octets
    /// - La mémoire reste valide pendant la durée de vie de l'allocateur
    
    pub unsafe fn init(&self, heap_start: *mut u8, heap_size: usize) {
        let region = heap_size / 3;

        let count_32 = region / 32;
        let count_64 = region / 64;
        let count_128 = region / 128;

        let mut offset = 0;
        *self.slab_32.lock() = Slab::new(heap_start.add(offset), 32, count_32);
        offset += count_32 * 32;

        *self.slab_64.lock() = Slab::new(heap_start.add(offset), 64, count_64);
        offset += count_64 * 64;

        *self.slab_128.lock() = Slab::new(heap_start.add(offset), 128, count_128);
    }
}

/// Implémentation de GlobalAlloc pour utilisation comme `#[global_allocator]`.
/// # Sécurité (Safety)

/// Sûr à utiliser si :
/// - `init()` a été appelé avec de la mémoire valide avant toute allocation
/// - La mémoire du tas reste valide pendant la durée de vie du programme

unsafe impl GlobalAlloc for SlabAllocator {

/// Alloue de la mémoire selon le layout donné.
    ///
    /// # Sécurité (Safety)
    ///
    /// - Retourne null pour les tailles > 128 octets
    /// - Retourne null si le slab approprié est épuisé
    /// - Le pointeur retourné est valide jusqu'à l'appel de `dealloc()`

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(layout.align());
        match size {
            0..=32 => self.slab_32.lock().alloc(),
            33..=64 => self.slab_64.lock().alloc(),
            65..=128 => self.slab_128.lock().alloc(),
            _ => null_mut(),
        }
    }

/// Désalloue de la mémoire précédemment allouée.
    ///
    /// # Sécurité (Safety)
    ///
    /// L'appelant doit s'assurer que :
    /// - `ptr` a été retourné par `alloc()` avec le même `layout`
    /// - `ptr` n'a pas été désalloué avant
    /// - `ptr` ne sera pas utilisé après cet appel

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(layout.align());
        match size {
            0..=32 => self.slab_32.lock().free(ptr),
            33..=64 => self.slab_64.lock().free(ptr),
            65..=128 => self.slab_128.lock().free(ptr),
            _ => {}
        }
    }
}

