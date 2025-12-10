#![allow(dead_code, unused_imports)]

use field_kinds::{
    FieldInfo, FieldMeta, Numeric, Text, TypeCategory, VisitFields,
};

struct TestStruct {
    field_a: i32,
    field_b: String,
}

#[derive(Clone, Copy)]
struct FieldA;
impl FieldInfo for FieldA {
    const NAME: &'static str = "field_a";
    const SERIALIZED_NAME: &'static str = "fieldA";
    const CATEGORY_NAME: &'static str = "numeric";
    const TAGS: &'static [&'static str] = &["primary", "indexed"];

    type Value = i32;
    type Category = Numeric;
}

#[derive(Clone, Copy)]
struct FieldB;
impl FieldInfo for FieldB {
    const NAME: &'static str = "field_b";
    const SERIALIZED_NAME: &'static str = "field_b";
    const CATEGORY_NAME: &'static str = "text";
    const TAGS: &'static [&'static str] = &["indexed"];

    type Value = String;
    type Category = Text;
}

impl VisitFields for TestStruct {
    const FIELDS: &'static [FieldMeta] = &[
        FieldMeta {
            name: "field_a",
            serialized_name: "fieldA",
            category: "numeric",
            tags: &["primary", "indexed"],
        },
        FieldMeta {
            name: "field_b",
            serialized_name: "field_b",
            category: "text",
            tags: &["indexed"],
        },
    ];
}

#[test]
fn field_names() {
    let names: Vec<_> = TestStruct::FIELDS.iter().map(|f| f.name).collect();
    assert_eq!(names, vec!["field_a", "field_b"]);
}

#[test]
fn serialized_names() {
    let names: Vec<_> = TestStruct::FIELDS
        .iter()
        .map(|f| f.serialized_name)
        .collect();
    assert_eq!(names, vec!["fieldA", "field_b"]);
}

#[test]
fn filter_by_category() {
    let numeric: Vec<_> = TestStruct::FIELDS
        .iter()
        .filter(|f| f.category == "numeric")
        .map(|f| f.name)
        .collect();
    assert_eq!(numeric, vec!["field_a"]);

    let text: Vec<_> = TestStruct::FIELDS
        .iter()
        .filter(|f| f.category == "text")
        .map(|f| f.name)
        .collect();
    assert_eq!(text, vec!["field_b"]);

    let bool_fields = TestStruct::FIELDS
        .iter()
        .filter(|f| f.category == "bool")
        .map(|f| f.name)
        .next()
        .is_none();
    assert!(bool_fields);
}

#[test]
fn filter_by_tag() {
    let indexed: Vec<_> = TestStruct::FIELDS
        .iter()
        .filter(|f| f.tags.contains(&"indexed"))
        .map(|f| f.name)
        .collect();
    assert_eq!(indexed, vec!["field_a", "field_b"]);

    let primary: Vec<_> = TestStruct::FIELDS
        .iter()
        .filter(|f| f.tags.contains(&"primary"))
        .map(|f| f.name)
        .collect();
    assert_eq!(primary, vec!["field_a"]);

    let nonexistent = TestStruct::FIELDS
        .iter()
        .filter(|f| f.tags.contains(&"nonexistent"))
        .map(|f| f.name)
        .next()
        .is_none();
    assert!(nonexistent);
}

#[test]
fn has_field() {
    assert!(TestStruct::FIELDS.iter().any(|f| f.name == "field_a"));
    assert!(TestStruct::FIELDS.iter().any(|f| f.name == "field_b"));
    assert!(!TestStruct::FIELDS.iter().any(|f| f.name == "field_c"));
}

#[test]
fn get_field_category() {
    assert_eq!(
        TestStruct::FIELDS
            .iter()
            .find(|f| f.name == "field_a")
            .map(|f| f.category),
        Some("numeric")
    );
    assert_eq!(
        TestStruct::FIELDS
            .iter()
            .find(|f| f.name == "field_b")
            .map(|f| f.category),
        Some("text")
    );
    assert_eq!(
        TestStruct::FIELDS
            .iter()
            .find(|f| f.name == "nonexistent")
            .map(|f| f.category),
        None
    );
}

#[test]
fn field_meta() {
    let meta = TestStruct::FIELDS;
    assert_eq!(meta.len(), 2);

    assert_eq!(meta[0].name, "field_a");
    assert_eq!(meta[0].serialized_name, "fieldA");
    assert_eq!(meta[0].category, "numeric");
    assert_eq!(meta[0].tags, &["primary", "indexed"]);

    assert_eq!(meta[1].name, "field_b");
    assert_eq!(meta[1].serialized_name, "field_b");
    assert_eq!(meta[1].category, "text");
}

#[test]
fn field_info_has_tag() {
    assert!(FieldA::has_tag("primary"));
    assert!(FieldA::has_tag("indexed"));
    assert!(!FieldA::has_tag("nonexistent"));

    assert!(FieldB::has_tag("indexed"));
    assert!(!FieldB::has_tag("primary"));
}

#[test]
fn field_meta_has_tag() {
    assert!(TestStruct::FIELDS[0].has_tag("primary"));
    assert!(TestStruct::FIELDS[0].has_tag("indexed"));
    assert!(!TestStruct::FIELDS[0].has_tag("nonexistent"));
}
