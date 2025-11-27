#![warn(missing_docs)]

//! Derive macro for compile-time struct field type introspection.

mod field_meta;
mod utils;

pub use field_kinds_derive::FieldKinds;
pub use field_meta::{FieldMeta, FieldKindsInfo};
pub use utils::*;

