#![allow(dead_code, unused_imports)]

use field_kinds::{
    CollectMeta, CollectNames, CollectSerializedNames, FieldInfo, FieldVisitor,
    FilterByCategory, FilterByTag, GetFieldCategory, HasField, Numeric, Text,
    TypeCategory, VisitFields,
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

// Manual impl VisitFields
impl VisitFields for TestStruct {
    fn visit_fields<V: FieldVisitor>(visitor: &mut V) {
        visitor.visit::<FieldA>();
        visitor.visit::<FieldB>();
    }
}

#[test]
fn collect_names() {
    let names = CollectNames::collect::<TestStruct>();
    assert_eq!(names, vec!["field_a", "field_b"]);
}

#[test]
fn collect_serialized_names() {
    let names = CollectSerializedNames::collect::<TestStruct>();
    assert_eq!(names, vec!["fieldA", "field_b"]);
}

#[test]
fn filter_by_category() {
    assert_eq!(
        FilterByCategory::collect::<TestStruct>("numeric"),
        vec!["field_a"]
    );
    assert_eq!(
        FilterByCategory::collect::<TestStruct>("text"),
        vec!["field_b"]
    );
    assert!(FilterByCategory::collect::<TestStruct>("bool").is_empty());
}

#[test]
fn filter_by_tag() {
    assert_eq!(
        FilterByTag::collect::<TestStruct>("indexed"),
        vec!["field_a", "field_b"]
    );
    assert_eq!(FilterByTag::collect::<TestStruct>("primary"), vec!["field_a"]);
    assert!(FilterByTag::collect::<TestStruct>("nonexistent").is_empty());
}

#[test]
fn has_field_visitor() {
    assert!(HasField::check::<TestStruct>("field_a"));
    assert!(HasField::check::<TestStruct>("field_b"));
    assert!(!HasField::check::<TestStruct>("field_c"));
}

#[test]
fn get_field_category() {
    assert_eq!(GetFieldCategory::get::<TestStruct>("field_a"), Some("numeric"));
    assert_eq!(GetFieldCategory::get::<TestStruct>("field_b"), Some("text"));
    assert_eq!(GetFieldCategory::get::<TestStruct>("nonexistent"), None);
}

#[test]
fn collect_meta() {
    let meta = CollectMeta::collect::<TestStruct>();
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
