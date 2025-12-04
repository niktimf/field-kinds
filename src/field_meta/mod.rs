mod categories;
mod field_info;
mod hlist_ops;
mod visitors;

pub use categories::{
    Bool, Categorized, Collection, Numeric, Optional, Text, TypeCategory,
    Unknown,
};
pub use field_info::FieldInfo;
pub use hlist_ops::{FieldCount, HListVisitor};
pub use visitors::{
    CollectMeta, CollectNames, CollectSerializedNames, FieldMeta, FieldVisitor,
    FilterByCategory, FilterByTag, GetFieldCategory, HasField, VisitFields,
};

/// Trait implemented by structs deriving [`FieldKinds`](derive@crate::FieldKinds).
///
/// Provides compile-time field count and access to field type information
/// via an `HList` of field marker types.
pub trait FieldKinds: VisitFields {
    /// `HList` type containing marker types for each field.
    type Fields: FieldCount + HListVisitor;

    /// Number of fields in the struct (compile-time constant).
    const FIELD_COUNT: usize = Self::Fields::COUNT;
}

/// Extension trait providing convenient methods for field introspection.
///
/// Automatically implemented for all types that implement [`VisitFields`].
pub trait FieldKindsExt: VisitFields {
    /// Returns original field names.
    fn field_names() -> Vec<&'static str> {
        CollectNames::collect::<Self>()
    }

    /// Returns serialized field names (respecting `#[serde(rename)]`).
    fn serialized_names() -> Vec<&'static str> {
        CollectSerializedNames::collect::<Self>()
    }

    /// Returns field names matching the given category.
    fn fields_by_category(category: &str) -> Vec<&'static str> {
        FilterByCategory::collect::<Self>(category)
    }

    /// Returns field names that have the given tag.
    fn fields_by_tag(tag: &str) -> Vec<&'static str> {
        FilterByTag::collect::<Self>(tag)
    }

    /// Checks if a field with the given name exists.
    fn has_field(name: &str) -> bool {
        HasField::check::<Self>(name)
    }

    /// Returns the category of a field by name, or `None` if not found.
    fn field_category(name: &str) -> Option<&'static str> {
        GetFieldCategory::get::<Self>(name)
    }

    /// Returns full metadata for all fields.
    fn field_meta() -> Vec<FieldMeta> {
        CollectMeta::collect::<Self>()
    }
}

impl<T: VisitFields> FieldKindsExt for T {}
