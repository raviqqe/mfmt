mod context;
mod ir;

pub use context::Context;
pub use ir::{build::*, count_lines, is_broken, Document};
