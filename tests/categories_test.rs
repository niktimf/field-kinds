#![allow(clippy::needless_pass_by_value)]

use field_kinds::{
    Bool, Categorized, Category, Collection, Numeric, Optional, Text,
    TypeCategory, Unknown,
};
use rstest::rstest;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};
use std::sync::Arc;

const fn category_of<T: Categorized>() -> Category {
    <T::Category as TypeCategory>::CATEGORY
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
#[case::non_zero_u8(category_of::<NonZeroU8>())]
#[case::non_zero_u16(category_of::<NonZeroU16>())]
#[case::non_zero_u32(category_of::<NonZeroU32>())]
#[case::non_zero_u64(category_of::<NonZeroU64>())]
#[case::non_zero_u128(category_of::<NonZeroU128>())]
#[case::non_zero_usize(category_of::<NonZeroUsize>())]
#[case::non_zero_i8(category_of::<NonZeroI8>())]
#[case::non_zero_i16(category_of::<NonZeroI16>())]
#[case::non_zero_i32(category_of::<NonZeroI32>())]
#[case::non_zero_i64(category_of::<NonZeroI64>())]
#[case::non_zero_i128(category_of::<NonZeroI128>())]
#[case::non_zero_isize(category_of::<NonZeroIsize>())]
fn numeric_types(#[case] category: Category) {
    assert_eq!(category, Category::NUMERIC);
}

#[rstest]
#[case::string(category_of::<String>())]
#[case::str_ref(category_of::<&str>())]
#[case::box_str(category_of::<Box<str>>())]
#[case::char(category_of::<char>())]
#[case::cow_str(category_of::<Cow<'_, str>>())]
#[case::arc_str(category_of::<Arc<str>>())]
#[case::rc_str(category_of::<std::rc::Rc<str>>())]
fn text_types(#[case] category: Category) {
    assert_eq!(category, Category::TEXT);
}

#[rstest]
#[case::bool_type(category_of::<bool>())]
fn bool_type(#[case] category: Category) {
    assert_eq!(category, Category::BOOL);
}

#[rstest]
#[case::option_i32(category_of::<Option<i32>>())]
#[case::option_string(category_of::<Option<String>>())]
#[case::option_vec(category_of::<Option<Vec<u8>>>())]
fn optional_types(#[case] category: Category) {
    assert_eq!(category, Category::OPTIONAL);
}

#[rstest]
#[case::vec(category_of::<Vec<i32>>())]
#[case::array(category_of::<[i32; 5]>())]
#[case::slice(category_of::<&[i32]>())]
#[case::hashset(category_of::<HashSet<String>>())]
#[case::btreeset(category_of::<BTreeSet<i32>>())]
#[case::hashmap(category_of::<HashMap<String, i32>>())]
#[case::btreemap(category_of::<BTreeMap<String, i32>>())]
fn collection_types(#[case] category: Category) {
    assert_eq!(category, Category::COLLECTION);
}
