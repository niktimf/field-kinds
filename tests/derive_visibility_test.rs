//! Generated code must compile for every field type the struct accepts,
//! including private types and types declared inside functions, and each
//! marker type must be visible wherever its field is.

#![allow(dead_code)]

use field_kinds::{
    Categorized, Category, FieldInfo, FieldKinds, FieldKindsExt, Numeric,
};

struct Money(i64);

impl Categorized for Money {
    type Category = Numeric;
}

#[derive(FieldKinds)]
struct Order {
    total: Money,
}

#[test]
fn private_field_type() {
    assert_eq!(Order::field_category("total"), Some(Category::NUMERIC));
    assert_eq!(<order_fields::Total as FieldInfo>::NAME, "total");
}

#[test]
fn types_declared_inside_a_function() {
    struct Celsius(f64);

    impl Categorized for Celsius {
        type Category = Numeric;
    }

    #[derive(FieldKinds)]
    struct Reading {
        temperature: Celsius,
    }

    assert_eq!(Reading::field_category("temperature"), Some(Category::NUMERIC));
    assert_eq!(<reading_fields::Temperature as FieldInfo>::NAME, "temperature");
}

mod shop {
    use field_kinds::{Categorized, FieldInfo, FieldKinds, Numeric};

    struct Secret(u64);

    impl Categorized for Secret {
        type Category = Numeric;
    }

    #[derive(FieldKinds)]
    pub struct Product {
        pub name: String,
        pub(crate) sku: u64,
        pub(super) price: u64,
        pub(in crate::shop) cost: u64,
        pub(self) margin: u64,
        hidden: Secret,
    }

    #[test]
    fn restricted_markers_are_visible_where_their_fields_are() {
        assert_eq!(<product_fields::Cost as FieldInfo>::NAME, "cost");
        assert_eq!(<product_fields::Margin as FieldInfo>::NAME, "margin");
        assert_eq!(<product_fields::Hidden as FieldInfo>::NAME, "hidden");
    }
}

#[test]
fn public_markers_are_visible_where_their_fields_are() {
    assert_eq!(<shop::product_fields::Name as FieldInfo>::NAME, "name");
    assert_eq!(<shop::product_fields::Sku as FieldInfo>::NAME, "sku");
    assert_eq!(<shop::product_fields::Price as FieldInfo>::NAME, "price");
}
