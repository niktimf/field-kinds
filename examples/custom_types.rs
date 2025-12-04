#![allow(dead_code)]

use field_kinds::{
    Categorized, FieldKinds, FieldKindsExt, Numeric, Text, TypeCategory,
    Unknown,
};

pub struct Money(i64);

impl Categorized for Money {
    type Category = Numeric;
}

pub struct Email(String);

impl Categorized for Email {
    type Category = Text;
}

pub struct CustomData {
    value: Vec<u8>,
}

impl Categorized for CustomData {
    type Category = Unknown;
}

#[derive(Debug, Clone, Copy)]
pub struct Binary;

impl TypeCategory for Binary {
    const NAME: &'static str = "binary";
}

pub struct Blob(Vec<u8>);

impl Categorized for Blob {
    type Category = Binary;
}

#[derive(FieldKinds)]
struct Order {
    id: u64,
    total: Money,
    customer_email: Email,
    metadata: CustomData,
    attachment: Blob,
}

fn main() {
    println!("=== Custom Type Categories ===");
    println!("Field names: {:?}", Order::field_names());

    println!("\n=== Categories ===");
    for field in Order::field_meta() {
        println!("  {}: {}", field.name, field.category);
    }

    println!("\n=== Filter by Category ===");
    println!("Numeric fields: {:?}", Order::fields_by_category("numeric"));
    println!("Text fields: {:?}", Order::fields_by_category("text"));
    println!("Unknown fields: {:?}", Order::fields_by_category("unknown"));
    println!("Binary fields: {:?}", Order::fields_by_category("binary"));
}
