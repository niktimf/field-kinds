#![allow(dead_code, non_snake_case)]

use field_kinds::{FieldKinds, FieldKindsExt, VisitFields};

#[derive(FieldKinds)]
#[serde(rename_all = "camelCase")]
struct CamelCaseStruct {
    user_name: String,
    created_at: u64,
    is_active: bool,
}

#[test]
fn rename_all_camel_case() {
    let names = CamelCaseStruct::serialized_names();
    assert_eq!(names, vec!["userName", "createdAt", "isActive"]);
}

#[derive(FieldKinds)]
#[serde(rename_all = "snake_case")]
struct SnakeCaseStruct {
    #[allow(non_snake_case)]
    userName: String,
    #[allow(non_snake_case)]
    createdAt: u64,
}

#[test]
fn rename_all_snake_case() {
    let names = SnakeCaseStruct::serialized_names();
    assert_eq!(
        names,
        vec!["userName", "createdAt"],
        "serde's snake_case leaves field names untouched: \
         they are assumed to be snake_case already"
    );
}

#[derive(FieldKinds)]
#[serde(rename_all = "PascalCase")]
struct PascalCaseStruct {
    user_name: String,
    is_active: bool,
}

#[test]
fn rename_all_pascal_case() {
    let names = PascalCaseStruct::serialized_names();
    assert_eq!(names, vec!["UserName", "IsActive"]);
}

#[derive(FieldKinds)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingSnakeStruct {
    user_name: String,
    is_active: bool,
}

#[test]
fn rename_all_screaming_snake() {
    let names = ScreamingSnakeStruct::serialized_names();
    assert_eq!(names, vec!["USER_NAME", "IS_ACTIVE"]);
}

#[derive(FieldKinds)]
#[serde(rename_all = "kebab-case")]
struct KebabCaseStruct {
    user_name: String,
    is_active: bool,
}

#[test]
fn rename_all_kebab_case() {
    let names = KebabCaseStruct::serialized_names();
    assert_eq!(names, vec!["user-name", "is-active"]);
}

// Field-level rename
#[derive(FieldKinds)]
struct FieldRenameStruct {
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "displayName")]
    name: String,
    normal_field: bool,
}

#[test]
fn field_rename() {
    let names = FieldRenameStruct::serialized_names();
    assert_eq!(names, vec!["ID", "displayName", "normal_field"]);
}

// Combination of rename_all + field rename
#[derive(FieldKinds)]
#[serde(rename_all = "camelCase")]
struct CombinedRenameStruct {
    user_name: String,
    #[serde(rename = "ID")]
    user_id: u64,
    is_active: bool,
}

#[test]
fn combined_rename() {
    let names = CombinedRenameStruct::serialized_names();
    assert_eq!(names, vec!["userName", "ID", "isActive"]);
}

#[derive(FieldKinds)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
struct ScreamingKebabStruct {
    user_name: String,
    is_active: bool,
}

#[test]
fn rename_all_screaming_kebab_case() {
    let names = ScreamingKebabStruct::serialized_names();
    assert_eq!(
        names,
        vec!["USER-NAME", "IS-ACTIVE"],
        "SCREAMING-KEBAB-CASE silently ignored, fields not renamed"
    );
}

#[derive(FieldKinds)]
#[serde(rename_all = "lowercase")]
struct LowercaseStruct {
    #[allow(non_snake_case)]
    SomeField: String,
}

#[test]
fn rename_all_lowercase() {
    let names = LowercaseStruct::serialized_names();
    assert_eq!(
        names,
        vec!["SomeField"],
        "serde's lowercase leaves field names untouched: \
         they are assumed to be lowercase snake_case already"
    );
}

#[derive(FieldKinds)]
#[serde(rename_all = "UPPERCASE")]
struct UppercaseStruct {
    some_field: String,
}

#[test]
fn rename_all_uppercase() {
    let names = UppercaseStruct::serialized_names();
    assert_eq!(
        names,
        vec!["SOME_FIELD"],
        "UPPERCASE silently ignored, field not renamed"
    );
}

#[derive(FieldKinds)]
#[serde]
#[serde(rename_all = "camelCase")]
struct BareSerdeAttr {
    user_name: String,
}

#[test]
fn bare_serde_attr_should_not_swallow_rename_all() {
    let names = BareSerdeAttr::serialized_names();
    assert_eq!(
        names,
        vec!["userName"],
        "#[serde] without args caused ok(?) to return None, \
         skipping the valid rename_all on the next attribute"
    );
}

#[derive(FieldKinds)]
struct BareSerdeFieldAttr {
    #[serde]
    #[serde(rename = "custom_name")]
    field: String,
}

#[test]
fn bare_serde_on_field_should_not_swallow_rename() {
    let names = BareSerdeFieldAttr::serialized_names();
    assert_eq!(
        names,
        vec!["custom_name"],
        "#[serde] without args on field caused ok(?) to return None, \
         skipping the valid rename on the next attribute"
    );
}

