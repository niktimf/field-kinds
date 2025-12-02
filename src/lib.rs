#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)] // Позволяем повторения имен модулей в типах
#![warn(missing_docs)]

//! Derive macro for compile-time struct field type introspection.

mod field_meta;

pub use field_kinds_derive::FieldKinds;
pub use field_meta::{
    Bool, Categorized, CollectMeta, CollectNames, CollectSerializedNames, Collection, FieldCount,
    FieldInfo, FieldKinds, FieldKindsExt, FieldMeta, FieldVisitor, FilterByCategory, FilterByTag,
    GetFieldCategory, HListVisitor, HasField, Numeric, Optional, Text, TypeCategory, Unknown,
    VisitFields,
};
