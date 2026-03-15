#![allow(dead_code, clippy::struct_field_names)]

use field_kinds::{Category, FieldKinds, FieldKindsExt, VisitFields};

#[derive(FieldKinds)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    user_name: String,
    is_active: bool,
    #[field_tags("sensitive", "pii")]
    email: Option<String>,
    #[field_kinds(skip)]
    internal_cache: Vec<u8>,
}

fn main() {
    println!("=== Field Names ===");
    println!("Original: {:?}", User::field_names());
    println!("Serialized: {:?}", User::serialized_names());

    println!("\n=== Categories ===");
    println!(
        "Numeric fields: {:?}",
        User::fields_by_category(Category::NUMERIC)
    );
    println!("Text fields: {:?}", User::fields_by_category(Category::TEXT));
    println!("Bool fields: {:?}", User::fields_by_category(Category::BOOL));
    println!(
        "Optional fields: {:?}",
        User::fields_by_category(Category::OPTIONAL)
    );

    println!("\n=== Tags ===");
    println!("Sensitive fields: {:?}", User::fields_by_tag("sensitive"));
    println!("PII fields: {:?}", User::fields_by_tag("pii"));

    println!("\n=== Field Lookup ===");
    println!("has 'user_id': {}", User::has_field("user_id"));
    println!("has 'internal_cache': {}", User::has_field("internal_cache"));
    println!("category of 'email': {:?}", User::field_category("email"));

    println!("\n=== Full Metadata ===");
    for field in User::field_meta() {
        println!(
            "  {} ({}) -> {} [tags: {:?}]",
            field.name, field.serialized_name, field.category, field.tags
        );
    }

    println!("\n=== Compile-time Constants ===");
    println!("Field count: {}", User::FIELD_COUNT);
}
