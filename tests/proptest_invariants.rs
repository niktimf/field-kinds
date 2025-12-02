use field_kinds::{FieldKinds, FieldKindsExt};
use proptest::prelude::*;

// Тестовая структура для property тестов
#[derive(FieldKinds)]
struct PropTestStruct {
    #[field_tags("a", "b")]
    field1: String,
    #[field_tags("b", "c")]
    field2: i32,
    #[field_tags("c")]
    field3: bool,
    field4: Vec<u8>,
}

// Инвариант: FIELD_COUNT == field_names().len()
#[test]
fn invariant_count_matches_names_len() {
    assert_eq!(
        PropTestStruct::FIELD_COUNT,
        PropTestStruct::field_names().len()
    );
}

// Инвариант: field_names() == serialized_names() когда нет rename
#[derive(FieldKinds)]
struct NoRenameStruct {
    alpha: i32,
    beta: String,
}

#[test]
fn invariant_names_equal_without_rename() {
    assert_eq!(
        NoRenameStruct::field_names(),
        NoRenameStruct::serialized_names()
    );
}

// Инвариант: все имена уникальны
#[test]
fn invariant_names_unique() {
    let names = PropTestStruct::field_names();
    let unique: std::collections::HashSet<_> = names.iter().collect();
    assert_eq!(names.len(), unique.len());
}

// Инвариант: has_field согласован с field_names
#[test]
fn invariant_has_field_consistent() {
    for name in PropTestStruct::field_names() {
        assert!(
            PropTestStruct::has_field(name),
            "has_field должен быть true для {}",
            name
        );
    }
}

// Инвариант: field_category возвращает Some для всех существующих полей
#[test]
fn invariant_category_exists_for_all_fields() {
    for name in PropTestStruct::field_names() {
        assert!(
            PropTestStruct::field_category(name).is_some(),
            "field_category должен вернуть Some для {}",
            name
        );
    }
}

// Инвариант: fields_by_category покрывает все поля
#[test]
fn invariant_categories_cover_all_fields() {
    let categories = [
        "numeric",
        "text",
        "bool",
        "optional",
        "collection",
        "unknown",
    ];
    let mut all_fields: Vec<&str> = Vec::new();

    for cat in categories {
        all_fields.extend(PropTestStruct::fields_by_category(cat));
    }

    let mut expected = PropTestStruct::field_names();
    all_fields.sort();
    expected.sort();

    assert_eq!(all_fields, expected);
}

#[test]
fn invariant_meta_len() {
    assert_eq!(
        PropTestStruct::field_meta().len(),
        PropTestStruct::FIELD_COUNT
    );
}

proptest! {
    #[test]
    fn prop_fields_by_tag_never_panics(tag in "[a-z]{1,10}") {
        let _ = PropTestStruct::fields_by_tag(&tag);
    }

    #[test]
    fn prop_has_field_never_panics(name in "[a-z_]{1,20}") {
        let _ = PropTestStruct::has_field(&name);
    }

    #[test]
    fn prop_field_category_never_panics(name in "[a-z_]{1,20}") {
        let _ = PropTestStruct::field_category(&name);
    }

    #[test]
    fn prop_fields_by_category_never_panics(cat in "[a-z]{1,15}") {
        let _ = PropTestStruct::fields_by_category(&cat);
    }
}
