use std::collections::HashMap;
use field_kinds::{FieldKinds, FieldKindsExt, VisitFields};

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
    assert_eq!(SimpleStruct::field_category("id"), Some("numeric"));
    assert_eq!(SimpleStruct::field_category("name"), Some("text"));
    assert_eq!(SimpleStruct::field_category("active"), Some("bool"));
    assert_eq!(SimpleStruct::field_category("nonexistent"), None);
}

#[test]
fn fields_by_category() {
    assert_eq!(SimpleStruct::fields_by_category("numeric"), vec!["id"]);
    assert_eq!(SimpleStruct::fields_by_category("text"), vec!["name"]);
    assert_eq!(SimpleStruct::fields_by_category("bool"), vec!["active"]);
    assert!(SimpleStruct::fields_by_category("optional").is_empty());
}

#[test]
fn field_meta() {
    let meta = SimpleStruct::field_meta();
    assert_eq!(meta.len(), 3);

    assert_eq!(meta[0].name, "id");
    assert_eq!(meta[0].category, "numeric");

    assert_eq!(meta[1].name, "name");
    assert_eq!(meta[1].category, "text");

    assert_eq!(meta[2].name, "active");
    assert_eq!(meta[2].category, "bool");
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
    assert_eq!(CollectionStruct::fields_by_category("collection"), vec!["items", "data"]);
    assert_eq!(CollectionStruct::fields_by_category("optional"), vec!["count"]);
}