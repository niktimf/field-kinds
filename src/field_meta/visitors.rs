/// Runtime-accessible metadata for a single field.
///
/// Contains all information about a field that can be queried at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldMeta {
    /// Original field name in Rust code.
    pub name: &'static str,
    /// Serialized name (may differ due to `#[serde(rename)]` or `rename_all`).
    pub serialized_name: &'static str,
    /// Type category (e.g., "numeric", "text", "bool", "optional", "collection").
    pub category: &'static str,
    /// Custom tags added via `#[field_tags(...)]`.
    pub tags: &'static [&'static str],
}

impl FieldMeta {
    /// Checks if this field has the given tag.
    pub const fn has_tag(&self, tag: &str) -> bool {
        let mut i = 0;
        while i < self.tags.len() {
            if const_str_eq(self.tags[i], tag) {
                return true;
            }
            i += 1;
        }
        false
    }
}

const fn const_str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Trait for types that provide static field metadata.
///
/// This trait is automatically implemented by the derive macro.
pub trait VisitFields {
    /// Static slice containing metadata for all fields.
    const FIELDS: &'static [FieldMeta];
}
