mod categories;
mod field_info;
mod hlist_ops;
mod visitor;

pub use categories::{
    Bool, Categorized, Collection, Numeric, Optional, Text, TypeCategory, Unknown,
};
pub use field_info::{
    CollectNames, CollectSerializedNames, FieldInfo, FilterByCategory, FilterByTag,
    GetFieldCategory, HasField,
};
pub use hlist_ops::{FieldCount, HListVisitor};
pub use visitor::{FieldVisitor, VisitFields};

pub trait FieldKinds: VisitFields {
    type Fields: FieldCount + HListVisitor;

    const FIELD_COUNT: usize = Self::Fields::COUNT;
}

#[derive(Debug, Clone)]
pub struct FieldMeta {
    pub name: &'static str,
    pub serialized_name: &'static str,
    pub category: &'static str,
    pub tags: &'static [&'static str],
    pub type_name: &'static str,
}

pub struct CollectMeta(pub Vec<FieldMeta>);

impl CollectMeta {
    pub fn new() -> Self {
        Self(Vec::new())
    }

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

pub trait FieldKindsExt: VisitFields {
    fn field_names() -> Vec<&'static str> {
        CollectNames::collect::<Self>()
    }

    fn serialized_names() -> Vec<&'static str> {
        CollectSerializedNames::collect::<Self>()
    }

    fn fields_by_category(category: &str) -> Vec<&'static str> {
        FilterByCategory::collect::<Self>(category)
    }

    fn fields_by_tag(tag: &str) -> Vec<&'static str> {
        FilterByTag::collect::<Self>(tag)
    }

    fn has_field(name: &str) -> bool {
        HasField::check::<Self>(name)
    }

    fn field_category(name: &str) -> Option<&'static str> {
        GetFieldCategory::get::<Self>(name)
    }

    fn field_meta() -> Vec<FieldMeta> {
        CollectMeta::collect::<Self>()
    }
}

impl<T: VisitFields> FieldKindsExt for T {}
