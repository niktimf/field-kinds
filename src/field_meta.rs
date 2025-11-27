//! Derive macro for compile-time struct field type introspection.
//!
//! # Example
//!
//! ```rust
//! use field_kinds::{FieldKinds, FieldMeta, is_rust_numeric};
//!
//! #[derive(FieldKinds)]
//! #[serde(rename_all = "SCREAMING_SNAKE_CASE")]  // supports serde attributes!
//! struct User {
//!     id: u64,                    // serialized_name: "ID"
//!     user_name: String,          // serialized_name: "USER_NAME"
//!     
//!     #[serde(rename = "SCORE")]
//!     score_value: f32,           // serialized_name: "SCORE"
//!     
//!     #[field_tags("numeric")]
//!     balance: MyDecimal,         // custom tag
//! }
//!
//! // By original name
//! let meta = User::field("id").unwrap();
//! assert_eq!(meta.serialized_name, "ID");
//!
//! // By serialized name
//! let meta = User::field_by_serialized("USER_NAME").unwrap();
//! assert_eq!(meta.name, "user_name");
//! ```


/// Metadata about a struct field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldMeta {
    /// Original Rust field name.
    pub name: &'static str,
    /// Serialized name (after rename/rename_all transformations).
    pub serialized_name: &'static str,
    /// Short type name (e.g., "u64", "String", "Decimal").
    pub type_name: &'static str,
    /// Full type path (e.g., "u64", "std::string::String", "rust_decimal::Decimal").
    pub type_path: &'static str,
    /// User-defined tags from `#[field_tags(...)]` attribute.
    pub tags: &'static [&'static str],
}

impl FieldMeta {
    /// Check if this field has a specific tag.
    #[inline]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag)
    }
}

/// Trait implemented by the derive macro.
pub trait FieldKindsInfo {
    /// All field metadata.
    const FIELDS: &'static [FieldMeta];

    /// Get field metadata by original Rust name.
    fn field(name: &str) -> Option<&'static FieldMeta> {
        Self::FIELDS.iter().find(|f| f.name == name)
    }

    /// Get field metadata by serialized name.
    fn field_by_serialized(name: &str) -> Option<&'static FieldMeta> {
        Self::FIELDS.iter().find(|f| f.serialized_name == name)
    }

    /// Get all fields.
    fn fields() -> &'static [FieldMeta] {
        Self::FIELDS
    }

    /// Iterate over fields that have a specific tag.
    fn fields_with_tag(tag: &str) -> impl Iterator<Item = &'static FieldMeta> {
        Self::FIELDS.iter().filter(move |f| f.has_tag(tag))
    }
}