#![allow(dead_code, unused_imports)]

use field_kinds::{
    FieldInfo, FieldKindsExt, FieldMeta, Numeric, Text, TypeCategory,
    VisitFields,
};
use rstest::rstest;

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

#[rstest]
#[case::exact_match("abc", true)]
#[case::second_tag("xyz", true)]
#[case::first_char_differs("xbc", false)]
#[case::middle_char_differs("axc", false)]
#[case::last_char_differs("abx", false)]
#[case::different_length_shorter("ab", false)]
#[case::different_length_longer("abcd", false)]
#[case::completely_different("zzz", false)]
fn has_tag_same_length_different_chars(
    #[case] tag: &str,
    #[case] expected: bool,
) {
    let meta = FieldMeta {
        name: "test",
        serialized_name: "test",
        category: "text",
        tags: &["abc", "xyz"],
    };
    assert_eq!(meta.has_tag(tag), expected);
}

#[test]
fn has_tag_empty_tags() {
    let meta = FieldMeta {
        name: "test",
        serialized_name: "test",
        category: "text",
        tags: &[],
    };
    assert!(!meta.has_tag("any"));
}

#[rstest]
#[case::empty_matches_empty("", true)]
#[case::single_char_no_match("x", false)]
#[case::nonempty_matches("nonempty", true)]
fn has_tag_empty_string(#[case] tag: &str, #[case] expected: bool) {
    let meta = FieldMeta {
        name: "test",
        serialized_name: "test",
        category: "text",
        tags: &["", "nonempty"],
    };
    assert_eq!(meta.has_tag(tag), expected);
}

#[rstest]
#[case::numeric("numeric", true)]
#[case::text("text", false)]
#[case::empty("", false)]
fn has_category(#[case] category: &str, #[case] expected: bool) {
    let meta = FieldMeta {
        name: "test",
        serialized_name: "test",
        category: "numeric",
        tags: &[],
    };
    assert_eq!(meta.has_category(category), expected);
}

#[rstest]
#[case::all_match(Some("field_a"), Some("numeric"), Some("primary"), true)]
#[case::name_only(Some("field_a"), None, None, true)]
#[case::category_only(None, Some("numeric"), None, true)]
#[case::tag_only(None, None, Some("indexed"), true)]
#[case::name_mismatch(Some("wrong"), Some("numeric"), Some("primary"), false)]
#[case::category_mismatch(
    Some("field_a"),
    Some("text"),
    Some("primary"),
    false
)]
#[case::tag_mismatch(Some("field_a"), Some("numeric"), Some("wrong"), false)]
#[case::all_none(None, None, None, true)]
fn matches(
    #[case] name: Option<&str>,
    #[case] category: Option<&str>,
    #[case] tag: Option<&str>,
    #[case] expected: bool,
) {
    assert_eq!(TestStruct::FIELDS[0].matches(name, category, tag), expected);
}

#[test]
fn find_by_name() {
    let found = TestStruct::find_by_name("field_a");
    assert!(found.is_some());
    assert_eq!(found.unwrap().serialized_name, "fieldA");

    let not_found = TestStruct::find_by_name("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn find_by_serialized_name() {
    let found = TestStruct::find_by_serialized_name("fieldA");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "field_a");

    let not_found = TestStruct::find_by_serialized_name("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn filter_by_category_iter() {
    let numeric: Vec<_> = TestStruct::filter_by_category("numeric")
        .map(|f| f.name)
        .collect();
    assert_eq!(numeric, vec!["field_a"]);

    let text: Vec<_> = TestStruct::filter_by_category("text")
        .map(|f| f.name)
        .collect();
    assert_eq!(text, vec!["field_b"]);

    let empty = TestStruct::filter_by_category("bool").next().is_none();
    assert!(empty);
}

#[test]
fn filter_by_tag_iter() {
    let indexed: Vec<_> = TestStruct::filter_by_tag("indexed")
        .map(|f| f.name)
        .collect();
    assert_eq!(indexed, vec!["field_a", "field_b"]);

    let primary: Vec<_> = TestStruct::filter_by_tag("primary")
        .map(|f| f.name)
        .collect();
    assert_eq!(primary, vec!["field_a"]);

    let empty = TestStruct::filter_by_tag("nonexistent").next().is_none();
    assert!(empty);
}

#[test]
fn field_names_iter() {
    let names: Vec<_> = TestStruct::field_names_iter().collect();
    assert_eq!(names, vec!["field_a", "field_b"]);
}

#[test]
fn serialized_names_iter() {
    let names: Vec<_> = TestStruct::serialized_names_iter().collect();
    assert_eq!(names, vec!["fieldA", "field_b"]);
}
