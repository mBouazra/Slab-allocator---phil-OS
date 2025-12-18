//! Slab Allocator Implementation
 
//! Authors: BOUAZRA MEHDI && MALIH OMAR

#![no_std]
#![feature(allocator_api)]

extern crate alloc;

pub mod slab;
pub mod allocator;
pub mod free_list;
pub mod utils;
pub mod fat32_types;

use allocator::SlabAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: SlabAllocator = SlabAllocator::new();


#[cfg(test)]
mod tests;
