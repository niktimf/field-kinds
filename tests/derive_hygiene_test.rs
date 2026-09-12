//! Generated code must resolve names the way the struct definition does:
//! a field name, which also names the field's marker type, must not shadow
//! the types and traits the struct refers to.

#![allow(dead_code)]

use field_kinds::{
    Bool, Categorized, Category, FieldInfo, FieldKinds, FieldKindsExt, Numeric,
    Text, Unknown,
};

/// Compiles only when `A` and `B` are the same type.
const fn assert_type_eq<A: ?Sized + SameAs<B>, B: ?Sized>() {}

trait SameAs<T: ?Sized> {}

impl<T: ?Sized> SameAs<T> for T {}

pub enum Status {
    Active,
}

impl Categorized for Status {
    type Category = Unknown;
}

#[derive(FieldKinds)]
struct Account {
    status: Status,
}

#[test]
fn field_named_after_its_type() {
    assert_eq!(Account::field_category("status"), Some(Category::UNKNOWN));
    assert_type_eq::<<account_fields::Status as FieldInfo>::Value, Status>();
}

#[derive(FieldKinds)]
struct Buffers {
    vec: Vec<u8>,
    string: String,
    option: Option<u8>,
    r#box: Box<str>,
}

#[test]
fn fields_named_after_prelude_types() {
    assert_eq!(Buffers::field_names(), vec!["vec", "string", "option", "box"]);
    assert_type_eq::<<buffers_fields::Vec as FieldInfo>::Value, Vec<u8>>();
    assert_type_eq::<<buffers_fields::String as FieldInfo>::Value, String>();
    assert_type_eq::<<buffers_fields::Option as FieldInfo>::Value, Option<u8>>(
    );
    assert_type_eq::<<buffers_fields::Box as FieldInfo>::Value, Box<str>>();
}

#[derive(FieldKinds)]
struct Introspection {
    field_info: u8,
    categorized: bool,
    type_category: String,
}

#[test]
fn fields_named_after_field_kinds_traits() {
    assert_eq!(
        Introspection::field_names(),
        vec!["field_info", "categorized", "type_category"]
    );
    assert_eq!(
        <introspection_fields::FieldInfo as FieldInfo>::NAME,
        "field_info"
    );
    assert_eq!(
        <introspection_fields::TypeCategory as FieldInfo>::CATEGORY_NAME,
        "text"
    );
}

#[derive(FieldKinds)]
struct Settings<T: Default + Categorized, U>
where
    U: Clone + Categorized,
{
    default: T,
    clone: U,
}

#[test]
fn field_names_shadowing_generic_bounds() {
    assert_eq!(
        Settings::<u8, bool>::field_category("default"),
        Some(Category::NUMERIC)
    );
    assert_eq!(
        Settings::<u8, bool>::field_category("clone"),
        Some(Category::BOOL)
    );
    assert_type_eq::<
        <settings_fields::Default<u8, bool> as FieldInfo>::Value,
        u8,
    >();
    assert_type_eq::<
        <settings_fields::Clone<u8, bool> as FieldInfo>::Category,
        Bool,
    >();
}

#[derive(FieldKinds)]
#[serde(rename_all = "camelCase")]
struct Profile {
    #[field_tags("pii")]
    display_name: String,
    login_count: u32,
    #[field_kinds(skip)]
    cache: Vec<u8>,
}

#[test]
fn markers_describe_their_fields() {
    type DisplayName = profile_fields::DisplayName;
    type LoginCount = profile_fields::LoginCount;

    assert_eq!(DisplayName::NAME, "display_name");
    assert_eq!(DisplayName::SERIALIZED_NAME, "displayName");
    assert_eq!(DisplayName::CATEGORY_NAME, "text");
    assert_eq!(DisplayName::TAGS, &["pii"]);
    assert_type_eq::<<DisplayName as FieldInfo>::Value, String>();
    assert_type_eq::<<DisplayName as FieldInfo>::Category, Text>();

    assert_eq!(LoginCount::SERIALIZED_NAME, "loginCount");
    assert!(LoginCount::TAGS.is_empty());
    assert_type_eq::<<LoginCount as FieldInfo>::Category, Numeric>();
}
