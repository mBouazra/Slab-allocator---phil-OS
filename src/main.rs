//! # Point d'entrée du Kernel

//! C'est le point d'entrée principal du kernel.
//! Il initialise l'allocateur slab et entre dans une boucle infinie.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod free_list;
mod slab;
mod allocator;
mod utils;

use allocator::SlabAllocator;

/// Instance globale de l'allocateur slab.
static ALLOCATOR: SlabAllocator = SlabAllocator::new();

/// Région mémoire du tas (64 Ko).
static mut HEAP: [u8; 65536] = [0; 65536];


/// Cette fonction est appelée par le bootloader et  Elle initialise l'allocateur et boucle indéfiniment.
#[no_mangle]
pub extern "C" fn _start() -> ! {
 // Sécurité : HEAP est un tableau statique que nous possédons.
    // Ceci est appelé une fois au démarrage avant toute allocation.
    let heap_ptr = unsafe { HEAP.as_mut_ptr() };
    let heap_size = 65536;
  
 // Sécurité : heap_ptr pointe vers de la mémoire valide de heap_size octets.
  unsafe {
        ALLOCATOR.init(heap_ptr, heap_size);
    }
    loop {}
}

/// Gestionnaire de panic pour le kernel.
/// Appelé quand un panic se produit. Boucle indéfiniment.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
