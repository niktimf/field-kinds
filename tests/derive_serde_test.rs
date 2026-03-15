#![allow(dead_code, non_snake_case)]

use field_kinds::{FieldKinds, FieldKindsExt};

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
    assert_eq!(names, vec!["user_name", "created_at"]);
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
        vec!["somefield"],
        "lowercase silently ignored, field not renamed"
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
