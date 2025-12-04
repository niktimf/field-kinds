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
