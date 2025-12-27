# Slab Allocator - Phil-OS Integration

A minimal slab allocator implementation in Rust, integrated into the phil-os codebase.

## Authors

- BOUAZRA Mehdi
- MALIH Omar

## Description

This project implements a slab allocator inspired by the Linux kernel SLUB allocator.
The slab allocator provides efficient memory allocation for fixed-size objects.

## Build Instructions

### Run tests

cargo test --lib


### Build for kernel (x86_64)

rustup target add x86_64-unknown-none
cargo build --target x86_64-unknown-none


## Project Structure

<img width="528" height="179" alt="image" src="https://github.com/user-attachments/assets/337d54b7-ec61-4371-a6dd-7c54903299ad" />



## Features

- `no_std` compatible
- Three slab sizes: 32, 64, 128 bytes
- O(1) allocation and deallocation
- Thread-safe with spin locks

## Sources

- [Phil Opp's OS Tutorial](https://os.phil-opp.com/)
- [Linux SLUB Allocator - LWN](https://lwn.net/Articles/229984/)
- [The Slab Allocator Paper - Bonwick](https://people.eecs.berkeley.edu/~kubitron/courses/cs194-24-S13/hand-outs/bonwick_slab.pdf)
