#![doc = include_str!("../README.md")]
#![feature(allocator_api)]
#![no_std]

extern crate alloc;

mod build;
mod document;
mod format;
pub mod utility;

pub use build::*;
pub use document::*;
pub use format::*;
