use crate::field_meta::categories::TypeCategory;
use crate::field_meta::visitor::{FieldVisitor, VisitFields};

pub trait FieldInfo: Copy + 'static {
    const NAME: &'static str;
    const SERIALIZED_NAME: &'static str;
    const CATEGORY_NAME: &'static str;
    const TAGS: &'static [&'static str];

    type Value;
    type Category: TypeCategory;

    fn has_tag(tag: &str) -> bool {
        Self::TAGS.iter().any(|t| *t == tag)
    }
}

pub struct CollectNames(pub Vec<&'static str>);

impl CollectNames {
    pub fn new() -> Self {
        Self(Vec::new())
    }

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

pub struct CollectSerializedNames(pub Vec<&'static str>);

impl CollectSerializedNames {
    pub fn new() -> Self {
        Self(Vec::new())
    }

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

/// Фильтрует по категории
pub struct FilterByCategory<'a> {
    pub category: &'a str,
    pub names: Vec<&'static str>,
}

impl<'a> FilterByCategory<'a> {
    pub fn new(category: &'a str) -> Self {
        Self {
            category,
            names: Vec::new(),
        }
    }

    pub fn collect<T: VisitFields + ?Sized>(category: &'a str) -> Vec<&'static str> {
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

/// Фильтрует по тегу
pub struct FilterByTag<'a> {
    pub tag: &'a str,
    pub names: Vec<&'static str>,
}

impl<'a> FilterByTag<'a> {
    pub fn new(tag: &'a str) -> Self {
        Self {
            tag,
            names: Vec::new(),
        }
    }

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

/// Проверяет существование поля
pub struct HasField<'a> {
    pub name: &'a str,
    pub found: bool,
}

impl<'a> HasField<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, found: false }
    }

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

/// Получает категорию поля по имени
pub struct GetFieldCategory<'a> {
    pub name: &'a str,
    pub category: Option<&'static str>,
}

impl<'a> GetFieldCategory<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            category: None,
        }
    }

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
