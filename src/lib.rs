//! Slab Allocator Implementation
 
//! ## Vue d'ensemble
//!
//! Ce crate fournit un allocateur mémoire utilisant la technique d'allocation slab
//! pour une allocation efficace d'objets de taille fixe.
//!
//! ## Fonctionnalités
//!
//! - Compatible `no_std` pour utilisation kernel/embarqué
//! - Trois tailles de slab : 32, 64, 128 octets
//! - Allocation et désallocation en O(1)
//! - Thread-safe avec spin locks

//! Authors: BOUAZRA MEHDI && MALIH OMAR

//! ## Sources
//!
//! - [Tutoriel OS de Phil Opp](https://os.phil-opp.com/)
//! - [Allocateur SLUB Linux](https://lwn.net/Articles/229984/)

#![no_std]

pub mod free_list;
pub mod slab;
pub mod allocator;
pub mod utils;

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests;