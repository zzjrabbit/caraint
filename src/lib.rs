#![no_std]
#![deny(unsafe_code)]

extern crate alloc;

#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub mod ast;
pub mod back;
pub mod front;
