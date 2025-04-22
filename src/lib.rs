#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "allocator-api", feature(allocator_api))]
#![no_std]

extern crate alloc;

mod build;
mod document;
mod format;
mod options;
pub mod utility;

pub use build::*;
pub use document::*;
pub use format::*;
pub use options::*;
