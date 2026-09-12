#![allow(dead_code)]

use field_kinds::{
    Categorized, Category, FieldInfo, FieldKinds, FieldKindsExt, VisitFields,
};

#[derive(FieldKinds)]
struct Wrapper<T: Categorized> {
    value: T,
}

#[test]
fn single_type_param() {
    assert_eq!(Wrapper::<i32>::FIELD_COUNT, 1);
    assert_eq!(Wrapper::<i32>::field_names(), vec!["value"]);
    assert_eq!(
        Wrapper::<i32>::field_category("value"),
        Some(Category::NUMERIC)
    );
    assert_eq!(
        Wrapper::<String>::field_category("value"),
        Some(Category::TEXT)
    );
    assert_eq!(Wrapper::<bool>::field_category("value"), Some(Category::BOOL));
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
    assert_eq!(
        Pair::<i32, String>::field_category("first"),
        Some(Category::NUMERIC)
    );
    assert_eq!(
        Pair::<i32, String>::field_category("second"),
        Some(Category::TEXT)
    );
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
    assert_eq!(Mixed::<i32>::field_category("name"), Some(Category::TEXT));
    assert_eq!(Mixed::<i32>::field_category("value"), Some(Category::NUMERIC));
    assert_eq!(Mixed::<i32>::field_category("active"), Some(Category::BOOL));
}

#[derive(FieldKinds)]
struct Borrowed<'a> {
    text: &'a str,
    count: u32,
}

#[test]
fn lifetime_param() {
    assert_eq!(Borrowed::field_names(), vec!["text", "count"]);
    assert_eq!(Borrowed::field_category("text"), Some(Category::TEXT));
    assert_eq!(Borrowed::field_category("count"), Some(Category::NUMERIC));
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
    assert_eq!(Complex::<i32>::field_category("label"), Some(Category::TEXT));
    assert_eq!(
        Complex::<i32>::field_category("value"),
        Some(Category::NUMERIC)
    );
    assert_eq!(Complex::<bool>::field_category("value"), Some(Category::BOOL));
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
    assert_eq!(
        Constrained::<String>::field_category("data"),
        Some(Category::TEXT)
    );
}

#[derive(FieldKinds)]
struct Buffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

#[test]
fn const_param_only() {
    assert_eq!(Buffer::<4>::FIELD_COUNT, 2);
    assert_eq!(Buffer::<4>::field_names(), vec!["data", "len"]);
    assert_eq!(Buffer::<4>::field_category("data"), Some(Category::COLLECTION));
    let _: <buffer_fields::Data<4> as FieldInfo>::Value = [0_u8; 4];
}

#[derive(FieldKinds)]
struct Grid<T: Categorized, const W: usize, const H: usize> {
    cells: [[T; W]; H],
    fill: T,
}

#[test]
fn type_and_const_params() {
    assert_eq!(Grid::<bool, 2, 3>::field_names(), vec!["cells", "fill"]);
    assert_eq!(
        Grid::<bool, 2, 3>::field_category("fill"),
        Some(Category::BOOL)
    );
    let _: <grid_fields::Cells<bool, 2, 3> as FieldInfo>::Value =
        [[false; 2]; 3];
}

#[derive(FieldKinds)]
struct Boxed<T: ?Sized>
where
    Box<T>: Categorized,
{
    value: Box<T>,
}

#[test]
fn unsized_type_param() {
    assert_eq!(Boxed::<str>::field_category("value"), Some(Category::TEXT));
    let _: <boxed_fields::Value<str> as FieldInfo>::Value = Box::from("");
}

#[derive(FieldKinds)]
struct Page<'a, T: Categorized> {
    items: &'a [T],
    total: usize,
}

#[test]
fn borrowed_generic_field() {
    assert_eq!(Page::<u8>::field_names(), vec!["items", "total"]);
    assert_eq!(Page::<u8>::field_category("items"), Some(Category::COLLECTION));
    let _: <page_fields::Items<'static, u8> as FieldInfo>::Value = &[1_u8];
}

#[derive(FieldKinds)]
struct Borrowing<'a, T: ?Sized>
where
    &'a T: Categorized,
{
    value: &'a T,
}

#[test]
fn reference_to_a_generic_field() {
    assert_eq!(
        Borrowing::<'static, str>::field_category("value"),
        Some(Category::TEXT)
    );
    assert_eq!(
        Borrowing::<'static, [u8]>::field_category("value"),
        Some(Category::COLLECTION)
    );
}

/// Defaults on generic parameters belong to the struct definition only: an
/// `impl` block that repeated them would not compile, so the derive has to
/// strip them when generating `FieldInfo` impls.
#[derive(FieldKinds)]
struct Defaulted<T: Categorized = i32, const N: usize = 4> {
    value: T,
    data: [u8; N],
}

#[test]
fn generic_params_with_defaults() {
    assert_eq!(Defaulted::<i32, 4>::FIELD_COUNT, 2);
    assert_eq!(Defaulted::<i32, 4>::field_names(), vec!["value", "data"]);
    assert_eq!(
        Defaulted::<String, 2>::field_category("value"),
        Some(Category::TEXT)
    );
    assert_eq!(
        Defaulted::<i32, 2>::field_category("data"),
        Some(Category::COLLECTION)
    );
    let _: <defaulted_fields::Value<bool, 8> as FieldInfo>::Value = true;
}
