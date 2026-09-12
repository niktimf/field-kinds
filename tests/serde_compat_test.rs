//! `serialized_names()` must match the keys serde actually emits.
//!
//! Every struct here derives both `Serialize` and `FieldKinds`. Each field
//! holds its own index, so the tests pin every serialized name to its field
//! instead of comparing unordered sets.

#![allow(dead_code, non_snake_case)]

use field_kinds::{FieldKinds, FieldKindsExt, VisitFields};
use serde::Serialize;

#[track_caller]
fn assert_serialized_names_match_serde<T: Serialize + VisitFields>(value: &T) {
    let json = serde_json::to_value(value).unwrap();
    let object = json.as_object().unwrap();
    let serde_keys: Vec<&String> = object.keys().collect();

    assert_eq!(object.len(), T::FIELD_COUNT, "serde keys: {serde_keys:?}");
    for (index, field) in T::FIELDS.iter().enumerate() {
        assert_eq!(
            object.get(field.serialized_name),
            Some(&serde_json::json!(index)),
            "field `{}` reports `{}`, serde keys: {serde_keys:?}",
            field.name,
            field.serialized_name,
        );
    }
}

const fn default_id() -> u8 {
    0
}

#[derive(Serialize, FieldKinds)]
#[serde(rename = "Account", deny_unknown_fields, rename_all = "camelCase")]
struct SerdeOptionsAroundRenames {
    user_name: u8,
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayName")]
    name: Option<u8>,
    #[serde(default = "default_id", alias = "identifier", rename = "ID")]
    id: u8,
    #[serde(rename(serialize = "EMAIL", deserialize = "email"))]
    email: u8,
    #[serde(rename(deserialize = "legacy_flag"), default)]
    is_active: u8,
}

#[test]
fn serde_options_around_renames() {
    assert_serialized_names_match_serde(&SerdeOptionsAroundRenames {
        user_name: 0,
        name: Some(1),
        id: 2,
        email: 3,
        is_active: 4,
    });
}

#[derive(Serialize, FieldKinds)]
#[serde(rename_all(serialize = "kebab-case", deserialize = "camelCase"))]
struct RenameAllPerDirection {
    user_name: u8,
    is_active: u8,
}

#[test]
fn rename_all_per_direction() {
    assert_serialized_names_match_serde(&RenameAllPerDirection {
        user_name: 0,
        is_active: 1,
    });
}

/// For every `rename_all` rule, declares a struct whose field names cover
/// what serde's field renaming is sensitive to: digits, leading, trailing
/// and repeated underscores, names that are not `snake_case`, and raw
/// identifiers.
///
/// No two fields may collide under any rule, or serde would emit fewer keys.
macro_rules! rename_all_matches_serde {
    ($($module:ident => $rule:tt),* $(,)?) => {$(
        mod $module {
            use super::*;

            #[derive(Serialize, FieldKinds)]
            #[serde(rename_all = $rule)]
            struct Fields {
                user_name: u8,
                field1: u8,
                address_line1: u8,
                line1_text: u8,
                v2beta: u8,
                base64data: u8,
                _leading: u8,
                trailing_: u8,
                double__underscore: u8,
                createdAt: u8,
                SomeField: u8,
                userID: u8,
                r#type: u8,
            }

            #[test]
            fn serialized_names_match_serde() {
                assert_serialized_names_match_serde(&Fields {
                    user_name: 0,
                    field1: 1,
                    address_line1: 2,
                    line1_text: 3,
                    v2beta: 4,
                    base64data: 5,
                    _leading: 6,
                    trailing_: 7,
                    double__underscore: 8,
                    createdAt: 9,
                    SomeField: 10,
                    userID: 11,
                    r#type: 12,
                });
            }
        }
    )*};
}

rename_all_matches_serde! {
    lowercase => "lowercase",
    uppercase => "UPPERCASE",
    pascal_case => "PascalCase",
    camel_case => "camelCase",
    snake_case => "snake_case",
    screaming_snake_case => "SCREAMING_SNAKE_CASE",
    kebab_case => "kebab-case",
    screaming_kebab_case => "SCREAMING-KEBAB-CASE",
}

#[derive(Serialize, FieldKinds)]
struct WithSkippedFields {
    kept: u8,
    #[serde(skip)]
    ignored: u8,
    #[serde(skip_serializing)]
    write_only: u8,
    #[serde(skip_deserializing)]
    read_only: u8,
}

#[test]
fn skipped_fields_are_not_serialized() {
    let value = WithSkippedFields {
        kept: 0,
        ignored: 1,
        write_only: 2,
        read_only: 3,
    };
    let json = serde_json::to_value(&value).unwrap();
    let object = json.as_object().unwrap();
    let mut serde_keys: Vec<&str> = object.keys().map(String::as_str).collect();
    let mut ours = WithSkippedFields::serialized_names().to_vec();
    serde_keys.sort_unstable();
    ours.sort_unstable();
    assert_eq!(ours, serde_keys);
}

/// `SERIALIZED_NAMES` is shorter than `NAMES` when serde skips a field, so
/// the two consts must not be assumed to line up.
#[test]
fn skipped_fields_shorten_serialized_names_const() {
    assert_eq!(
        WithSkippedFields::NAMES,
        vec!["kept", "ignored", "write_only", "read_only"]
    );
    assert_eq!(WithSkippedFields::SERIALIZED_NAMES, vec!["kept", "read_only"]);
}
