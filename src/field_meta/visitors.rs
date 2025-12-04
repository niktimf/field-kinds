use crate::field_meta::field_info::FieldInfo;

/// Trait for visiting fields of a struct.
///
/// Implement this trait to create custom field visitors that can
/// collect or process field information in different ways.
///
/// # Example
///
/// ```rust
/// use field_kinds::{FieldVisitor, FieldInfo, VisitFields, FieldKinds};
///
/// struct PrintVisitor;
///
/// impl FieldVisitor for PrintVisitor {
///     fn visit<F: FieldInfo>(&mut self) {
///         println!("Field: {} ({})", F::NAME, F::CATEGORY_NAME);
///     }
/// }
///
/// #[derive(FieldKinds)]
/// struct MyStruct {
///     id: u32,
///     name: String,
/// }
///
/// let mut visitor = PrintVisitor;
/// MyStruct::visit_fields(&mut visitor);
/// ```
pub trait FieldVisitor {
    /// Called for each field in the struct.
    fn visit<F: FieldInfo>(&mut self);
}

/// Trait for types that can be visited by a [`FieldVisitor`].
///
/// This trait is automatically implemented by the derive macro.
pub trait VisitFields {
    /// Visits all fields with the given visitor.
    fn visit_fields<V: FieldVisitor>(visitor: &mut V);
}

/// Runtime-accessible metadata for a single field.
///
/// Contains all information about a field that can be queried at runtime.
#[derive(Debug, Clone)]
pub struct FieldMeta {
    /// Original field name in Rust code.
    pub name: &'static str,
    /// Serialized name (may differ due to `#[serde(rename)]` or `rename_all`).
    pub serialized_name: &'static str,
    /// Type category (e.g., "numeric", "text", "bool", "optional", "collection").
    pub category: &'static str,
    /// Custom tags added via `#[field_tags(...)]`.
    pub tags: &'static [&'static str],
    /// Full type name from [`std::any::type_name`].
    pub type_name: &'static str,
}

/// Visitor that collects [`FieldMeta`] for all fields.
pub struct CollectMeta(pub Vec<FieldMeta>);

impl CollectMeta {
    /// Creates a new empty collector.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Collects metadata for all fields of type `T`.
    pub fn collect<T: VisitFields + ?Sized>() -> Vec<FieldMeta> {
        let mut v = Self::new();
        T::visit_fields(&mut v);
        v.0
    }
}

impl Default for CollectMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldVisitor for CollectMeta {
    fn visit<F: FieldInfo>(&mut self) {
        self.0.push(FieldMeta {
            name: F::NAME,
            serialized_name: F::SERIALIZED_NAME,
            category: F::CATEGORY_NAME,
            tags: F::TAGS,
            type_name: std::any::type_name::<F::Value>(),
        });
    }
}

/// Visitor that collects field names.
pub struct CollectNames(pub Vec<&'static str>);

impl CollectNames {
    /// Creates a new empty collector.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Collects field names for type `T`.
    pub fn collect<T: VisitFields + ?Sized>() -> Vec<&'static str> {
        let mut v = Self::new();
        T::visit_fields(&mut v);
        v.0
    }
}

impl Default for CollectNames {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldVisitor for CollectNames {
    fn visit<F: FieldInfo>(&mut self) {
        self.0.push(F::NAME);
    }
}

/// Visitor that collects serialized field names.
pub struct CollectSerializedNames(pub Vec<&'static str>);

impl CollectSerializedNames {
    /// Creates a new empty collector.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Collects serialized field names for type `T`.
    pub fn collect<T: VisitFields + ?Sized>() -> Vec<&'static str> {
        let mut v = Self::new();
        T::visit_fields(&mut v);
        v.0
    }
}

impl Default for CollectSerializedNames {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldVisitor for CollectSerializedNames {
    fn visit<F: FieldInfo>(&mut self) {
        self.0.push(F::SERIALIZED_NAME);
    }
}

/// Visitor that filters fields by category.
pub struct FilterByCategory<'a> {
    /// Category to filter by.
    pub category: &'a str,
    /// Collected field names.
    pub names: Vec<&'static str>,
}

impl<'a> FilterByCategory<'a> {
    /// Creates a new filter for the given category.
    pub const fn new(category: &'a str) -> Self {
        Self {
            category,
            names: Vec::new(),
        }
    }

    /// Collects field names matching the category for type `T`.
    pub fn collect<T: VisitFields + ?Sized>(
        category: &'a str,
    ) -> Vec<&'static str> {
        let mut v = Self::new(category);
        T::visit_fields(&mut v);
        v.names
    }
}

impl FieldVisitor for FilterByCategory<'_> {
    fn visit<F: FieldInfo>(&mut self) {
        if F::CATEGORY_NAME == self.category {
            self.names.push(F::NAME);
        }
    }
}

/// Visitor that filters fields by tag.
pub struct FilterByTag<'a> {
    /// Tag to filter by.
    pub tag: &'a str,
    /// Collected field names.
    pub names: Vec<&'static str>,
}

impl<'a> FilterByTag<'a> {
    /// Creates a new filter for the given tag.
    pub const fn new(tag: &'a str) -> Self {
        Self {
            tag,
            names: Vec::new(),
        }
    }

    /// Collects field names having the tag for type `T`.
    pub fn collect<T: VisitFields + ?Sized>(tag: &'a str) -> Vec<&'static str> {
        let mut v = Self::new(tag);
        T::visit_fields(&mut v);
        v.names
    }
}

impl FieldVisitor for FilterByTag<'_> {
    fn visit<F: FieldInfo>(&mut self) {
        if F::has_tag(self.tag) {
            self.names.push(F::NAME);
        }
    }
}

/// Visitor that checks if a field exists.
pub struct HasField<'a> {
    /// Field name to search for.
    pub name: &'a str,
    /// Whether the field was found.
    pub found: bool,
}

impl<'a> HasField<'a> {
    /// Creates a new checker for the given field name.
    pub const fn new(name: &'a str) -> Self {
        Self { name, found: false }
    }

    /// Checks if type `T` has a field with the given name.
    pub fn check<T: VisitFields + ?Sized>(name: &'a str) -> bool {
        let mut v = Self::new(name);
        T::visit_fields(&mut v);
        v.found
    }
}

impl FieldVisitor for HasField<'_> {
    fn visit<F: FieldInfo>(&mut self) {
        if F::NAME == self.name {
            self.found = true;
        }
    }
}

/// Visitor that retrieves a field's category by name.
pub struct GetFieldCategory<'a> {
    /// Field name to look up.
    pub name: &'a str,
    /// The category if found.
    pub category: Option<&'static str>,
}

impl<'a> GetFieldCategory<'a> {
    /// Creates a new lookup for the given field name.
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            category: None,
        }
    }

    /// Gets the category for a field by name in type `T`.
    pub fn get<T: VisitFields + ?Sized>(name: &'a str) -> Option<&'static str> {
        let mut v = Self::new(name);
        T::visit_fields(&mut v);
        v.category
    }
}

impl FieldVisitor for GetFieldCategory<'_> {
    fn visit<F: FieldInfo>(&mut self) {
        if F::NAME == self.name {
            self.category = Some(F::CATEGORY_NAME);
        }
    }
}
