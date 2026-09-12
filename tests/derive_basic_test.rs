#![allow(dead_code, unused_imports)]

use field_kinds::{Category, FieldKinds, FieldKindsExt, VisitFields};
use std::collections::HashMap;

#[derive(FieldKinds)]
struct SimpleStruct {
    id: u64,
    name: String,
    active: bool,
}

#[test]
fn field_count() {
    assert_eq!(SimpleStruct::FIELD_COUNT, 3);
}

#[test]
fn field_names() {
    let names = SimpleStruct::field_names();
    assert_eq!(names, vec!["id", "name", "active"]);
}

#[test]
fn serialized_names_default() {
    let names = SimpleStruct::serialized_names();
    assert_eq!(names, vec!["id", "name", "active"]);
}

#[test]
fn has_field() {
    assert!(SimpleStruct::has_field("id"));
    assert!(SimpleStruct::has_field("name"));
    assert!(SimpleStruct::has_field("active"));
    assert!(!SimpleStruct::has_field("nonexistent"));
}

#[test]
fn field_category() {
    assert_eq!(SimpleStruct::field_category("id"), Some(Category::NUMERIC));
    assert_eq!(SimpleStruct::field_category("name"), Some(Category::TEXT));
    assert_eq!(SimpleStruct::field_category("active"), Some(Category::BOOL));
    assert_eq!(SimpleStruct::field_category("nonexistent"), None);
}

#[test]
fn fields_by_category() {
    assert_eq!(SimpleStruct::fields_by_category(Category::NUMERIC), vec!["id"]);
    assert_eq!(SimpleStruct::fields_by_category(Category::TEXT), vec!["name"]);
    assert_eq!(
        SimpleStruct::fields_by_category(Category::BOOL),
        vec!["active"]
    );
    assert!(SimpleStruct::fields_by_category(Category::OPTIONAL).is_empty());
}

#[test]
fn field_meta() {
    let meta = SimpleStruct::field_meta();
    assert_eq!(meta.len(), 3);

    assert_eq!(meta[0].name, "id");
    assert_eq!(meta[0].category, Category::NUMERIC);

    assert_eq!(meta[1].name, "name");
    assert_eq!(meta[1].category, Category::TEXT);

    assert_eq!(meta[2].name, "active");
    assert_eq!(meta[2].category, Category::BOOL);
}

#[derive(FieldKinds)]
struct EmptyStruct {}

#[test]
fn empty_struct() {
    assert_eq!(EmptyStruct::FIELD_COUNT, 0);
    assert!(EmptyStruct::field_names().is_empty());
}

#[derive(FieldKinds)]
struct CollectionStruct {
    items: Vec<String>,
    count: Option<u32>,
    data: HashMap<String, i32>,
}

#[test]
fn collection_struct_categories() {
    assert_eq!(
        CollectionStruct::fields_by_category(Category::COLLECTION),
        vec!["items", "data"]
    );
    assert_eq!(
        CollectionStruct::fields_by_category(Category::OPTIONAL),
        vec!["count"]
    );
}

#[derive(FieldKinds)]
struct RawIdentStruct {
    r#type: String,
    r#match: bool,
    id: u64,
}

#[test]
fn raw_identifiers_should_be_reported_without_prefix() {
    assert_eq!(
        RawIdentStruct::field_names(),
        vec!["type", "match", "id"],
        "the `r#` prefix is Rust syntax, not part of the field name"
    );
    assert_eq!(RawIdentStruct::serialized_names(), vec!["type", "match", "id"]);
    assert!(RawIdentStruct::has_field("type"));
    assert_eq!(RawIdentStruct::field_category("type"), Some(Category::TEXT));
}

/// `NAMES` must not drift from the names inside `FIELDS`.
#[test]
fn names_const_agrees_with_fields() {
    let from_fields: Vec<_> =
        SimpleStruct::FIELDS.iter().map(|f| f.name).collect();
    assert_eq!(SimpleStruct::NAMES, from_fields);
}

#[test]
fn serialized_names_const_agrees_with_fields() {
    let from_fields: Vec<_> = SimpleStruct::FIELDS
        .iter()
        .filter(|f| !f.skip_serializing)
        .map(|f| f.serialized_name)
        .collect();
    assert_eq!(SimpleStruct::SERIALIZED_NAMES, from_fields);
}

/// The point of the consts: reading the names hands out the static slice
/// itself, rather than copying it into a fresh allocation.
#[test]
fn field_names_does_not_copy() {
    assert!(std::ptr::eq(SimpleStruct::field_names(), SimpleStruct::NAMES));
    assert!(std::ptr::eq(
        SimpleStruct::serialized_names(),
        SimpleStruct::SERIALIZED_NAMES
    ));
}

/// Being consts rather than methods, the names are usable at compile time.
#[test]
fn names_are_usable_in_const_context() {
    const COUNT: usize = SimpleStruct::NAMES.len();
    const FIRST: &str = SimpleStruct::NAMES[0];
    static IDS: [u8; COUNT] = [0; 3];

    assert_eq!(COUNT, SimpleStruct::FIELD_COUNT);
    assert_eq!(FIRST, "id");
    assert_eq!(IDS.len(), 3);
}
