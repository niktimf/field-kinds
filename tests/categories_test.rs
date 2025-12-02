use field_kinds::{Bool, Categorized, Collection, Numeric, Optional, Text, TypeCategory, Unknown};
use rstest::rstest;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn category_of<T: Categorized>() -> &'static str {
    <T::Category as TypeCategory>::NAME
}

#[rstest]
#[case::numeric(Numeric::NAME, "numeric")]
#[case::text(Text::NAME, "text")]
#[case::bool(Bool::NAME, "bool")]
#[case::optional(Optional::NAME, "optional")]
#[case::collection(Collection::NAME, "collection")]
#[case::unknown(Unknown::NAME, "unknown")]
fn category_names(#[case] actual: &str, #[case] expected: &str) {
    assert_eq!(actual, expected);
}

#[rstest]
#[case::u8(category_of::<u8>())]
#[case::u16(category_of::<u16>())]
#[case::u32(category_of::<u32>())]
#[case::u64(category_of::<u64>())]
#[case::u128(category_of::<u128>())]
#[case::usize(category_of::<usize>())]
#[case::i8(category_of::<i8>())]
#[case::i16(category_of::<i16>())]
#[case::i32(category_of::<i32>())]
#[case::i64(category_of::<i64>())]
#[case::i128(category_of::<i128>())]
#[case::isize(category_of::<isize>())]
#[case::f32(category_of::<f32>())]
#[case::f64(category_of::<f64>())]
fn numeric_types(#[case] category: &str) {
    assert_eq!(category, "numeric");
}

// Text types
#[rstest]
#[case::string(category_of::<String>())]
#[case::str_ref(category_of::<&str>())]
#[case::box_str(category_of::<Box<str>>())]
#[case::char(category_of::<char>())]
fn text_types(#[case] category: &str) {
    assert_eq!(category, "text");
}

// Bool
#[rstest]
#[case::bool_type(category_of::<bool>())]
fn bool_type(#[case] category: &str) {
    assert_eq!(category, "bool");
}

// Optional
#[rstest]
#[case::option_i32(category_of::<Option<i32>>())]
#[case::option_string(category_of::<Option<String>>())]
#[case::option_vec(category_of::<Option<Vec<u8>>>())]
fn optional_types(#[case] category: &str) {
    assert_eq!(category, "optional");
}

// Collection types
#[rstest]
#[case::vec(category_of::<Vec<i32>>())]
#[case::array(category_of::<[i32; 5]>())]
#[case::slice(category_of::<&[i32]>())]
#[case::hashset(category_of::<HashSet<String>>())]
#[case::btreeset(category_of::<BTreeSet<i32>>())]
#[case::hashmap(category_of::<HashMap<String, i32>>())]
#[case::btreemap(category_of::<BTreeMap<String, i32>>())]
fn collection_types(#[case] category: &str) {
    assert_eq!(category, "collection");
}
