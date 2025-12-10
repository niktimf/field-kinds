# field-kinds

[![Crates.io](https://img.shields.io/crates/v/field-kinds.svg)](https://crates.io/crates/field-kinds)
[![Documentation](https://docs.rs/field-kinds/badge.svg)](https://docs.rs/field-kinds)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Compile-time struct field introspection for Rust.

## Features

- **Field names** - Get field names as `&'static str`
- **Serialized names** - Supports `#[serde(rename)]` and `#[serde(rename_all)]`
- **Type categories** - Automatic categorization: numeric, text, bool, optional, collection
- **Custom tags** - Add arbitrary tags via `#[field_tags("tag1", "tag2")]`
- **Static metadata** - All field info available as `const FIELDS: &'static [FieldMeta]`
- **Zero runtime cost** - All metadata computed at compile time

## Installation

```toml
[dependencies]
field-kinds = "0.2"
```

## Quick Start

```rust
use field_kinds::{FieldKinds, FieldKindsExt, VisitFields};

#[derive(FieldKinds)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    user_name: String,
    is_active: bool,
    #[field_tags("sensitive", "pii")]
    email: Option<String>,
}

fn main() {
    // Field names
    assert_eq!(User::field_names(), vec!["user_id", "user_name", "is_active", "email"]);
    
    // Serialized names (with rename_all applied)
    assert_eq!(User::serialized_names(), vec!["userId", "userName", "isActive", "email"]);
    
    // Filter by category
    assert_eq!(User::fields_by_category("numeric"), vec!["user_id"]);
    assert_eq!(User::fields_by_category("text"), vec!["user_name"]);
    assert_eq!(User::fields_by_category("optional"), vec!["email"]);
    
    // Filter by tag
    assert_eq!(User::fields_by_tag("sensitive"), vec!["email"]);
    
    // Check field existence
    assert!(User::has_field("user_id"));
    assert!(!User::has_field("nonexistent"));
    
    // Get field category
    assert_eq!(User::field_category("user_id"), Some("numeric"));
    
    // Access static metadata directly
    assert_eq!(User::FIELDS.len(), 4);
    
    // Iterate over field metadata
    for field in User::FIELDS {
        println!("{}: {} [{:?}]", field.name, field.category, field.tags);
    }
}
```

## Attributes

### Struct-level

| Attribute | Description |
|-----------|-------------|
| `#[serde(rename_all = "...")]` | Apply case conversion to serialized names |

Supported cases: `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, `kebab-case`

### Field-level

| Attribute | Description |
|-----------|-------------|
| `#[serde(rename = "...")]` | Override serialized name |
| `#[field_tags("tag1", "tag2")]` | Add custom tags |
| `#[field_kinds(skip)]` | Exclude field from introspection |

## Type Categories

Types are automatically categorized:

| Category | Types |
|----------|-------|
| `numeric` | `i8`-`i128`, `u8`-`u128`, `f32`, `f64`, `isize`, `usize` |
| `text` | `String`, `&str`, `Box<str>`, `char` |
| `bool` | `bool` |
| `optional` | `Option<T>` |
| `collection` | `Vec<T>`, `HashSet<T>`, `HashMap<K,V>`, `BTreeSet<T>`, `BTreeMap<K,V>`, `[T; N]`, `&[T]` |
| `unknown` | Everything else |

### Custom Categories

Implement `Categorized` for your types:

```rust
use field_kinds::{Categorized, Numeric};

struct Money(i64);

impl Categorized for Money {
    type Category = Numeric;
}
```

## MSRV

Minimum supported Rust version is **1.90.0** (Rust 2024 edition).

## License

MIT
