#![warn(missing_docs)]

//! Derive macro for compile-time struct field type introspection.

mod field_meta;

pub use field_kinds_derive::FieldKinds;
pub use field_meta::{
    Categorized, FieldCount, FieldInfo, FieldKinds, FieldKindsExt, FieldMeta, FieldVisitor,
    HListVisitor, TypeCategory, VisitFields, Numeric, Text, Bool, Optional, Collection, Unknown,
    CollectMeta, CollectNames, CollectSerializedNames, FilterByCategory, FilterByTag, 
    GetFieldCategory, HasField,
};
