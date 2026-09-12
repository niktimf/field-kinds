//! Field names whose `PascalCase` form is not a usable type name still get a
//! marker type: a keyword, a digit start, underscores only, or a duplicate.

#![allow(dead_code)]

use field_kinds::{
    Category, FieldInfo, FieldKinds, FieldKindsExt, VisitFields,
};

#[derive(FieldKinds)]
struct Edges {
    self_: u8,
    _1: u8,
    __: u8,
}

#[test]
fn names_that_do_not_form_a_type_name() {
    assert_eq!(Edges::field_names(), vec!["self_", "_1", "__"]);
    assert_eq!(Edges::field_category("self_"), Some(Category::NUMERIC));
    assert_eq!(<edges_fields::Self_ as FieldInfo>::NAME, "self_");
    assert_eq!(<edges_fields::_1 as FieldInfo>::NAME, "_1");
    assert_eq!(<edges_fields::__ as FieldInfo>::NAME, "__");
}

#[derive(FieldKinds)]
struct Collide {
    field1: u8,
    field_1: u8,
}

#[test]
fn colliding_marker_names_stay_distinct() {
    assert_eq!(Collide::field_names(), vec!["field1", "field_1"]);
    assert_eq!(Collide::FIELD_COUNT, 2);
    assert_eq!(<collide_fields::Field1 as FieldInfo>::NAME, "field1");
    assert_eq!(<collide_fields::Field1_2 as FieldInfo>::NAME, "field_1");
}
