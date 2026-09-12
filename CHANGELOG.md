# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `#[field_kinds(category = ...)]` to set a field's category explicitly, for field types that cannot implement `Categorized`, such as types from other crates
- `FieldMeta::skip_serializing`, set for fields serde leaves out of the serialized output
- `VisitFields::NAMES` and `VisitFields::SERIALIZED_NAMES`, the field names as static slices, usable in const context

### Fixed

- `#[serde(rename)]` and `#[serde(rename_all)]` were silently ignored when an option with a value preceded them in the same attribute, e.g. `#[serde(skip_serializing_if = "Option::is_none", rename = "id")]` (regression in 0.6.0)
- `#[serde(rename(serialize = "..."))]` and `#[serde(rename_all(serialize = "..."))]` are now honored
- `#[serde(rename_all)]` now renames fields exactly like serde: no word splitting on digits or case changes (`address_line1` becomes `ADDRESS_LINE1`, not `ADDRESS_LINE_1`), and `snake_case` / `lowercase` leave field names unchanged
- Raw identifier fields such as `r#type` no longer make the derive panic and are reported without the `r#` prefix
- Deriving no longer fails when a field is named after a type or trait it uses, e.g. `status: Status`, `vec: Vec<u8>` or `default: T` with `T: Default`
- Private field types and types declared inside functions are now supported
- Structs whose only generic parameters are const generics are now supported
- Field names whose `PascalCase` form cannot name a type (`self_`, `_1`, `__`) no longer make the derive panic
- Field names that convert to the same marker type name, such as `field1` and `field_1`, no longer collide
- Generic structs with a borrowed generic field, such as `struct Page<'a, T> { items: &'a [T] }`, now derive: the marker's `FieldInfo` impl spells out the `T: 'a` bound that the struct itself infers

### Changed

- Generated field marker types now have the visibility of their field instead of always being `pub`
- `serialized_names()`, `serialized_names_iter()` and `find_by_serialized_name()` no longer report fields serde does not serialize (`#[serde(skip)]`, `#[serde(skip_serializing)]`)
- **Breaking**: `field_names()` and `serialized_names()` return `&'static [&'static str]` instead of `Vec<&'static str>`, and no longer allocate. Call `.to_vec()` where an owned `Vec` is needed
- **Breaking**: `VisitFields` gained the required `NAMES` and `SERIALIZED_NAMES` consts, which hand-written implementations must provide. The derive generates them
- `syn` dependency updated from 2.0 to 3.0

## [0.6.0] - 2026-03-15

### Added

- `Category` newtype for type-safe category handling with compile-time constants (`Category::NUMERIC`, `Category::TEXT`, etc.)
- `TypeCategory::CATEGORY` associated constant with default implementation
- `PartialEq<&str>` for `Category` for runtime string comparison
- `Display` implementation for `Category`
- `Categorized` implementations for `Cow<str>`, `Arc<str>`, `Rc<str>`
- `Categorized` implementations for all `NonZero*` types
- Support for `lowercase`, `UPPERCASE`, and `SCREAMING-KEBAB-CASE` in `#[serde(rename_all)]`
- `#[non_exhaustive]` on `FieldMeta` for future-proof extensibility

### Fixed

- `#[serde]` without arguments no longer breaks parsing of subsequent serde attributes
- Multiple `#[field_tags]` attributes on a field now merge correctly
- Unsupported `rename_all` variants no longer silently ignored

### Changed

- **Breaking**: `FieldMeta.category` is now `Category` instead of `&'static str`
- **Breaking**: `fields_by_category()`, `filter_by_category()` now take `Category` instead of `&str`
- **Breaking**: `field_category()` returns `Option<Category>` instead of `Option<&'static str>`
- **Breaking**: `has_category()` takes `Category` instead of `&str`
- **Breaking**: `matches()` takes `Option<Category>` instead of `Option<&str>` for category
- **Breaking**: Removed `FieldKinds` trait, `HCons`, `HNil`, `FieldCount`, `HListVisitor` from public API
- `FIELD_COUNT` moved to `VisitFields` trait (with default `Self::FIELDS.len()`)
- Generated field marker module is now `#[doc(hidden)]`
- `syn` dependency reduced from `full` to `derive` feature for faster compilation

## [0.5.0] - 2026-03-07

### Added

- Support for generic structs with type parameters, lifetimes, and where-clauses

### Changed

- **Breaking**: Removed `Copy + 'static` bounds from `FieldInfo` trait

## [0.4.0] - 2026-02-22

### Changed

- **Breaking**: Removed `frunk` dependency — `HCons` and `HNil` are now defined locally
- Zero external runtime dependencies

## [0.3.0] - 2025-12-10

### Added

- `FieldMeta::has_category()` - const fn for checking field category
- `FieldMeta::matches()` - check if field matches multiple criteria (name, category, tag)
- `FieldKindsExt::find_by_name()` - find field metadata by original name
- `FieldKindsExt::find_by_serialized_name()` - find field metadata by serialized name
- `FieldKindsExt::filter_by_category()` - iterator over fields with given category
- `FieldKindsExt::filter_by_tag()` - iterator over fields with given tag
- `FieldKindsExt::field_names_iter()` - iterator over original field names
- `FieldKindsExt::serialized_names_iter()` - iterator over serialized field names

## [0.2.0] - 2025-12-10

### Changed

- **Breaking**: Replaced `FieldVisitor` pattern with static `const FIELDS: &'static [FieldMeta]`
- Field metadata is now available as a compile-time constant slice
- Removed `FieldVisitor` trait and all visitor structs (`CollectMeta`, `CollectNames`, etc.)
- `FieldMeta` no longer contains `type_name` field

### Added

- `FieldMeta::has_tag()` const fn for compile-time tag checking

## [0.1.0] - 2024-12-04

### Added

- `FieldKinds` derive macro for compile-time field introspection
- Automatic type categorization (numeric, text, bool, optional, collection)
- Support for `#[serde(rename)]` and `#[serde(rename_all)]` attributes
- Custom field tags via `#[field_tags(...)]` attribute
- Field skipping via `#[field_kinds(skip)]` attribute
- `FieldVisitor` trait for extensible field processing
- `FieldKindsExt` extension trait with convenience methods:
  - `field_names()` - get original field names
  - `serialized_names()` - get serialized field names
  - `fields_by_category()` - filter fields by type category
  - `fields_by_tag()` - filter fields by custom tag
  - `has_field()` - check field existence
  - `field_category()` - get category for a field
  - `field_meta()` - get full metadata for all fields

[Unreleased]: https://github.com/niktimf/field-kinds/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/niktimf/field-kinds/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/niktimf/field-kinds/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/niktimf/field-kinds/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/niktimf/field-kinds/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/niktimf/field-kinds/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/niktimf/field-kinds/releases/tag/v0.1.0