#[derive(FieldKinds)]
struct OptionsBeforeRename {
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayName")]
    name: Option<String>,
    #[serde(default = "default_id", alias = "identifier", rename = "ID")]
    id: u64,
}

#[test]
fn options_with_values_before_rename_should_not_hide_it() {
    let names = OptionsBeforeRename::serialized_names();
    assert_eq!(
        names,
        vec!["displayName", "ID"],
        "`key = value` options before `rename` were not consumed, \
         so parsing stopped before reaching `rename`"
    );
}

#[derive(FieldKinds)]
#[serde(rename = "Account", rename_all = "camelCase")]
struct ContainerRenameBeforeRenameAll {
    user_name: String,
}

#[test]
fn container_rename_before_rename_all_should_not_hide_it() {
    let names = ContainerRenameBeforeRenameAll::serialized_names();
    assert_eq!(
        names,
        vec!["userName"],
        "container `rename = ..` before `rename_all` was not consumed, \
         so parsing stopped before reaching `rename_all`"
    );
}

#[derive(FieldKinds)]
struct ListOptionBeforeRename<T> {
    #[serde(bound(serialize = "T: Clone"), rename = "item")]
    value: Vec<T>,
}

#[test]
fn list_option_before_rename_should_not_hide_it() {
    let names = ListOptionBeforeRename::<u8>::serialized_names();
    assert_eq!(
        names,
        vec!["item"],
        "`bound(..)` before `rename` was not consumed, \
         so parsing stopped before reaching `rename`"
    );
}

#[derive(FieldKinds)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
struct RenamePerDirection {
    user_name: String,
    #[serde(rename(serialize = "ID", deserialize = "id"))]
    user_id: u64,
    #[serde(rename(deserialize = "active"))]
    is_active: bool,
}

#[test]
fn per_direction_renames_should_use_serialize_value() {
    let names = RenamePerDirection::serialized_names();
    assert_eq!(
        names,
        vec!["userName", "ID", "isActive"],
        "`rename(serialize = ..)` and `rename_all(serialize = ..)` \
         were ignored"
    );
}

#[derive(FieldKinds)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingSnakeDigits {
    address_line1: String,
}

#[derive(FieldKinds)]
#[serde(rename_all = "camelCase")]
struct CamelCaseDigits {
    sha256_hash: String,
    base64data: String,
}

#[test]
fn rename_all_should_not_split_words_on_digits() {
    assert_eq!(
        ScreamingSnakeDigits::serialized_names(),
        vec!["ADDRESS_LINE1"],
        "a word boundary was inserted before the digit"
    );
    assert_eq!(
        CamelCaseDigits::serialized_names(),
        vec!["sha256Hash", "base64data"],
        "a word boundary was inserted after the digit"
    );
}

#[derive(FieldKinds)]
struct SerdeSkipStruct {
    kept: String,
    #[serde(skip)]
    ignored: u64,
    #[serde(skip_serializing)]
    write_only: u64,
    #[serde(skip_deserializing)]
    read_only: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    maybe: Option<u64>,
}

#[test]
fn serde_skip_removes_only_the_serialized_name() {
    assert_eq!(
        SerdeSkipStruct::serialized_names(),
        vec!["kept", "read_only", "maybe"],
        "serde does not serialize #[serde(skip)] or #[serde(skip_serializing)] fields"
    );
    assert_eq!(
        SerdeSkipStruct::field_names(),
        vec!["kept", "ignored", "write_only", "read_only", "maybe"],
        "the fields still exist in Rust"
    );
    assert_eq!(SerdeSkipStruct::FIELD_COUNT, 5);
    assert!(SerdeSkipStruct::has_field("ignored"));
    assert!(SerdeSkipStruct::find_by_name("ignored").is_some());
    assert!(SerdeSkipStruct::find_by_serialized_name("ignored").is_none());
}

#[test]
fn field_meta_reports_skipped_serialization() {
    let ignored = SerdeSkipStruct::find_by_name("ignored").unwrap();
    assert!(ignored.skip_serializing);

    let write_only = SerdeSkipStruct::find_by_name("write_only").unwrap();
    assert!(write_only.skip_serializing);

    let kept = SerdeSkipStruct::find_by_name("kept").unwrap();
    assert!(!kept.skip_serializing);

    let conditional = SerdeSkipStruct::find_by_name("maybe").unwrap();
    assert!(
        !conditional.skip_serializing,
        "skip_serializing_if is decided at runtime, not by the metadata"
    );
}

/// A `rename_all` that only names the deserialization side leaves the
/// serialized names alone, and is not mistaken for an unknown rule.
#[derive(FieldKinds)]
#[serde(rename_all(deserialize = "camelCase"))]
struct DeserializeOnlyRenameAll {
    user_id: u32,
    user_name: String,
}

#[test]
fn deserialize_only_rename_all_does_not_rename() {
    assert_eq!(
        DeserializeOnlyRenameAll::serialized_names(),
        ["user_id", "user_name"]
    );
}
