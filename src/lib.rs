#![warn(missing_docs)]

//! Compile-time struct field introspection via derive macro.
//!
//! This crate provides the [`FieldKinds`] derive macro that generates
//! compile-time metadata about struct fields, including their names,
//! types, categories, and custom tags.
//!
//! # Features
//!
//! - **Field names**: Get field names as `&'static str`
//! - **Serialized names**: Supports `#[serde(rename)]` and `#[serde(rename_all)]`
//! - **Type categories**: Automatic categorization (numeric, text, bool, optional, collection)
//! - **Custom tags**: Add arbitrary tags to fields via `#[field_tags("tag1", "tag2")]`
//! - **Visitor pattern**: Extensible via [`FieldVisitor`] trait
//! - **Zero runtime cost**: All metadata is computed at compile time
//!
//! # Example
//!
//! ```rust
//! use field_kinds::{FieldKinds, FieldKindsExt};
//!
//! #[derive(FieldKinds)]
//! #[serde(rename_all = "camelCase")]
//! struct User {
//!     user_id: u64,
//!     user_name: String,
//!     is_active: bool,
//!     #[field_tags("sensitive", "pii")]
//!     email: Option<String>,
//! }
//!
//! // Get field names
//! assert_eq!(User::field_names(), vec!["user_id", "user_name", "is_active", "email"]);
//!
//! // Get serialized names (with rename_all applied)
//! assert_eq!(User::serialized_names(), vec!["userId", "userName", "isActive", "email"]);
//!
//! // Filter by category
//! assert_eq!(User::fields_by_category("numeric"), vec!["user_id"]);
//! assert_eq!(User::fields_by_category("text"), vec!["user_name"]);
//!
//! // Filter by tag
//! assert_eq!(User::fields_by_tag("sensitive"), vec!["email"]);
//!
//! // Check field existence
//! assert!(User::has_field("user_id"));
//! assert!(!User::has_field("nonexistent"));
//! ```
//!
//! # Attributes
//!
//! ## Struct-level
//!
//! - `#[serde(rename_all = "...")]` - Apply case conversion to serialized names
//!
//! ## Field-level
//!
//! - `#[serde(rename = "...")]` - Override serialized name for a field
//! - `#[field_tags("tag1", "tag2")]` - Add custom tags to a field
//! - `#[field_kinds(skip)]` - Skip a field from introspection

mod field_meta;

pub use field_kinds_derive::FieldKinds;
pub use field_meta::{
    Bool, Categorized, CollectMeta, CollectNames, CollectSerializedNames,
    Collection, FieldCount, FieldInfo, FieldKinds, FieldKindsExt, FieldMeta,
    FieldVisitor, FilterByCategory, FilterByTag, GetFieldCategory,
    HListVisitor, HasField, Numeric, Optional, Text, TypeCategory, Unknown,
    VisitFields,
};
