//! `#[field_kinds(category = ...)]` gives a category to field types that
//! cannot implement `Categorized`, such as types from other crates.

#![allow(dead_code)]

use field_kinds::{
    Category, FieldInfo, FieldKinds, FieldKindsExt, Numeric, Text,
    TypeCategory, Unknown, VisitFields,
};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Copy)]
struct Binary;

impl TypeCategory for Binary {
    const NAME: &'static str = "binary";
}

struct Blob(Vec<u8>);

#[derive(FieldKinds)]
struct Record {
    #[field_kinds(category = Numeric)]
    elapsed: Duration,
    #[field_kinds(category = Text)]
    path: PathBuf,
    #[field_kinds(category = Binary)]
    payload: Blob,
    #[field_kinds(category = Unknown)]
    started: SystemTime,
    #[field_kinds(category = Text)]
    version: u32,
    count: u64,
    #[field_kinds(skip)]
    cache: Vec<u8>,
}

#[test]
fn category_attribute_replaces_the_categorized_impl() {
    assert_eq!(Record::field_category("elapsed"), Some(Category::NUMERIC));
    assert_eq!(Record::field_category("path"), Some(Category::TEXT));
    assert_eq!(Record::field_category("payload"), Some(Binary::CATEGORY));
    assert_eq!(Record::field_category("started"), Some(Category::UNKNOWN));
}

#[test]
fn category_attribute_overrides_an_existing_impl() {
    assert_eq!(Record::field_category("version"), Some(Category::TEXT));
}

#[test]
fn fields_without_the_attribute_keep_their_impl() {
    assert_eq!(Record::field_category("count"), Some(Category::NUMERIC));
    assert_eq!(Record::FIELD_COUNT, 6);
    assert!(!Record::has_field("cache"));
}

#[test]
fn markers_report_the_given_category() {
    assert_eq!(<record_fields::Path as FieldInfo>::CATEGORY_NAME, "text");
    assert_eq!(<record_fields::Payload as FieldInfo>::CATEGORY_NAME, "binary");
    let _: <record_fields::Elapsed as FieldInfo>::Value =
        Duration::from_secs(1);
}
