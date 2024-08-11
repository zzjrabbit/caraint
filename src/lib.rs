#![no_std]
// #![deny(unsafe_code)]

extern crate alloc;

#[cfg(feature = "snmalloc")]
#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub mod ast;
pub mod backend;
pub mod frontend;
