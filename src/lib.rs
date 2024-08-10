#![no_std]
#![deny(unsafe_code)]

extern crate alloc;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod ast;
pub mod back;
pub mod front;
