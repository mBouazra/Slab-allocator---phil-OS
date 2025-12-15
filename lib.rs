//! Slab Allocator Implementation
//! 
//! Authors: TON_NOM, NOM_BINOME

#![no_std]

pub mod slab;
pub mod allocator;
pub mod free_list;  // ← Ton binôme codera ce module
pub mod utils;      // ← Ton binôme codera ce module

pub use slab::Slab;
pub use allocator::SlabAllocator;
