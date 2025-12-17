//! Slab Allocator Implementation
 
//! Authors: BOUAZRA MEHDI && MALIH OMAR

#![no_std]

#[cfg(test)]
extern crate std;

pub mod slab;
pub mod allocator;
pub mod free_list;
pub mod utils;

pub use slab::Slab;
pub use allocator::SlabAllocator;
