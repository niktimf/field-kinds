#![allow(dead_code)]

use field_kinds::{Categorized, FieldKinds, FieldKindsExt};

#[derive(FieldKinds)]
struct Wrapper<T: Categorized> {
    value: T,
}

#[test]
fn single_type_param() {
    assert_eq!(Wrapper::<i32>::FIELD_COUNT, 1);
    assert_eq!(Wrapper::<i32>::field_names(), vec!["value"]);
    assert_eq!(Wrapper::<i32>::field_category("value"), Some("numeric"));
    assert_eq!(Wrapper::<String>::field_category("value"), Some("text"));
    assert_eq!(Wrapper::<bool>::field_category("value"), Some("bool"));
}

#[derive(FieldKinds)]
struct Pair<T: Categorized, U: Categorized> {
    first: T,
    second: U,
}

#[test]
fn multiple_type_params() {
    assert_eq!(Pair::<i32, String>::FIELD_COUNT, 2);
    assert_eq!(Pair::<i32, String>::field_names(), vec!["first", "second"]);
    assert_eq!(Pair::<i32, String>::field_category("first"), Some("numeric"));
    assert_eq!(Pair::<i32, String>::field_category("second"), Some("text"));
}

#[derive(FieldKinds)]
struct Mixed<T: Categorized> {
    name: String,
    value: T,
    active: bool,
}

#[test]
fn concrete_and_generic_fields() {
    assert_eq!(Mixed::<i32>::FIELD_COUNT, 3);
    assert_eq!(Mixed::<i32>::field_names(), vec!["name", "value", "active"]);
    assert_eq!(Mixed::<i32>::field_category("name"), Some("text"));
    assert_eq!(Mixed::<i32>::field_category("value"), Some("numeric"));
    assert_eq!(Mixed::<i32>::field_category("active"), Some("bool"));
}

#[derive(FieldKinds)]
struct Borrowed<'a> {
    text: &'a str,
    count: u32,
}

#[test]
fn lifetime_param() {
    assert_eq!(Borrowed::field_names(), vec!["text", "count"]);
    assert_eq!(Borrowed::field_category("text"), Some("text"));
    assert_eq!(Borrowed::field_category("count"), Some("numeric"));
}

#[derive(FieldKinds)]
struct Complex<'a, T: Categorized> {
    label: &'a str,
    value: T,
}

#[test]
fn lifetime_and_type_param() {
    assert_eq!(Complex::<i32>::FIELD_COUNT, 2);
    assert_eq!(Complex::<i32>::field_names(), vec!["label", "value"]);
    assert_eq!(Complex::<i32>::field_category("label"), Some("text"));
    assert_eq!(Complex::<i32>::field_category("value"), Some("numeric"));
    assert_eq!(Complex::<bool>::field_category("value"), Some("bool"));
}

#[derive(FieldKinds)]
struct Constrained<T>
where
    T: Categorized + Clone,
{
    data: T,
}

#[test]
fn where_clause() {
    assert_eq!(Constrained::<i32>::field_names(), vec!["data"]);
    assert_eq!(Constrained::<String>::field_category("data"), Some("text"));
}
