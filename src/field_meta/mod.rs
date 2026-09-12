mod categories;
mod field_info;
mod visitors;

pub use categories::{
    Bool, Categorized, Category, Collection, Numeric, Optional, Text,
    TypeCategory, Unknown,
};
pub use field_info::FieldInfo;
pub use visitors::{FieldMeta, VisitFields};

/// Extension trait providing convenient methods for field introspection.
///
/// Automatically implemented for all types that implement [`VisitFields`].
pub trait FieldKindsExt: VisitFields {
    /// Returns original field names.
    fn field_names() -> &'static [&'static str] {
        Self::NAMES
    }

    /// Returns an iterator over original field names.
    fn field_names_iter() -> impl Iterator<Item = &'static str> {
        Self::NAMES.iter().copied()
    }

    /// Returns serialized field names (respecting `#[serde(rename)]`).
    ///
    /// Fields serde does not serialize have no serialized name and are
    /// left out.
    fn serialized_names() -> &'static [&'static str] {
        Self::SERIALIZED_NAMES
    }

    /// Returns an iterator over serialized field names.
    ///
    /// Fields serde does not serialize are left out.
    fn serialized_names_iter() -> impl Iterator<Item = &'static str> {
        Self::SERIALIZED_NAMES.iter().copied()
    }

    /// Returns field names matching the given category.
    ///
    /// Allocates: how many fields match depends on `category`, a runtime
    /// value, so the result has no compile-time length. Use
    /// [`filter_by_category`](Self::filter_by_category) to avoid it.
    fn fields_by_category(category: Category) -> Vec<&'static str> {
        Self::FIELDS
            .iter()
            .filter(|f| f.category == category)
            .map(|f| f.name)
            .collect()
    }

    /// Returns field names that have the given tag.
    ///
    /// Allocates, for the same reason as
    /// [`fields_by_category`](Self::fields_by_category). Use
    /// [`filter_by_tag`](Self::filter_by_tag) to avoid it.
    fn fields_by_tag(tag: &str) -> Vec<&'static str> {
        Self::FIELDS
            .iter()
            .filter(|f| f.tags.contains(&tag))
            .map(|f| f.name)
            .collect()
    }

    /// Returns an iterator over fields matching the given category.
    fn filter_by_category(
        category: Category,
    ) -> impl Iterator<Item = &'static FieldMeta> {
        Self::FIELDS.iter().filter(move |f| f.category == category)
    }

    /// Returns an iterator over fields that have the given tag.
    fn filter_by_tag(
        tag: &'static str,
    ) -> impl Iterator<Item = &'static FieldMeta> {
        Self::FIELDS.iter().filter(move |f| f.tags.contains(&tag))
    }

    /// Checks if a field with the given name exists.
    fn has_field(name: &str) -> bool {
        Self::FIELDS.iter().any(|f| f.name == name)
    }

    /// Finds a field by its original name.
    fn find_by_name(name: &str) -> Option<&'static FieldMeta> {
        Self::FIELDS.iter().find(|f| f.name == name)
    }

    /// Finds a field by its serialized name.
    ///
    /// Fields serde does not serialize are left out.
    fn find_by_serialized_name(name: &str) -> Option<&'static FieldMeta> {
        Self::FIELDS
            .iter()
            .find(|f| !f.skip_serializing && f.serialized_name == name)
    }

    /// Returns the category of a field by name, or `None` if not found.
    fn field_category(name: &str) -> Option<Category> {
        Self::FIELDS
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.category)
    }

    /// Returns full metadata for all fields.
    fn field_meta() -> &'static [FieldMeta] {
        Self::FIELDS
    }
}

impl<T: VisitFields> FieldKindsExt for T {}
