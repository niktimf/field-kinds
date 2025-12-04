#![allow(dead_code)]

use field_kinds::{FieldKinds, FieldKindsExt};

#[derive(FieldKinds)]
struct TaggedStruct {
    #[field_tags("tag1", "sensitive")]
    email: String,
    #[field_tags("tag1")]
    phone: String,
    #[field_tags("indexed")]
    id: u64,
    normal: bool,
}

#[test]
fn fields_by_tag() {
    assert_eq!(TaggedStruct::fields_by_tag("tag1"), vec!["email", "phone"]);
    assert_eq!(TaggedStruct::fields_by_tag("sensitive"), vec!["email"]);
    assert_eq!(TaggedStruct::fields_by_tag("indexed"), vec!["id"]);
    assert!(TaggedStruct::fields_by_tag("nonexistent").is_empty());
}

#[test]
fn field_meta_contains_tags() {
    let meta = TaggedStruct::field_meta();

    let email_meta = meta.iter().find(|m| m.name == "email").unwrap();
    assert_eq!(email_meta.tags, &["tag1", "sensitive"]);

    let normal_meta = meta.iter().find(|m| m.name == "normal").unwrap();
    assert!(normal_meta.tags.is_empty());
}

#[derive(FieldKinds)]
struct SkipStruct {
    visible: String,
    #[field_kinds(skip)]
    hidden: String,
    also_visible: u32,
}

#[test]
fn skip_field() {
    assert_eq!(SkipStruct::FIELD_COUNT, 2);
    assert_eq!(SkipStruct::field_names(), vec!["visible", "also_visible"]);
    assert!(!SkipStruct::has_field("hidden"));
}

#[derive(FieldKinds)]
struct MixedStruct {
    #[field_tags("important")]
    keep: String,
    #[field_kinds(skip)]
    #[field_tags("should_not_appear")]
    skipped: String,
}

#[test]
fn skip_with_tags() {
    assert_eq!(MixedStruct::fields_by_tag("important"), vec!["keep"]);
    assert!(MixedStruct::fields_by_tag("should_not_appear").is_empty());
}
