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
    fn field_names() -> Vec<&'static str> {
        Self::FIELDS.iter().map(|f| f.name).collect()
    }

    /// Returns an iterator over original field names.
    fn field_names_iter() -> impl Iterator<Item = &'static str> {
        Self::FIELDS.iter().map(|f| f.name)
    }

    /// Returns serialized field names (respecting `#[serde(rename)]`).
    fn serialized_names() -> Vec<&'static str> {
        Self::FIELDS.iter().map(|f| f.serialized_name).collect()
    }

    /// Returns an iterator over serialized field names.
    fn serialized_names_iter() -> impl Iterator<Item = &'static str> {
        Self::FIELDS.iter().map(|f| f.serialized_name)
    }

    /// Returns field names matching the given category.
    fn fields_by_category(category: Category) -> Vec<&'static str> {
        Self::FIELDS
            .iter()
            .filter(|f| f.category == category)
            .map(|f| f.name)
            .collect()
    }

    /// Returns field names that have the given tag.
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
    fn find_by_serialized_name(name: &str) -> Option<&'static FieldMeta> {
        Self::FIELDS.iter().find(|f| f.serialized_name == name)
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
